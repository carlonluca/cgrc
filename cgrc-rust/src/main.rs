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

pub mod cgrcconfmanager;
pub mod cgrcconfstorage;
pub mod cgrcdata;
pub mod cgrcparser;
pub mod cgrccli;

use std::io::{BufRead, stdin};
use cgrcparser::CGRCParser;
use cgrcconfmanager::CGRCConfManager;
use cgrccli::Cli;
use clap::Parser;

fn main() {
    unsafe {
        libc::signal(libc::SIGINT, libc::SIG_IGN);
    }

    env_logger::init();

    let args = Cli::parse();
    if args.list_locations {
        println!("Locations on your system used by cgrc:");
        println!("\tSystem location: {}", CGRCConfManager::default_system_path());
        if let Some(user_path) = CGRCConfManager::default_user_path() {
            println!("\tUser location  : {}", user_path);
        }

        return;
    }

    if args.location_user {
        if let Some(user_path) = CGRCConfManager::default_user_path() {
            println!("{}", user_path);
        }
        
        return;
    }

    if args.location_system {
        println!("{}", CGRCConfManager::default_system_path());
        return;
    }

    if args.list_configurations {
        CGRCConfManager::print_avail_confs();
        return;
    }

    let args_conf = match args.conf {
        None => {
            println!("Missing argument");
            return;
        },
        Some(conf) => conf
    };

    let is_local_path = args.conf_path;
    let conf_data = CGRCConfManager::load_conf(&args_conf, is_local_path);
    if None == conf_data {
        println!("Failed to find conf file: {0}", args_conf);
        return;
    }

    let conf = CGRCParser::parse_conf_string(conf_data.unwrap());
    if args.debug {
        log::debug!("Conf file includes {} items and description is {:?}",
            conf.items.len(),
            conf.description
        );
    }

    let conf_items = &conf.items;
    let stdin = stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(v) => v,
            Err(e) => {
                println!("Error: {}", e.to_string());
                continue
            }
        };
        let formatted = CGRCParser::parse_log_line(
            &conf_items,
            &line,
            args.debug
        );
        
        if let Some(formatted) = formatted {
            println!("{}", formatted);
        }
    }
}
