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

use std::collections::HashSet;
use std::cmp::Eq;
use regex::Regex;

///
/// Values to set attribs to text.
/// 
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug)]
pub enum CGRC_ResetAttrib {
    CGRC_RESET_NONE          = 0,
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

#[derive(Copy, Clone)]
enum CGRC_Color {
    LC_BLACK = 0,
    LC_RED = 1,
    LC_GREEN = 2,
    LC_YELLOW = 3,
    LC_BLUE = 4,
    LC_MAGENTA = 5,
    LC_CYAN = 6,
    LC_WHITE = 7,
    LC_DEFAULT = 9
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LC_BackColor {
    LC_BACK_COL_BLACK = 40 + CGRC_Color::LC_BLACK as isize,
    LC_BACK_COL_RED = 40 + CGRC_Color::LC_RED as isize,
    LC_BACK_COL_GREEN = 40 + CGRC_Color::LC_GREEN as isize,
    LC_BACK_COL_YELLOW = 40 + CGRC_Color::LC_YELLOW as isize,
    LC_BACK_COL_BLUE = 40 + CGRC_Color::LC_BLUE as isize,
    LC_BACK_COL_MAGENTA = 40 + CGRC_Color::LC_MAGENTA as isize,
    LC_BACK_COL_CYAN = 40 + CGRC_Color::LC_CYAN as isize,
    LC_BACK_COL_WHITE = 40 + CGRC_Color::LC_WHITE as isize,
    LC_BACK_BRIGHT_COL_BLACK = 100 + CGRC_Color::LC_BLACK as isize,
    LC_BACK_BRIGHT_COL_RED = 100 + CGRC_Color::LC_RED as isize,
    LC_BACK_BRIGHT_COL_GREEN = 100 + CGRC_Color::LC_GREEN as isize,
    LC_BACK_BRIGHT_COL_YELLOW = 100 + CGRC_Color::LC_YELLOW as isize,
    LC_BACK_BRIGHT_COL_BLUE = 100 + CGRC_Color::LC_BLUE as isize,
    LC_BACK_BRIGHT_COL_MAGENTA = 100 + CGRC_Color::LC_MAGENTA as isize,
    LC_BACK_BRIGHT_COL_CYAN = 100 + CGRC_Color::LC_CYAN as isize,
    LC_BACK_BRIGHT_COL_WHITE = 100 + CGRC_Color::LC_WHITE as isize,
    LC_BACK_COL_DEFAULT = 40 + CGRC_Color::LC_DEFAULT as isize
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum LC_LogColor {
    LC_FORG_COL_BLACK = 30 + CGRC_Color::LC_BLACK as isize,
    LC_FORG_COL_RED = 30 + CGRC_Color::LC_RED as isize,
    LC_FORG_COL_GREEN = 30 + CGRC_Color::LC_GREEN as isize,
    LC_FORG_COL_YELLOW = 30 + CGRC_Color::LC_YELLOW as isize,
    LC_FORG_COL_BLUE = 30 + CGRC_Color::LC_BLUE as isize,
    LC_FORG_COL_MAGENTA = 30 + CGRC_Color::LC_MAGENTA as isize,
    LC_FORG_COL_CYAN = 30 + CGRC_Color::LC_CYAN as isize,
    LC_FORG_COL_WHITE = 30 + CGRC_Color::LC_WHITE as isize,
    LC_FORG_BRIGHT_COL_BLACK = 90 + CGRC_Color::LC_BLACK as isize,
    LC_FORG_BRIGHT_COL_RED = 90 + CGRC_Color::LC_RED as isize,
    LC_FORG_BRIGHT_COL_GREEN = 90 + CGRC_Color::LC_GREEN as isize,
    LC_FORG_BRIGHT_COL_YELLOW = 90 + CGRC_Color::LC_YELLOW as isize,
    LC_FORG_BRIGHT_COL_BLUE = 90 + CGRC_Color::LC_BLUE as isize,
    LC_FORG_BRIGHT_COL_MAGENTA = 90 + CGRC_Color::LC_MAGENTA as isize,
    LC_FORG_BRIGHT_COL_CYAN = 90 + CGRC_Color::LC_CYAN as isize,
    LC_FORG_BRIGHT_COL_WHITE = 90 + CGRC_Color::LC_WHITE as isize,
    LC_FORG_COL_DEFAULT = 30 + CGRC_Color::LC_DEFAULT as isize
}

static COLORS_ATTRS: phf::Map<&'static str, CGRC_Attrib> = phf::phf_map! {
    "none" => CGRC_Attrib::CGRC_NONE,
    "unchanged" => CGRC_Attrib::CGRC_NONE,
    "default" => CGRC_Attrib::CGRC_RESET,
    "bold" => CGRC_Attrib::CGRC_BRIGHT,
    "underline" => CGRC_Attrib::CGRC_UNDERLINE,
    "blink" => CGRC_Attrib::CGRC_BLINK,
    "reverse" => CGRC_Attrib::CGRC_REVERSE,
    "concealed" => CGRC_Attrib::CGRC_HIDDEN,
    "dark" => CGRC_Attrib::CGRC_DIM,
    "italic" => CGRC_Attrib::CGRC_ITALIC,
    "rapidblink" => CGRC_Attrib::CGRC_RAPID_BLINK,
    "strikethrough" => CGRC_Attrib::CGRC_STRIKETHROUGH,
};

pub fn colors_attr_clear(attr: &CGRC_Attrib) -> CGRC_ResetAttrib {
    return match attr {
        CGRC_Attrib::CGRC_BRIGHT => CGRC_ResetAttrib::CGRC_RESET_BRIGHT,
        CGRC_Attrib::CGRC_NONE => CGRC_ResetAttrib::CGRC_RESET_NONE,
        CGRC_Attrib::CGRC_RESET => CGRC_ResetAttrib::CGRC_RESET_NONE,
        CGRC_Attrib::CGRC_DIM => CGRC_ResetAttrib::CGRC_RESET_DIM,
        CGRC_Attrib::CGRC_ITALIC => CGRC_ResetAttrib::CGRC_RESET_ITALIC,
        CGRC_Attrib::CGRC_UNDERLINE => CGRC_ResetAttrib::CGRC_RESET_UNDERLINE,
        CGRC_Attrib::CGRC_BLINK => CGRC_ResetAttrib::CGRC_RESET_BLINK,
        CGRC_Attrib::CGRC_RAPID_BLINK => CGRC_ResetAttrib::CGRC_RESET_RAPID_BLINK,
        CGRC_Attrib::CGRC_REVERSE => CGRC_ResetAttrib::CGRC_RESET_REVERSE,
        CGRC_Attrib::CGRC_HIDDEN => CGRC_ResetAttrib::CGRC_RESET_HIDDEN,
        CGRC_Attrib::CGRC_STRIKETHROUGH => CGRC_ResetAttrib::CGRC_RESET_HIDDEN
    };
}

static COLORS_BACK: phf::Map<&'static str, LC_BackColor> = phf::phf_map! {
    "on_black" => LC_BackColor::LC_BACK_COL_BLACK,
    "on_red" => LC_BackColor::LC_BACK_COL_RED,
    "on_green" => LC_BackColor::LC_BACK_COL_GREEN,
    "on_yellow" => LC_BackColor::LC_BACK_COL_YELLOW,
    "on_blue" => LC_BackColor::LC_BACK_COL_BLUE,
    "on_magenta" => LC_BackColor::LC_BACK_COL_MAGENTA,
    "on_cyan" => LC_BackColor::LC_BACK_COL_CYAN,
    "on_white" => LC_BackColor::LC_BACK_COL_WHITE,
    "on_bright_black" => LC_BackColor::LC_BACK_BRIGHT_COL_BLACK,
    "on_bright_red" => LC_BackColor::LC_BACK_BRIGHT_COL_RED,
    "on_bright_green" => LC_BackColor::LC_BACK_BRIGHT_COL_GREEN,
    "on_bright_yellow" => LC_BackColor::LC_BACK_BRIGHT_COL_YELLOW,
    "on_bright_blue" => LC_BackColor::LC_BACK_BRIGHT_COL_BLUE,
    "on_bright_magenta" => LC_BackColor::LC_BACK_BRIGHT_COL_MAGENTA,
    "on_bright_cyan" => LC_BackColor::LC_BACK_BRIGHT_COL_CYAN,
    "on_bright_white" => LC_BackColor::LC_BACK_BRIGHT_COL_WHITE,
};

static COLORS_FORG: phf::Map<&'static str, LC_LogColor> = phf::phf_map! {
    "black" => LC_LogColor::LC_FORG_COL_BLACK,
    "red" => LC_LogColor::LC_FORG_COL_RED,
    "green" => LC_LogColor::LC_FORG_COL_GREEN,
    "yellow" => LC_LogColor::LC_FORG_COL_YELLOW,
    "blue" => LC_LogColor::LC_FORG_COL_BLUE,
    "magenta" => LC_LogColor::LC_FORG_COL_MAGENTA,
    "cyan" => LC_LogColor::LC_FORG_COL_CYAN,
    "white" => LC_LogColor::LC_FORG_COL_WHITE,
    "bright_black" => LC_LogColor::LC_FORG_BRIGHT_COL_BLACK,
    "bright_red" => LC_LogColor::LC_FORG_BRIGHT_COL_RED,
    "bright_green" => LC_LogColor::LC_FORG_BRIGHT_COL_GREEN,
    "bright_yellow" => LC_LogColor::LC_FORG_BRIGHT_COL_YELLOW,
    "bright_blue" => LC_LogColor::LC_FORG_BRIGHT_COL_BLUE,
    "bright_magenta" => LC_LogColor::LC_FORG_BRIGHT_COL_MAGENTA,
    "bright_cyan" => LC_LogColor::LC_FORG_BRIGHT_COL_CYAN,
    "bright_white" => LC_LogColor::LC_FORG_BRIGHT_COL_WHITE,
};

///
/// Item containing a color for a line.
///
struct CGRCColorItem {
    pub attrs: HashSet<CGRC_Attrib>,
    pub forg: LC_LogColor,
    pub back: LC_BackColor,
    escape_seq: String,
    clear_seq: String
}

impl CGRCColorItem {
    ///
    /// Creates a new color item for a line.
    /// 
    pub fn new(&self, attrs: HashSet<CGRC_Attrib>, forg: LC_LogColor, back: LC_BackColor) -> CGRCColorItem {
        CGRCColorItem {
            attrs: attrs,
            forg: forg,
            back: back,
            escape_seq: self.build_escape_seq(),
            clear_seq: self.build_clear_seq()
        }
    }

    ///
    /// Builds the escape sequence.
    /// 
    fn build_escape_seq(&self) -> String {
        let mut seq = format!("{}[{};{}", 0x1b, self.forg as i32, self.back as i32);
        for attr in &self.attrs {
            seq += &format!(";{}", *attr as u32);
        }
        seq += "m";
        seq
    }

    ///
    /// Builds the clear sequence.
    /// 
    fn build_clear_seq(&self) -> String {
        if self.attrs.is_empty() {
            return String::new();
        }
        let mut seq = format!(
            "{}[{};{}",
            0x1b,
            LC_LogColor::LC_FORG_COL_DEFAULT as u32,
            LC_BackColor::LC_BACK_COL_DEFAULT as u32
        );
        for attr in &self.attrs {
            seq += &format!(";{}", colors_attr_clear(&attr) as u32);
        }
        seq += "m";
        seq
    }
}

struct CGRCConfItem {
    pub regex: Regex,
    pub colors: Vec<CGRCColorItem>,
    pub skip: bool,
    pub countMode: CGRP_CountMode
}

impl PartialEq for CGRCConfItem {
    fn eq(&self, other: &Self) -> bool {
        return self.regex.as_str() == other.regex.as_str();
    }
}

struct CGRCConf {
    pub items: Vec<CGRCConfItem>,
    pub description: String
}
