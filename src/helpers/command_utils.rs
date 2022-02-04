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
 *  command_utils.rs */

use crate::structures::{
    cmd_data::{PrefixMap, PubCreds},
    EmojiIdentifier,
};
use regex::Regex;
use serenity::{
    client::Context,
    model::{
        id::{
            ChannelId,
            GuildId,
            MessageId
        },
        channel::Message,
        id::EmojiId,
    }
};

#[allow(clippy::needless_lifetimes)]
pub async fn get_command_name<'a>(ctx: &Context, msg: &'a Message) -> &'a str {
    let (prefixes, default_prefix) = {
        let data = ctx.data.read().await;
        let prefixes = data.get::<PrefixMap>().unwrap().clone();
        let default_prefix = data
            .get::<PubCreds>().unwrap().get("default prefix").cloned().unwrap();
        (prefixes, default_prefix)
    };
    let guild_id = msg.guild_id.unwrap();
    let prefix_length = match prefixes.get(&guild_id) {
        Some(guild_prefix) => guild_prefix.value().len(),
        None => default_prefix.len()
    };
    let words = msg.content.split_whitespace().collect::<Vec<&str>>();
    let command = words[0];
    &command[prefix_length..]
}

pub fn check_mention_prefix(msg: &Message) -> bool {
    let words = msg.content.split_whitespace().collect::<Vec<&str>>();
    let re = Regex::new(r"<@!?\d+>").unwrap();
    re.is_match(words[0])
}

pub fn get_time(initial: u64, param: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let value = match param {
        "s" => initial,
        "m" => initial * 60,
        "h" => initial * 60 * 60,
        "d" => initial * 60 * 60 * 24,
        "w" => initial * 60 * 60 * 24 * 7,
        "y" => initial * 60 * 60 * 24 * 365.25
    };
    Ok(value)
}

pub fn get_allowed_commands() -> Vec<String> {
    let allowed_commands: Vec<String> = vec![
        "prefix".to_owned(),
        "help".to_owned(),
        "restore".to_owned(),
        "resetprefix".to_owned(),
        "restoredb".to_owned(),
    ];
    allowed_commands
}

pub fn get_custom_emoji(emoji: String, name: String, animated: bool) -> String {
    let mut emoji_string = String::new();
    if animated {
        emoji_string.push_str("<a:");
    } else {
        emoji_string.push_str("<:");
    }
    emoji_string.push_str(&format!("{}:{}>", name, emoji));
    emoji_string
}

pub fn get_message_url(guild_id: GuildId, channel_id: ChannelId, message_id: MessageId) -> String {
    format!(
        "https://discordapp.com/channels/{}/{}/{}",
        guild_id.0, channel_id.0, message_id.0
    )
}

pub fn parse_emoji(mention: impl AsRef<str>) -> Option<EmojiIdentifier> {
    let mention = mention.as_ref();
    let len = mention.len();
    if !(6..=56).contains(&len) { return None; }
    if (mention.starts_with("<:") || mention.starts_with("<a:")) && mention.ends_with('>') {
        let mut name = String::default();
        let mut id = String::default();
        let animated = &mention[1..3] == "a:";
        let start = if animated { 3 } else { 2 };
        for (i, x) in mention[start..].chars().enumerate() {
            if x == ':' {
                let from = i + start + 1;
                for y in mention[from..].chars() {
                    if y == '>' {
                        break;
                    } else {
                        id.push(y);
                    }
                }
                break;
            } else {
                name.push(x);
            }
        }
        match id.parse::<u64>() {
            Ok(x) => Some(EmojiIdentifier {
                animated, name, id: EmojiId(x),
            }),
            _ => None,
        }
    } else {
        None
    }
}

