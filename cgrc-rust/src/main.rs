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

pub mod cgrcconf;
pub mod cgrcconfstorage;
pub mod cgrc_data;

use clap::Parser;

use crate::cgrcconf::CGRCConf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long = "list-locations")]
    pub list_locations: bool,
    #[arg(long = "location-user")]
    pub location_user: bool,
    #[arg(long = "location-system")]
    pub location_system: bool,
    #[arg(long = "list-configurations")]
    pub list_configurations: bool
}

fn main() {
    unsafe {
        libc::signal(libc::SIGINT, libc::SIG_IGN);
    }

    env_logger::init();

    let args = Cli::parse();
    if args.list_locations {
        log::info!("Locations on your system used by cgrc:");
        log::info!("\tSystem location: {}", CGRCConf::default_system_path());
        if let Some(user_path) = CGRCConf::default_user_path() {
            if let Some(user_path_string) = user_path.to_str() {
                log::info!("\tUser location  : {}", user_path_string);
            }
        }

        return;
    }

    if args.location_user {
        if let Some(user_path) = CGRCConf::default_user_path() {
            if let Some(user_path_string) = user_path.to_str() {
                log::info!("{}", user_path_string);
            }
        }
        
        return;
    }

    if args.location_system {
        log::info!("{}", CGRCConf::default_system_path());
        return;
    }

    if args.list_configurations {
        CGRCConf::print_avail_confs();
        return;
    }
}
