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

use std::{process, fs::File, io::{BufReader, BufRead}, thread::current, collections::HashSet};
use regex::Regex;
use crate::{
    cgrcconfmanager::CGRCConfManager,
    cgrcdata::{
        CGRCColorItem,
        CGRP_CountMode,
        CGRCConfItem,
        CGRCConf,
        CGRC_Attrib,
        LC_LogColor,
        LC_BackColor,
        COLORS_ATTRS, COLORS_FORG, COLORS_BACK
    }
};

pub struct CGRCParser {

}

impl CGRCParser {
    ///
    /// Parsers a conf file.
    /// 
    pub fn parse_conf(conf_file: &String) -> CGRCConfManager {
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
    pub fn parse_conf_lines<T: BufRead>(reader: T) -> CGRCConfManager {
        let mut items: Vec<CGRCConfItem> = vec![];
        let mut item = CGRCConfItem {
            colors: None,
            regex: None,
            count_mode: None,
            skip: None
        };
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
                    log::info!("Line: {line}");
                    CGRCParser::parse_conf_line(&line, &mut conf,  &mut item);
                }
            }
        }

        return CGRCConfManager {};
    }

    // Private portion
    // ===============
    fn parse_conf_line(line: &String, conf: &mut CGRCConf, item: &mut CGRCConfItem) {
        if line.starts_with("desc=") {
            conf.description = Some(line.replace("desc=", ""));
            return;
        }

        if line.starts_with("regexp=") {
            item.regex = match Regex::new(line.replace("regexp=", "").as_str()) {
                Err(e) => {
                    log::error!("Failed to parse regex: {line}");
                    log::error!("{}", e.to_string());
                    process::exit(1);
                },
                Ok(r) => Some(r)
            };
        }

        if line.starts_with("colours=") {
            let color_items = CGRCParser::parse_colors(line);
            return;
        }
    }

    fn parse_colors(line: &String) -> Vec<CGRCColorItem> {
        let mut items: Vec<CGRCColorItem> = vec![];
        let mut line_tokens = line.split(",");
        for line_token in line_tokens {
            let options = line_token.split(" ");
            let mut attrs: HashSet<CGRC_Attrib> = HashSet::new();
            let mut forg = LC_LogColor::LC_FORG_COL_DEFAULT;
            let mut back: LC_BackColor = LC_BackColor::LC_BACK_COL_DEFAULT;
            for option in options {
                let lower_option = option.to_lowercase();
                if COLORS_FORG.contains_key(option) {
                    attrs.insert(COLORS_ATTRS.get(&lower_option).unwrap().clone());
                    continue;
                }
                if COLORS_BACK.contains_key(option) {
                    back = COLORS_BACK.get(&lower_option).unwrap().clone();
                    continue;
                }
                if COLORS_ATTRS.contains_key(option) {
                    forg = COLORS_FORG.get(&lower_option).unwrap().clone();
                    continue;
                }
            }

            let item = CGRCColorItem::new(attrs, forg, back);
            items.push(item);
        }

        items
    }
}
