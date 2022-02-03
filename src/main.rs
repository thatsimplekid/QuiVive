mod helpers;

use dashmap::DashMap;
use reqwest::Client as Reqwest;
use serenity::{client::bridge::gateway::GatewayIntents, http::Http, Client};
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::{atomic::AtomicBool, Arc},
};
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
            owners.insert(info.owners.id);
            (owners, info.id)
        },
        Err(why) => panic!("Failed to access application info: {:?}", why),
    };
    let pool = database_helper::obtain_db_pool(creds.db_connection).await?;
    let prefixes = database_helper::fetch_prefixes(&pool).await?;
    let reqwest_client = Reqwest::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:73.0) Gecko/20100101 Firefox/73.0")
        .build()?;
    let mut pub_creds = HashMap::new();
    pub_creds.insert("default prefix".to_owned(), creds.default_prefix);
    let emergency_commands = command_utils::get_allowed_commands();
    let mut client = Client::builder(&token)
        .framework(get_framework(bot_id, owners).await)
        .event_handler(SerenityHandler {
            run_loop: AtomicBool::new(true),
        })
        .intents({
            GatewayIntents::GUILD_MEMBERS           |
            GatewayIntents::GUILD_PRESENCES         |
            GatewayIntents::GUILD_BANS              |
            GatewayIntents::GUILD_EMOJIS            |
            GatewayIntents::GUILD_MESSAGES          |
            GatewayIntents::GUILD_MESSAGE_REACTIONS
        })
        .cache_settings(|s| s.max_messages(300))
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
    let shard_manager = client.shard_manager.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Failed to register Cc handler");
        shard_manager.lock().await.shutdown_all().await;
    });
    if let Err(why) = client.start_shards(4).await {
        error!("Client error: {:?}", why);
    }
    Ok(())
}