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
 *  botinfo.rs */

use std::{env, process};

use crate::{structures::*, ReqwestClient, ShardManagerContainer};
use serenity::{
    client::{bridge::gateway::ShardId, Context},
    framework::standard::CommandResult,
};
use tokio::process::Command;

pub async fn get_last_commit(ctx: &Context) -> Result<CommitResponse, Box<dyn std::error::Error + Send + Sync>> {
    let reqwest_client = ctx.data.read().await.get::<ReqwestClient>().cloned().unwrap();
    let resp = reqwest_client.get("https://api.github.com/repos/thatsimplekid/QuiVive/commits/master")
        .send().await?.json::<CommitResponse>().await?;
    Ok(resp)
}

pub async fn get_system_info(ctx: &Context) -> CommandResult<SysInfo> {
    let shard_manager = ctx.data.read().await.get::<ShardManagerContainer>().cloned().unwrap();
    let shard_latency = {
        let manager = shard_manager.lock().await;
        let runners = manager.runners.lock().await;
        let runner_raw = runners.get(&ShardId(ctx.shard_id));
        match runner_raw {
            Some(runner) => match runner.latency {
                Some(ms) => format!("{}ms", ms.as_millis()),
                None => "?ms".to_string()
            },
            None => "?ms".to_string()
        }
    };
    let pid = process::id();
    let raw_bin_path = env::current_exe()?;
    let bin_path = raw_bin_path.to_string_lossy();
    let bin_str = bin_path.rsplit('/').next().unwrap();
    let mem_stdout = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "pmap {} | grep {} | awk 'NR>1 {{sum+=substr($2, 1, length($2)-1)}} END {{print sum}}'",
            pid, bin_str
        ))
        .output()
        .await
        .expect("Failed to get memory usage");
    let mem_used = String::from_utf8(mem_stdout.stdout).unwrap();
    let memory = &mem_used[..mem_used.len() - 1].parse::<f32>().unwrap() / 1000f32;
    let sys_info = SysInfo {
        shard_latency,
        memory,
    };
    Ok(sys_info)
}