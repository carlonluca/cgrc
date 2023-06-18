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

use std::io::{BufRead, Cursor, BufReader, stdin};
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
        log::info!("Locations on your system used by cgrc:");
        log::info!("\tSystem location: {}", CGRCConfManager::default_system_path());
        if let Some(user_path) = CGRCConfManager::default_user_path() {
            if let Some(user_path_string) = user_path.to_str() {
                log::info!("\tUser location  : {}", user_path_string);
            }
        }

        return;
    }

    if args.location_user {
        if let Some(user_path) = CGRCConfManager::default_user_path() {
            if let Some(user_path_string) = user_path.to_str() {
                log::info!("{}", user_path_string);
            }
        }
        
        return;
    }

    if args.location_system {
        log::info!("{}", CGRCConfManager::default_system_path());
        return;
    }

    if args.list_configurations {
        CGRCConfManager::print_avail_confs();
        return;
    }

    let is_local_path = args.conf_path;
    let conf_data = CGRCConfManager::load_conf(&args.conf, is_local_path);
    if None == conf_data {
        log::error!("Failed to find conf file: {0}", args.conf);
        return;
    }

    let cursor = Cursor::new(conf_data.unwrap());
    let reader = BufReader::new(cursor);
    let conf = CGRCParser::parse_conf_lines(reader);
    let conf_items = &conf.items;
    let stdin = stdin();
    for line in stdin.lock().lines() {
        // TODO: handle errors here.
        let line = line.unwrap();
        let formatted = CGRCParser::parse_log_line(
            &conf_items,
            &line,
            args.debug
        );
        println!("{}", line);
        if let Some(formatted) = formatted {
            println!("{}", formatted);
        }
    }
}
