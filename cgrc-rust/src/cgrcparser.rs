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

use std::{process, fs::File, io::{BufReader, BufRead}, collections::HashSet};
use core::ptr::addr_of;
use libc::IN_CLOEXEC;
use regex::Regex;
use crate::cgrcdata::{
    CGRCColorItem,
    CGRP_CountMode,
    CGRCConfItem,
    CGRCConf,
    CGRC_Attrib,
    LC_LogColor,
    LC_BackColor,
    COLORS_ATTRS,
    COLORS_FORG,
    COLORS_BACK
};

pub struct CGRCParser {}

impl CGRCParser {
    ///
    /// Parsers a conf file.
    /// 
    pub fn parse_conf(conf_file: &String) -> CGRCConf {
        let file = match File::open(conf_file) {
            Err(e) => {
                log::error!("Failed to open conf file {conf_file}: {e}");
                process::exit(1);
            },
            Ok(f) => {
                f
            }
        };

        let reader = BufReader::new(file);
        return Self::parse_conf_lines(reader);
    }

    ///
    /// Parse lines from a buffered reader.
    /// 
    pub fn parse_conf_lines<T: BufRead>(reader: T) -> CGRCConf {
        let mut item = CGRCConfItem::new();
        let mut conf = CGRCConf {
            description: None,
            items: vec![]
        };
        for line in reader.lines() {
            match line {
                Err(e) => {
                    log::error!("Failed to read conf file: {e}");
                    process::exit(1);
                },
                Ok(line) => {
                    if CGRCParser::parse_conf_line(&line, &mut conf,  &mut item) {
                        conf.items.push(item.clone());
                    }
                }
            }
        }

        conf
    }

    // Private portion
    // ===============
    fn parse_conf_line(line: &String, conf: &mut CGRCConf, item: &mut CGRCConfItem) -> bool {
        let lline = line.to_lowercase();

        if lline.starts_with("desc=") {
            conf.description = Some(lline.replace("desc=", ""));
            return false;
        }

        if lline.starts_with("regexp=") {
            item.regex = match Regex::new(lline.replace("regexp=", "").as_str()) {
                Err(e) => {
                    log::error!("Failed to parse regex: {line}");
                    log::error!("{}", e.to_string());
                    process::exit(1);
                },
                Ok(r) => Some(r)
            };
            return false;
        }

        if lline.starts_with("colours=") {
            item.colors.append(&mut CGRCParser::parse_colors(&lline));
            return false;
        }

        if lline.starts_with("skip=") {
            item.skip = Some(lline.to_lowercase() == "skip=yes");
            return false;
        }

        if lline.starts_with("count=") {
            match lline.as_str() {
                "count=once" => {
                    item.count_mode = Some(CGRP_CountMode::CGRC_COUNT_ONCE);
                },
                "count=more" => {
                    item.count_mode = Some(CGRP_CountMode::CGRC_COUNT_MORE);
                },
                "count=stop" => {
                    item.count_mode = Some(CGRP_CountMode::CGRC_COUNT_STOP)
                },
                "count=previous" => {
                    item.count_mode = Some(CGRP_CountMode::CGRC_COUNT_PREVIOUS)
                },
                "count=block" => {
                    item.count_mode = Some(CGRP_CountMode::CGRC_COUNT_BLOCK)
                },
                "count=unblock" => {
                    item.count_mode = Some(CGRP_CountMode::CGRC_COUNT_UNBLOCK)
                }
                _default => {
                    log::error!("Invalid count mode");
                    process::exit(1);
                }
            }
            return false;
        }

        return item.regex.is_some();
    }

    fn parse_colors(line: &String) -> Vec<CGRCColorItem> {
        let mut items: Vec<CGRCColorItem> = vec![];
        let line_tokens = line.split(",");
        for line_token in line_tokens {
            let options = line_token.split(" ");
            let mut attrs: HashSet<CGRC_Attrib> = HashSet::new();
            let mut forg = LC_LogColor::LC_FORG_COL_DEFAULT;
            let mut back: LC_BackColor = LC_BackColor::LC_BACK_COL_DEFAULT;
            for option in options {
                let lower_option = option.to_lowercase();
                if COLORS_ATTRS.contains_key(option) {
                    attrs.insert(COLORS_ATTRS.get(&lower_option).unwrap().clone());
                    continue;
                }
                if COLORS_BACK.contains_key(option) {
                    back = COLORS_BACK.get(&lower_option).unwrap().clone();
                    continue;
                }
                if COLORS_FORG.contains_key(option) {
                    forg = COLORS_FORG.get(&lower_option).unwrap().clone();
                    continue;
                }
            }

            let item = CGRCColorItem::new(attrs, forg, back);
            items.push(item);
        }

        items
    }

    ///
    /// Parses the line.
    /// 
    pub fn parse_log_line(conf_items: &Vec<CGRCConfItem>, in_line: &String) -> Option<String> {
        let in_line_length = in_line.len();
        let mut char_colors: Vec<Option<&CGRCColorItem>> = vec![None; in_line_length];

        let mut stop_processing = false;
        for conf_item in conf_items {
            log::warn!("Conf:");
            if stop_processing {
                break;
            }

            let regex = conf_item.regex.as_ref().unwrap();
            if let Some(captures) = regex.captures(&in_line) {
                log::warn!("Capture:");
                if conf_item.skip.unwrap_or(false) {
                    return None;
                }

                stop_processing = conf_item.count_mode
                    .as_ref()
                    .unwrap_or(&CGRP_CountMode::CGRC_COUNT_STOP)
                    .clone() == CGRP_CountMode::CGRC_COUNT_STOP;
                for i in 1..captures.len() {
                    let capture = match captures.get(i) {
                        None => continue,
                        Some(v) => v
                    };
                    let from = capture.start();
                    let to   = capture.end();
                    for j in from..to {
                        if i >= conf_item.colors.len() {
                            break;
                        }
                        if !conf_item.colors[i].attrs.contains(&CGRC_Attrib::CGRC_NONE) {
                            char_colors[j] = Some(&conf_item.colors[i]);
                            log::warn!("Color: {:?}", conf_item.colors[i]);
                        }
                    }
                }
            }
        }

        let mut formatted_line = String::new();
        let mut formatted_seq;
        let mut last_color = addr_of!(char_colors[0]);
        let mut last_index = 0;
        for i in 1..=in_line_length {
            if addr_of!(char_colors[i]) == last_color && i != in_line_length {
                continue;
            }
            let v_last_color = unsafe { *last_color };
            if v_last_color.is_none() {
                formatted_seq = "\x1b[0m".to_string() + &in_line[last_index..i].to_owned();
            }
            else {
                formatted_seq = v_last_color.unwrap().escape_seq.to_owned() + &in_line[last_index..i].to_owned() + &v_last_color.unwrap().clear_seq;
            }

            formatted_line += &formatted_seq;

            last_index = i;
            last_color = addr_of!(char_colors[i]);
        }

        formatted_line += &"\x1b[0m".to_string();

        return Some(formatted_line);
    }
}
