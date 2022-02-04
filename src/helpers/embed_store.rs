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
 *  embed_store.rs */

use serenity::{
    builder::CreateEmbed,
    model::misc::Mentionable,
    model::{
        id::{ChannelId, RoleId},
        prelude::User,
    },
    utils::Colour,
};

static COLOR_RED: Color = 0xef5f5f;
static COLOR_GREEN: Color = 0x5fefa7;
static COLOR_ORANGE: Color = 0xefa75f;
static COLOR_PURPLE: Color = 0xc76ae6;

pub fn get_ban_embed(user: &User, reason: &str, use_id: bool) -> CreateEmbed {
    let mut eb = CreateEmbed::default();
    eb.color(COLOR_RED);
    eb.title(if use_id { "User Banned by ID" } else { "User Banned" });
    eb.thumbnail(user.face());
    eb.field("Username", user.mention(), false);
    eb.field("Reason", reason, false);
    eb
}

pub fn get_unban_embed(user: &User, use_id: bool) -> CreateEmbed {
    let mut eb = CreateEmbed::default();
    eb.color(COLOR_GREEN);
    eb.title(if use_id { "User Unbanned by ID" } else { "User Unbanned" });
    eb.thumbnail(user.face());
    eb.field("Username", user.mention(), false);
    eb
}

pub fn get_kick_embed(user: &User, reason: &str, use_id: bool) -> CreateEmbed {
    let mut eb = CreateEmbed::default();
    eb.color(COLOR_RED);
    eb.title(if use_id { "User Kicked by ID" } else { "User Kicked" });
    eb.thumbnail(user.face());
    eb.field("Username", user.mention(), false);
    eb.field("Reason", reason, false);
    eb
}

pub fn get_warn_embed(user: &User, warn_count: i32, reason: &str) -> CreateEmbed {
    let mut eb = CreateEmbed::default();
    eb.color(COLOR_ORANGE);
    eb.title("User Warned");
    eb.thumbnail(user.face());
    eb.field("Username", user.mention(), false);
    eb.field("Warn Count", warn_count, false);
    eb.field("Reason", reason, false);
    eb
}

pub fn get_pardon_embed(user: &User, warn_count: i32) -> CreateEmbed {
    let mut eb = CreateEmbed::default();
    eb.color(COLOR_GREEN);
    eb.title("User Pardoned");
    eb.thumbnail(user.face());
    eb.field("Username", user.mention(), false);
    eb.field("Warn Count", warn_count, false);
    eb
}

pub fn get_guild_warns_embed(guild_name: String, warns_string: String) -> CreateEmbed {
    let mut eb = CreateEmbed::default();
    eb.color(COLOR_PURPLE);
    eb.title(format!("Warns for {}", guild_name));
    eb.description(warns_string);
    eb.footer(|f| f.text("If a mention has an invalid user, consider removing the warns!"));
    eb
}

pub fn get_mute_embed(user: &User, new_mute: bool, use_time: bool, mute_time: Option<&str>) -> CreateEmbed {
    let mut eb = CreateEmbed::default();
    eb.color(if new_mute { COLOR_ORANGE } else { COLOR_GREEN });
    eb.title(if new_mute { "User Muted" } else { "User Unmuted" });
    eb.thumbnail(user.face());
    eb.description(if use_time {
        "This mute will expire after the given time"
    } else {
        "This mute can only be removed by an admin"
    });
    eb.field("Username", user.mention(), false);
    if use_time { eb.field("Mute Length", mute_time.unwrap(), false) };
    eb
}

pub fn get_guild_mutes_embed(guild_name: String, permanent_mute_string: String, timed_mute_string: String) -> CreateEmbed {
    let mut eb = CreateEmbed::default();
    eb.color(COLOR_PURPLE);
    eb.title(format!("Mutes for {}", guild_name));
    eb.description("All times are in UTC");
    eb.field("Permanent mutes", permanent_mute_string, false);
    eb.field("Timed mutes", timed_mute_string, false);
    eb.footer(|f| f.text("Please use ginfo for config info"));
    eb
}

pub fn get_channel_embed(channel_id: ChannelId, channel_type: &str) -> CreateEmbed {
    let mut eb = CreateEmbed::default();
    eb.color(COLOR_PURPLE);
    eb.title(format!("New {} Channel", channel_type));
    eb.description(format!("New channel: {}", channel_id.mention()));
    eb
}

pub fn get_new_member_embed(msg: String, channel_id: ChannelId, message_type: &str) -> CreateEmbed {
    let mut eb = CreateEmbed::default();
    eb.color(COLOR_PURPLE);
    eb.title(format!("{} information", message_type));
    eb.description(format!(
        "Current welcome/leave channel: {}", channel_id.mention()
    ));
    eb.field("Message", format!("```{} \n```", msg), false);
    eb
}

pub fn get_welcome_roles_embed(role_ids: Vec<RoleId>) -> CreateEmbed {
    let mut eb = CreateEmbed::default();
    eb.color(COLOR_PURPLE);
    eb.description("The following roles will be assigned to new joiners");
    for role_id in role_ids {
        eb.field(role_id.mention(), "", false);
    }
    eb
}



