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
use ::phf::phf_map;

///
/// Values to set attribs to text.
/// 
#[derive(Debug)]
pub enum CGRC_Attrib {
    CGRC_NONE          = -1,
    CGRC_RESET         = 0,
    CGRC_BRIGHT        = 1,
    CGRC_DIM           = 2,
    CGRC_ITALIC        = 3,
    CGRC_UNDERLINE     = 4,
    CGRC_BLINK         = 5,
    CGRC_RAPID_BLINK   = 6,
    CGRC_REVERSE       = 7,
    CGRC_HIDDEN        = 8,
    CGRC_STRIKETHROUGH = 9
}

///
/// Values to remove attribs to text.
/// 
pub enum CGRC_ResetAttrib {
    CGRC_RESET_BRIGHT        = 21,
    CGRC_RESET_DIM           = 22,
    CGRC_RESET_ITALIC        = 23,
    CGRC_RESET_UNDERLINE     = 24,
    CGRC_RESET_BLINK         = 25,
    CGRC_RESET_RAPID_BLINK   = 26,
    CGRC_RESET_REVERSE       = 27,
    CGRC_RESET_HIDDEN        = 28,
    CGRC_RESET_STRIKETHROUGH = 29
}

///
/// Values to set count mode.
/// 
pub enum CGRP_CountMode {
    CGRC_COUNT_ONCE,
    CGRC_COUNT_MORE,
    CGRC_COUNT_STOP,
    CGRC_COUNT_PREVIOUS,
    CGRC_COUNT_BLOCK,
    CGRC_COUNT_UNBLOCK
}

///
/// Map attrib name to value.
/// 
pub static COLORS_ATTRS: phf::Map<&'static str, CGRC_Attrib> = phf_map! {
    "none" => CGRC_Attrib::CGRC_NONE,
    "unchanged" => CGRC_Attrib::CGRC_NONE,
    "default" => CGRC_Attrib::CGRC_RESET
};
