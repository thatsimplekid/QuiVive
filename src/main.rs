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
 *  main.rs */

mod commands;
mod handlers;
mod helpers;
//mod reactions;
mod structures;

use dashmap::DashMap;
use helpers::{command_utils, database_helper};
use reqwest::Client as Reqwest;
use serenity::{client::bridge::gateway::GatewayIntents, http::Http, Client};
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::{atomic::AtomicBool, Arc}
};
use handlers::{event_handler::SerenityHandler, framework::get_framework};
use structures::{cmd_data::*, errors::*};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt::init();
    let args: Vec<String> = env::args().collect();
    let creds = helpers::credentials_helper::read_creds(args[1].to_owned()).unwrap();
    let token = &creds.bot_token;
    let http = Http::new_with_token(token);
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };
    let pool = database_helper::obtain_db_pool(creds.db_connection).await?;
    let prefixes = database_helper::fetch_prefixes(&pool).await?;
    let reqwest_client = Reqwest::builder()
        .user_agent("Mozilla/5.0 (compatible; QuiVive/1.0.3; +http://qv.herty.xyz/bots").build()?;
    let mut pub_creds = HashMap::new();
    pub_creds.insert("default prefix".to_owned(), creds.default_prefix);
    let emergency_commands = command_utils::get_allowed_commands();
    let mut client = Client::builder(&token)
        .framework(get_framework(bot_id, owners).await)
        .event_handler(SerenityHandler {
            run_loop: AtomicBool::new(true),
        })
        .intents({
            GatewayIntents::GUILDS |
            GatewayIntents::GUILD_MEMBERS |
            GatewayIntents::GUILD_PRESENCES |
            GatewayIntents::GUILD_BANS |
            GatewayIntents::GUILD_EMOJIS |
            GatewayIntents::GUILD_MESSAGES |
            GatewayIntents::GUILD_MESSAGE_REACTIONS
        })
        .cache_settings(|settings| settings.max_messages(300))
        .await
        .expect("Error creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<PubCreds>(Arc::new(pub_creds));
        data.insert::<ConnectionPool>(pool);
        data.insert::<MuteMap>(Arc::new(DashMap::new()));
        data.insert::<PrefixMap>(Arc::new(prefixes));
        data.insert::<BotId>(bot_id);
        data.insert::<ReqwestClient>(Arc::new(reqwest_client));
        data.insert::<EmergencyCommands>(Arc::new(emergency_commands));
    }
    if let Err(why) = client.start_shards(4).await {
        error!("Client error: {:?}", why);
    }
    Ok(())
}