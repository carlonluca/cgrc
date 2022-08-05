#include "cgrc_data.h"
#include "lqtutils/lqtutils_string.h"

QMap<QString, CGRC_Attrib> COLORS_ATTRS = {
    { QSL("none"), CGRC_NONE },
    { QSL("unchanged"), CGRC_NONE },
    { QSL("default"), CGRC_RESET },
    { QSL("bold"), CGRC_BRIGHT },
    { QSL("underline"), CGRC_UNDERLINE },
    { QSL("blink"), CGRC_BLINK },
    { QSL("reverse"), CGRC_REVERSE },
    { QSL("concealed"), CGRC_HIDDEN },
    { QSL("dark"), CGRC_DIM },
    { QSL("italic"), CGRC_ITALIC },
    { QSL("rapidblink"), CGRC_RAPID_BLINK },
    { QSL("strikethrough"), CGRC_STRIKETHROUGH }
};

QMap<CGRC_Attrib, CGRC_ResetAttrib> COLORS_ATTR_CLEAR = {
    { CGRC_BRIGHT, CGRC_RESET_BRIGHT },
    { CGRC_DIM, CGRC_RESET_DIM },
    { CGRC_ITALIC, CGRC_RESET_ITALIC },
    { CGRC_UNDERLINE, CGRC_RESET_UNDERLINE },
    { CGRC_BLINK, CGRC_RESET_BLINK },
    { CGRC_RAPID_BLINK, CGRC_RESET_RAPID_BLINK },
    { CGRC_REVERSE, CGRC_RESET_REVERSE },
    { CGRC_HIDDEN, CGRC_RESET_HIDDEN },
    { CGRC_STRIKETHROUGH, CGRC_RESET_STRIKETHROUGH }
};

QMap<QString, LC_BackColor> COLORS_BACK = {
    { QSL("on_black"), LC_BackColor::LC_BACK_COL_BLACK },
    { QSL("on_red"), LC_BackColor::LC_BACK_COL_RED },
    { QSL("on_green"), LC_BackColor::LC_BACK_COL_GREEN },
    { QSL("on_yellow"), LC_BackColor::LC_BACK_COL_YELLOW },
    { QSL("on_blue"), LC_BackColor::LC_BACK_COL_BLUE },
    { QSL("on_magenta"), LC_BackColor::LC_BACK_COL_MAGENTA },
    { QSL("on_cyan"), LC_BackColor::LC_BACK_COL_CYAN },
    { QSL("on_white"), LC_BackColor::LC_BACK_COL_WHITE },
    { QSL("on_bright_black"), LC_BackColor::LC_BACK_BRIGHT_COL_BLACK },
    { QSL("on_bright_red"), LC_BackColor::LC_BACK_BRIGHT_COL_RED },
    { QSL("on_bright_green"), LC_BackColor::LC_BACK_BRIGHT_COL_GREEN },
    { QSL("on_bright_yellow"), LC_BackColor::LC_BACK_BRIGHT_COL_YELLOW },
    { QSL("on_bright_blue"), LC_BackColor::LC_BACK_BRIGHT_COL_BLUE },
    { QSL("on_bright_magenta"), LC_BackColor::LC_BACK_BRIGHT_COL_MAGENTA },
    { QSL("on_bright_cyan"), LC_BackColor::LC_BACK_BRIGHT_COL_CYAN },
    { QSL("on_bright_white"), LC_BackColor::LC_BACK_BRIGHT_COL_WHITE },
};

QMap<QString, LC_LogColor> COLORS_FORG = {
    { QSL("black"), LC_LogColor::LC_FORG_COL_BLACK },
    { QSL("red"), LC_LogColor::LC_FORG_COL_RED },
    { QSL("green"), LC_LogColor::LC_FORG_COL_GREEN },
    { QSL("yellow"), LC_LogColor::LC_FORG_COL_YELLOW },
    { QSL("blue"), LC_LogColor::LC_FORG_COL_BLUE },
    { QSL("magenta"), LC_LogColor::LC_FORG_COL_MAGENTA },
    { QSL("cyan"), LC_LogColor::LC_FORG_COL_CYAN },
    { QSL("white"), LC_LogColor::LC_FORG_COL_WHITE },
    { QSL("bright_black"), LC_LogColor::LC_FORG_BRIGHT_COL_BLACK },
    { QSL("bright_red"), LC_LogColor::LC_FORG_BRIGHT_COL_RED },
    { QSL("bright_green"), LC_LogColor::LC_FORG_BRIGHT_COL_GREEN },
    { QSL("bright_yellow"), LC_LogColor::LC_FORG_BRIGHT_COL_YELLOW },
    { QSL("bright_blue"), LC_LogColor::LC_FORG_BRIGHT_COL_BLUE },
    { QSL("bright_magenta"), LC_LogColor::LC_FORG_BRIGHT_COL_MAGENTA },
    { QSL("bright_cyan"), LC_LogColor::LC_FORG_BRIGHT_COL_CYAN },
    { QSL("bright_white"), LC_LogColor::LC_FORG_BRIGHT_COL_WHITE }
};
