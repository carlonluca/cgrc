/*
 * This file is part of cgrc.
 *
 * Copyright (c) 2023 Luca Carlon
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

 use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(long = "list-locations")]
    pub list_locations: bool,
    #[arg(long = "location-user")]
    pub location_user: bool,
    #[arg(long = "location-system")]
    pub location_system: bool,
    #[arg(long = "list-configurations")]
    pub list_configurations: bool,
    #[arg(long = "conf-path")]
    pub conf_path: bool,
    #[arg(long = "debug")]
    pub debug: bool,
    pub conf: Option<String>
}
