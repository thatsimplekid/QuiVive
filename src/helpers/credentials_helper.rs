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
 *  credentials_helper.rs */

use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::BufReader,
};

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub bot_token: String,
    pub default_prefix: String,
    pub db_connection: String,
}

pub fn read_creds(path: String) -> Result<Credentials, Box<dyn std::error::Error + 'static>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let info: Credentials = serde_json::from_reader(reader).unwrap();
    Ok(info)
}