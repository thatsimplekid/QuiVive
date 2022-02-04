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
 *  database_helper.rs */

use dashmap::DashMap;
use serenity::{
    framework::standard::CommandResult,
    model::id::GuildId
};
use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn obtain_db_pool(db_connection: String) -> CommandResult<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_connection)
        .await?;
    Ok(pool)
}

pub async fn fetch_prefixes(pool: &PgPool) -> CommandResult<DashMap<GuildId, String>> {
    let prefixes: DashMap<GuildId, String> = DashMap::new();
    let cursor = sqlx::query!("SELECT guild_id, prefix FROM guild_info")
        .fetch_all(pool).await?;
    for i in cursor {
        if let Some(prefix) = i.prefix {
            prefixes.insert(GuildId::from(i.guild_id as u64), prefix);
        }
    }
    Ok(prefixes)
}
