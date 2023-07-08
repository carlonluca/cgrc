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

use std::{process, fs::File, io::{BufReader, BufRead, Cursor}, collections::HashSet};
use std::ptr;
use fancy_regex::Regex;
use crate::cgrcdata::{
    CgrcColorItem,
    CgrcCountMode,
    CgrcConfItem,
    CgrcConf,
    CgrcAttrib,
    LcLogColor,
    LcBackColor,
    COLORS_ATTRS,
    COLORS_FORG,
    COLORS_BACK
};

pub struct CGRCParser {}

impl CGRCParser {
    ///
    /// Parsers a conf file.
    /// 
    pub fn parse_conf(conf_file: &String) -> CgrcConf {
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
    pub fn parse_conf_lines<T: BufRead>(reader: T) -> CgrcConf {
        let mut item = CgrcConfItem::new();
        let mut conf = CgrcConf {
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
                        item = CgrcConfItem::new();
                    }
                }
            }
        }

        if item.regex.is_some() {
            conf.items.push(item.clone());
        }

        conf
    }

    ///
    /// Parses a configuration from a string.
    /// 
    pub fn parse_conf_string(conf: String) -> CgrcConf {
        let cursor = Cursor::new(conf);
        let reader = BufReader::new(cursor);
        return CGRCParser::parse_conf_lines(reader);
    }

    // Private portion
    // ===============
    fn parse_conf_line(line: &String, conf: &mut CgrcConf, item: &mut CgrcConfItem) -> bool {
        let lline = line.to_lowercase();

        if lline.starts_with("desc=") {
            conf.description = Some(lline.replace("desc=", ""));
            return false;
        }

        if lline.starts_with("regexp=") {
            item.regex = match Regex::new(format!("{}", line.replace("regexp=", "")).as_str()) {
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
            item.colors.append(&mut CGRCParser::parse_colors(&lline.replace("colours=", "")));
            return false;
        }

        if lline.starts_with("skip=") {
            item.skip = Some(lline.to_lowercase() == "skip=yes");
            return false;
        }

        if lline.starts_with("count=") {
            match lline.as_str() {
                "count=once" => {
                    item.count_mode = Some(CgrcCountMode::CgrcCountOnce);
                },
                "count=more" => {
                    item.count_mode = Some(CgrcCountMode::CgrcCountMore);
                },
                "count=stop" => {
                    item.count_mode = Some(CgrcCountMode::CgrcCountStop)
                },
                "count=previous" => {
                    item.count_mode = Some(CgrcCountMode::CgrcCountPrevious)
                },
                "count=block" => {
                    item.count_mode = Some(CgrcCountMode::CgrcCountBlock)
                },
                "count=unblock" => {
                    item.count_mode = Some(CgrcCountMode::CgrcCountUnblock)
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

    fn parse_colors(line: &String) -> Vec<CgrcColorItem> {
        let mut items: Vec<CgrcColorItem> = vec![];
        let line_tokens = line.split(",");
        for line_token in line_tokens {
            let options = line_token.split(" ");
            let mut attrs: HashSet<CgrcAttrib> = HashSet::new();
            let mut forg = LcLogColor::LcForgColDefault;
            let mut back: LcBackColor = LcBackColor::LcBackColDefault;
            for option in options {
                let lower_option = option.to_lowercase();
                if COLORS_ATTRS.contains_key(option) {
                    attrs.insert(COLORS_ATTRS.get(&lower_option).unwrap().clone());
                    continue;
                }
                if COLORS_BACK.contains_key(option) {
                    back = COLORS_BACK.get(&lower_option).unwrap().clone();
                    log::info!("Found color: {:?}", back);
                    continue;
                }
                if COLORS_FORG.contains_key(option) {
                    forg = COLORS_FORG.get(&lower_option).unwrap().clone();
                    continue;
                }
            }

            let item = CgrcColorItem::new(attrs, forg, back);
            items.push(item);
        }

        items
    }

    ///
    /// Parses the line.
    /// 
    pub fn parse_log_line(conf_items: &Vec<CgrcConfItem>, in_line: &String, debug: bool) -> Option<String> {
        let in_line_length = in_line.len();
        let mut char_colors: Vec<*const CgrcColorItem> = vec![ptr::null(); in_line_length];
        let mut stop_processing = false;
        for conf_item in conf_items {
            if debug {
                log::debug!("Testing conf: {:?}", conf_item);
            }
            
            if stop_processing {
                break;
            }

            let count_mode = conf_item.count_mode.as_ref().unwrap_or(&CgrcCountMode::CgrcCountMore);
            let regex = conf_item.regex.as_ref().unwrap();
            for regex_match in regex.captures_iter(&in_line) {
                if let Ok(regex_match) = regex_match {
                    if conf_item.skip.unwrap_or(false) {
                        return None;
                    }
                    
                    for i in 0..regex_match.len() {
                        if i >= conf_item.colors.len() {
                            break;
                        }
    
                        let capture = match regex_match.get(i) {
                            None => continue,
                            Some(v) => v
                        };
                        if debug {
                            log::debug!("Captured: {:?}", capture.as_str());
                        }
                        let from = capture.start();
                        let to   = capture.end();
                        for j in from..to {
                            if !conf_item.colors[i].attrs.contains(&CgrcAttrib::CgrcNone) {
                                char_colors[j] = &(conf_item.colors[i]);
                                if debug {
                                    log::warn!("Color: {:?}", conf_item.colors[i]);
                                }
                            }
                        }
                    }

                    stop_processing = count_mode == &CgrcCountMode::CgrcCountStop;

                    if count_mode == &CgrcCountMode::CgrcCountOnce {
                        break;
                    }
                }
            }
            
        }

        let mut formatted_line = String::new();
        let mut formatted_seq;
        let mut last_color = char_colors[0];
        let mut last_index = 0;
        for i in 1..=in_line_length {
            if i != in_line_length && char_colors[i] == last_color {
                continue;
            }
            if last_color.is_null() {
                formatted_seq = "\x1b[0m".to_string().to_string() + &in_line[last_index..i].to_owned();
            }
            else {
                unsafe {
                    formatted_seq = (*last_color).escape_seq.to_owned()
                        + &in_line[last_index..i].to_owned()
                        + "\x1b[0m".to_string().as_str();
                }
            }

            formatted_line += &formatted_seq;

            if i != in_line_length {
                last_index = i;
                last_color = char_colors[i];
            }
        }

        formatted_line += &"\x1b[0;0m".to_string();

        return Some(formatted_line);
    }
}
