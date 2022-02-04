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
 *  commands.rs */

// use crate::commands::{};

use serenity::framework::standard::macros::group;

#[group("General")]
#[help_available(false)]
#[commands()]
pub struct General;

#[group("Configuration")]
#[description = "Admin/Mod commands to configure the bot's behavior"]
#[commands()]
pub struct Config;

#[group("Moderation")]
#[description = "Moderation commands"]
#[commands()]
pub struct Moderation;

#[group("Logging")]
#[description = "Commands for logging activity"]
#[commands()]
pub struct Logging;

#[group("Information")]
#[description = "Generic information commands"]
#[commands()]
pub struct Info;

#[group("Welcome")]
#[description = "Commands for new member welcoming"]
#[commands()]
pub struct Welcome;

#[group("Support")]
#[description = "Support and help commands"]
#[commands()]
pub struct Support;
