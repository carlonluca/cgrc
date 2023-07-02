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

use std::collections::HashMap;
use std::include_bytes;

pub fn load_confs() -> HashMap<&'static str, &'static [u8]> {
    HashMap::from([
        ("dockerps", &include_bytes!("../conf/dockerps")[..]),
        ("dockerstats", &include_bytes!("../conf/dockerstats")[..]),
        ("logcat", &include_bytes!("../conf/logcat")[..]),
        ("nginx", &include_bytes!("../conf/nginx")[..]),
        ("ping", &include_bytes!("../conf/ping")[..]),
        ("prio", &include_bytes!("../conf/prio")[..])
    ])
}
