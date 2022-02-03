use serenity::framework::standard::{macros::command, CommandResult, Args};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::fs::*;
use std::io::Write as _;
use rlua::{Function, Lua, MetaMethod, Result, UserData, UserDataMethods, Variadic};

use crate::ShardManagerContainer;

/*#[command]
#[owners_only]
async fn eval(ctx: &Context, msg: &Message) -> CommandResult {
    let lua = Lua::new();
    lua.context(|lua_ctx| {
        lua_ctx.load(msg.content.replace(msg.))
    })?;
}*/

#[command]
#[owners_only]
async fn restart(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    if let Some(manager) = data.get::<ShardManagerContainer>() {
        msg.reply(ctx, "Bye! (for now)").await?;
        manager.lock().await.shutdown_all().await;
    } else {
        msg.reply(ctx, "Failed to get the shard manager :sad:").await?;
        return Ok(());
    }
    Ok(())
}

#[command]
#[owners_only]
async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    if let Some(manager) = data.get::<ShardManagerContainer>() {
        let mut file = File::create("norestart")?;
        file.write_all(b"This file prevents the bot from restarting.")?;
        msg.reply(ctx, "Bye!").await?;
        manager.lock().await.shutdown_all().await;
    } else {
        msg.reply(ctx, "Failed to get the shard manager :sad:").await?;
        return Ok(());
    }
    Ok(())
}