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
 *  errors.rs */

use std::fmt;

#[derive(Debug)]
pub enum QVError<'a> {
    PermissionError(PermissionType<'a>),
    InvalidArgumentError(&'a str),
    InvalidTargetError(&'a str),
    FailError(&'a str),
    InternalError(&'a str),
}

impl fmt::Display for QVError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QVError::PermissionError(perm) => write!(f, "{}", perm),
            QVError::InvalidArgumentError(arg) => write!(f, "Invalid argument: {}!", arg),
            QVError::InvalidTargetError(cmd) => write!(f, "Running {} against yourself is not permitted.", cmd),
            QVError::FailError(cmd) => write!(f, "{} failed. Make sure you've followed the setup guide.", cmd),
            QVError::InternalError(message) => write!(f, "{}", message),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PermissionType<'b> {
    SelfPerm(&'b str),
    Mention(&'b str, &'b str),
}

impl fmt::Display for PermissionType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PermissionType::SelfPerm(perm) => write!(f, "You can't execute this command - you're not a {} on this server", perm),
            PermissionType::Mention(cmd, perm) => write!(f, "I can't {} a(n) {}! Please demote the user first, or enable auto demotion", cmd, perm)
        }
    }
}

