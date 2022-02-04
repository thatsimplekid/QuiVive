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
 *  delete_buffer.rs */

use crate::{
    structures::cmd_data::PrefixMap,
    ConnectionPool
};
use serenity::{
    framework::standard::CommandResult,
    model::prelude::*,
    prelude::*
};
use sqlx::PgPool;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;
use tracing::{error, info};

pub async fn mark_for_deletion(pool: &PgPool, guild_id: GuildId) -> CommandResult {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time can't go backwards.")
        .as_secs();
    sqlx::query!(
        "INSERT INTO delete_time_store VALUES($1, $2)",
        guild_id.0 as i64,
        current_time as i64 + 604800,
    ).execute(pool).await?;
    Ok(())
}

pub async fn add_new_guild(pool: &PgPool, guild_id: GuildId, is_new: bool) -> CommandResult {
    let delete_data = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM delete_time_store WHERE guild_id = $1)", guild_id.0 as i64
    ).fetch_one(pool).await?;
    if delete_data.exists.unwrap() {
        sqlx::query!(
            "DELETE FROM delete_time_store WHERE guild_id = $1", guild_id.0 as i64
        ).execute(pool).await?;
    } else if is_new {
        sqlx::query!(
            "INSERT INTO guild_info VALUES($1, null, null, null) ON CONFLICT DO NOTHING", guild_id.0 as i64
        ).execute(pool).await?;
    }
    Ok(())
}

pub async fn guild_removal_loop(ctx: Context) -> CommandResult {
    let (pool, prefixes) = {
        let data = ctx.data.read().await;
        let pool = data.get::<ConnectionPool>().cloned().unwrap();
        let prefixes = data.get::<PrefixMap>().cloned().unwrap();
        (pool, prefixes)
    };
    loop {
        let delete_data = sqlx::query!("SELECT guild_id, delete_time FROM delete_time_store")
            .fetch_all(&pool).await?;
        for i in delete_data {
            info!("Checking deletion state on guild {}", i.guild_id);
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time can't go backwards.")
                .as_secs() as i64;
            if i.delete_time <= current_time {
                info!("Deleting guild {} from the database", i.guild_id);
                sqlx::query!("DELETE FROM guild_info WHERE guild_id = $1", i.guild_id).execute(&pool).await?;
                let guild_id = GuildId::from(i.guild_id as u64);
                if prefixes.contains_key(&guild_id) { prefixes.remove(&guild_id) };

            } else {
                error!("Entry is within retention. Keeping data for guild {}", i.guild_id);
            }
        }
        sleep(Duration::from_secs(345600)).await;
    }
}

pub async fn delete_leftover_reactions(pool: &PgPool, message_id: MessageId) -> CommandResult {
    let check = sqlx::query!("SELECT EXISTS(SELECT 1 FROM reaction_roles WHERE message_id = $1)", message_id.0 as i64)
        .fetch_one(pool).await?;
    if check.exists.unwrap() {
        sqlx::query!("DELETE FROM reaction_roles WHERE message_id = $1", message_id.0 as i64)
            .execute(pool).await?;
    }
    Ok(())
}

pub async fn guild_pruner(ctx: &Context) -> CommandResult {
    let pool = ctx.data.read().await.get::<ConnectionPool>().cloned().unwrap();
    let guilds = ctx.cache.guilds().await;
    let guild_data = sqlx::query!("SELECT guild_id FROM guild_info").fetch_all(&pool).await?;
    for guild in guild_data {
        if !guilds.contains(&GuildId::from(guild.guild_id as u64)) {
            let delete_check = sqlx::query!(
                "SELECT EXISTS(SELECT 1 FROM delete_time_store WHERE guild_id = $1)", guild.guild_id
            ).fetch_one(&pool).await?;
            if !delete_check.exists.unwrap() {
                info!("Removing guild: {}", guild.guild_id);
                sqlx::query!("DELETE FROM guild_info WHERE guild_id = $1", guild.guild_id)
                    .execute(&pool).await?;
            }
        }
    }
    Ok(())
}

























