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
use crate::cgrcparser::CGRCParser;

pub struct CGRCConfManager {}

impl CGRCConfManager {
    pub fn default_system_path() -> &'static str {
        "/etc/cgrc"
    }

    pub fn default_user_path() -> Option<String> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "luke", "cgrc") {
            if let Some(path) = proj_dirs.config_dir().to_str() {
                return Some(path.to_string());
            }
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
                        log::error!("Error: {}. Cannot read conf {}", e, conf);
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
                        log::error!("Error: {}. Cannot read conf {}", e, conf);
                        return None;
                    }
                }
            }

            if let Some(user_path) = Self::default_user_path() {
                let proposed_path = PathBuf::from(user_path)
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

    ///
    /// Prints the available system confs to stdout.
    /// 
    pub fn print_avail_confs() {
        println!("Embedded configurations:");
        Self::print_avail_embedded_confs();

        println!("");
        println!("User configurations:");
        Self::print_avail_user_confs();

        println!("");
        println!("System configurations:");
        Self::print_avail_system_confs();
    }

    ///
    /// Prints embedded configurations to stdout.
    /// 
    pub fn print_avail_embedded_confs() {
        let confs = load_confs();
        for (key, content) in confs {
            let conf_string = match str::from_utf8(content) {
                Err(_) => {
                    log::warn!("Failed to parse conf file");
                    continue;
                },
                Ok(s) => s
            };
            let conf = CGRCParser::parse_conf_string(conf_string.to_string());
            println!("\t{key} -> {}", match conf.description {
                None => String::from("?"),
                Some(v) => v
            });
        }
    }

    ///
    /// Prints user configurations to stdout.
    /// 
    pub fn print_avail_user_confs() {
        if let Some(v) = Self::default_user_path() {
            Self::print_avail_confs_in_path(&v.to_string());
        }
    }

    ///
    /// Prints system-wide configurations to stdout.
    /// 
    pub fn print_avail_system_confs() {
        Self::print_avail_confs_in_path(&Self::default_system_path().to_string());
    }

    ///
    /// Print available configurations in the path.
    /// 
    fn print_avail_confs_in_path(path: &String) {
        let read_dir = match fs::read_dir(path) {
            Err(_) => return,
            Ok(v) => v
        };
        for file in read_dir {
            let item = match file {
                Err(e) => {
                    log::warn!("Failed to list file: {}", e.to_string());
                    continue;
                },
                Ok(f) => f
            };
            let path = PathBuf::from(path).join(item.file_name());
            let path_string = match path.to_str() {
                None => continue,
                Some(v) => v
            };
            let conf = CGRCParser::parse_conf(&path_string.to_string());
            println!("\t{} -> {}", path_string, match conf.description {
                None => String::from("?"),
                Some(v) => v
            });
        }
    }
}
