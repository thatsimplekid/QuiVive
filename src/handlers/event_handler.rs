/*  Qui Vive - A Discord bot to keep out the riff-raff
 *  Copyright (C) 2022 Owen Flaherty
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Affero General Public License as published
 *  by the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Affero General Public License for more details.
 *
 *  You should have received a copy of the GNU Affero General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 *  event_handler.rs */

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use serde_json::Value;
use crate::{
    helpers::{delete_buffer, mute_helper::load_mute_timers},
    reactions::reaction_roles,
    structures::cmd_data::BotId,
    ConnectionPool
};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::{GuildChannel, Message, Reaction},
        event::MessageUpdateEvent,
        guild::{Guild, GuildUnavailable, Member},
        id::{ChannelId, GuildId, MessageId, RoleId},
        prelude::{Activity, Mentionable, Ready, User}
    }
};
use serenity::client::bridge::gateway::event::ShardStageUpdateEvent;
use serenity::model::channel::{Channel, ChannelCategory, PartialGuildChannel, StageInstance};
use serenity::model::event::{ChannelPinsUpdateEvent, GuildMembersChunkEvent, GuildMemberUpdateEvent, InviteCreateEvent, InviteDeleteEvent, PresenceUpdateEvent, ResumedEvent, ThreadListSyncEvent, ThreadMembersUpdateEvent, TypingStartEvent, VoiceServerUpdateEvent};
use serenity::model::gateway::Presence;
use serenity::model::guild::{Emoji, Integration, PartialGuild, Role, ThreadMember};
use serenity::model::id::{ApplicationId, EmojiId, IntegrationId};
use serenity::model::prelude::{CurrentUser, VoiceState};
use tracing::{info, error};

pub struct SerenityHandler {
    pub run_loop: AtomicBool,
}

#[async_trait]
impl EventHandler for SerenityHandler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        if self.run_loop.load(Ordering::Relaxed) {
            self.run_loop.store(false, Ordering::Relaxed);
            info!("Running guild pruner");
            if let Err(why) = delete_buffer::guild_pruner(&ctx).await {
                panic!("Error pruning guilds: {}", why);
            }
            info!("Initializing mute timers");
            if let Err(why) = load_mute_timers(&ctx).await {
                panic!("Error initializing mute timers: {}", why);
            }
            info!("Starting guild deletion loop");
            let ctx_clone = ctx.clone();
            tokio::spawn(async move {
                if let Err(why) = delete_buffer::guild_removal_loop(ctx_clone).await {
                    panic!("Guild deletion loop failed to start: {}", why);
                }
            });
            info!("Setting presence");
            ctx.shard.set_activity(Some(Activity::watching("you >:)")));
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn guild_member_addition(&self, _ctx: Context, guild_id: GuildId, mut new_member: Member) {
        if new_member.user.bot { return; }
        let pool = ctx.data.read().await.get::<ConnectionPool>().cloned().unwrap();
        let welcome_data = match sqlx::query!(
            "SELECT welcome_message, channel_id FROM new_members WHERE guild_id = $1", guild_id.0 as i64
        ).fetch_optional(&pool).await {
            Ok(data) => data,
            Err(why) => {
                error!("Error fetching welcome data: {}", why);
                return;
            }
        };
        if let Some(welcome_data) = welcome_data {
            if let Some(message) = welcome_data.welcome_message {
                let channel_id = ChannelId::from(welcome_data.channel_id as u64);
                let welcome_message = message
                    .replace("{user}", &new_member.user.mention().to_string())
                    .replace("{server}", &guild_id.name(&ctx).await.unwrap());
                let _ = channel_id.say(&ctx, welcome_message).await;
            }
        }
        let role_check = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM welcome_roles WHERE guild_id = $1)",
            guild_id.0 as i64
        ).fetch_one(&pool).await.unwrap();
        if role_check.exists.unwrap() {
            let welcome_roles = sqlx::query!(
                "SELECT role_id FROM welcome_roles WHERE guild_id = $1", guild_id.0 as i64
            ).fetch_all(&pool).await.unwrap();
            for i in welcome_roles {
                if new_member
                    .add_role(&ctx, RoleId::from(i.role_id as u64))
                    .await
                    .is_err() {
                    sqlx::query!(
                        "DELETE FROM welcome_roles WHERE guild_id = $1 AND role_id = $2",
                        guild_id.0 as i64, i.role_id
                    ).execute(&pool).await.unwrap();
                }
            }
        }
    }

    async fn guild_member_removal(&self, ctx: Context, guild_id: GuildId, user: User, _member_data_if_available: Option<Member>) {
        let pool = ctx.data.read().await.get::<ConnectionPool>().cloned().unwrap();
        if user.bot { return; }
        let leave_data = match sqlx::query!(
            "SELECT leave_message, channel_id FROM new_members WHERE guild_id = $1",
            guild_id.0 as i64
        ).fetch_optional(&pool).await {
            Ok(data) => data,
            Err(why) => {
                error!("Error fetching welcome data: {}", why);
                return;
            }
        };
        if let Some(leave_data) = leave_data {
            if let Some(message) = leave_data.leave_message {
                let channel_id = ChannelId::from(leave_data.channel_id as u64);
                let leave_message = message
                    .replace("{user}", &format!("**{}#{}**", user.name, user.discriminator))
                    .replace("{server}", *guild_id.name(&ctx).await.unwrap());
                let _ = channel_id.say(&ctx, leave_message).await;
            }
        }
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {
        let pool = ctx.data.read().await.get::<ConnectionPool>().cloned().unwrap();
        if let Err(why) = delete_buffer::add_new_guild(&pool, guild.id, is_new).await {
            error!("Error creating guild (ID {}): {}", guild.id.0, why)
        }
    }

    async fn guild_delete(&self, ctx: Context, incomplete: GuildUnavailable, _full: Option<Guild>) {
        let pool = ctx.data.read().await.get::<ConnectionPool>().cloned().unwrap();
        if let Err(why) = delete_buffer::mark_for_deletion(&pool, incomplete.id).await {
            error!("Error marking guild for deletion (ID {}): {}", incomplete.id.0, why);
        }
    }

    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        if let Err(why) = reaction_roles::dispatch_event(&ctx, &add_reaction, false).await {
            error!("Error in reaction dispatch (ID {}): {}", add_reaction.guild_id.unwrap().0, why);
            let _ = add_reaction.channel_id.say(ctx, concat!(
            "Looks like there was an error when you reacted! \n",
            "Please make sure you have the `Add Reactions` permission enabled for both the channel and bot role"
            )).await;
        }
    }

    async fn reaction_remove(&self, ctx: Context, removed_reaction: Reaction) {
        if let Err(why) = reaction_roles::dispatch_event(&ctx, &removed_reaction, true).await {
            error!("Error in reaction dispatch (ID {}): {}", removed_reaction.guild_id.unwrap().0, why);
            let _ = removed_reaction.channel_id.say(ctx, concat!(
            "Looks like there was an error when you reacted! \n",
            "Please make sure you have the `Add Reactions` permission enabled for both the channel and bot role"
            )).await;
        }
    }

    async fn reaction_remove_all(&self, ctx: Context, channel_id: ChannelId, message_id: MessageId) {
        let pool = ctx.data.read().await.get::<ConnectionPool>().cloned().unwrap();
        if let Err(why) = delete_buffer::delete_leftover_reactions(&pool, message_id).await {
            info!("Error deleting reactions in message delete: {}", why);
        }
    }

}