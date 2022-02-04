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
 *  cmd_data.rs */

use dashmap::DashMap;
use futures::future::AbortHandle;
use reqwest::Client as Reqwest;
use serenity::{
    client::bridge::gateway::ShardManager,
    model::id::{GuildId, UserId},
    prelude::{Mutex, TypeMapKey},
};
use sqlx::PgPool;
use std::{collections::HashMap, sync::Arc};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct PubCreds;

impl TypeMapKey for PubCreds {
    type Value = Arc<HashMap<String, String>>;
}

pub struct ConnectionPool;

impl TypeMapKey for ConnectionPool {
    type Value = PgPool;
}

pub struct MuteMap;

impl TypeMapKey for MuteMap {
    type Value = Arc<DashMap<(GuildId, UserId), AbortHandle>>;
}

pub struct PrefixMap;

impl TypeMapKey for PrefixMap {
    type Value = Arc<DashMap<GuildId, String>>;
}

pub struct BotId;

impl TypeMapKey for BotId {
    type Value = UserId;
}

pub struct ReqwestClient;

impl TypeMapKey for ReqwestClient {
    type Value = Arc<Reqwest>;
}

pub struct EmergencyCommands;

impl TypeMapKey for EmergencyCommands {
    type Value = Arc<Vec<String>>;
}