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

use directories::ProjectDirs;
use std::fs;
use std::str;
use std::path::{Path, PathBuf};
use crate::cgrcconfstorage::load_confs;

pub struct CGRCConf {}

impl CGRCConf {
    pub fn default_system_path() -> &'static str {
        "/etc/cgrc"
    }

    pub fn default_user_path() -> Option<PathBuf> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "luke", "cgrc") {
            return Some(proj_dirs.config_dir().to_path_buf());
        }

        return None;
    }

    pub fn load_conf(conf: &String, local_path: bool) -> Option<String> {
        if local_path {
            let path = Path::new(conf);
            return if path.exists() {
                match fs::read_to_string(path) {
                    Ok(c) => return Some(c),
                    Err(e) => {
                        log::error!("Cannot read conf {}", conf);
                        return None;
                    }
                }
            }
            else {
                None
            };
        }
        else {
            let confs = load_confs();
            if confs.contains_key(conf.as_str()) {
                match str::from_utf8(confs.get(conf.as_str()).unwrap()) {
                    Ok(c) => return Some(c.to_string()),
                    Err(e) => {
                        log::error!("Cannot read conf {}", conf);
                        return None;
                    }
                }
            }

            if let Some(user_path) = Self::default_user_path() {
                let proposed_path = Path::new(user_path.as_path())
                    .join(conf);
                if proposed_path.as_path().exists() {
                    if let Some(proposed_path_string) = proposed_path.as_path().to_str() {
                        return Some(proposed_path_string.to_string());
                    }
                }
            }

            let proposed_path = Path::new(Self::default_system_path())
                .join(conf);
            if proposed_path.as_path().exists() {
                if let Some(proposed_path_string) = proposed_path.as_path().to_str() {
                    return Some(proposed_path_string.to_string());
                }
            }
            
            None
        }
    }

    pub fn print_avail_confs() {
        log::info!("Embedded configurations:");
        Self::print_avail_embedded_confs();
    }

    pub fn print_avail_embedded_confs() {
        let confs = load_confs();
        for (key, value) in confs {
            log::info!("\t{key}");
        }
    }
}
