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
 *  warn_helper.rs */

use itertools::Itertools;
use serenity::{
    framework::standard::CommandResult,
    model::id::{GuildId, UserId},
    prelude::Mentionable
};
use sqlx::PgPool;

pub async fn fetch_warn_number(pool: &PgPool, guild_id: GuildId, warn_user_id: UserId) -> CommandResult<Option<i32>> {
    let guild_id = guild_id.0 as i64;
    let warn_user_id = warn_user_id.0 as i64;
    let warn_data = sqlx::query!(
        "SELECT warn_number FROM warns WHERE guild_id = $1 AND user_id = $2", guild_id, warn_user_id
    ).fetch_optional(pool).await?;
    let warn_number = warn_data.map(|data| data.warn_number);
    Ok(warn_number)
}

pub async fn fetch_guild_warns(pool: &PgPool, guild_id: GuildId) -> CommandResult<Option<String>> {
    let guild_id = guild_id.0 as i64;
    let warn_data_vec = sqlx::query!("SELECT * FROM warns WHERE guild_id = $1", guild_id)
        .fetch_all(pool).await?;
    if warn_data_vec.is_empty() { return Ok(None) };
    let guild_warns_string = warn_data_vec.iter()
        .format_with(" \n", |warn_data, f| {
            let user_id = UserId::from(warn_data.user_id as u64);
            let warn_number = warn_data.warn_number;
            f(&format_args!("{}: {}", user_id.mention(), &warn_number))
        }).to_string();
    Ok(Some(guild_warns_string))
}

pub async fn update_warn(pool: &PgPool, guild_id: GuildId, warn_user_id: UserId, warn_number: i32) -> CommandResult {
    let guild_id = guild_id.0 as i64;
    let warn_user_id = warn_user_id.0 as i64;
    sqlx::query!(
        "INSERT INTO warns(guild_id, user_id, warn_number)\
            VALUES($1, $2, $3)\
            ON CONFLICT (guild_id, user_id)\
            DO UPDATE\
            SET warn_number = EXCLUDED.warn_number",
        guild_id,
        warn_user_id,
        warn_number
    ).execute(pool).await?;
    Ok(())
}