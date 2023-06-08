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

use std::{process, fs::File, io::{BufReader, BufRead}};
use crate::cgrcconf::CGRCConf;

struct CGRCParser {

}

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
    fn parse_conf_lines<T: BufRead>(reader: T) -> CGRCConf {
        for line in reader.lines() {
            match line {
                Err(e) => {
                    log::error!("Failed to read conf file: {e}");
                    process::exit(1);
                },
                Ok(line) => {
                    log::info!("Line: {line}");
                }
            }
        }

        return CGRCConf {};
    }
}
