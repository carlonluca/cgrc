/*
 * This file is part of cgrc.
 *
 * Copyright (c) 2022 Luca Carlon
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

/**
 * Author:  Luca Carlon
 * Date:    2022.08.06
 * Company: -
 */

#ifndef CGRC_DATA_H
#define CGRC_DATA_H

#include <QMap>
#include <QSet>
#include <QRegularExpression>

#include "lightlogger/lc_logging.h"

using namespace lightlogger;

enum CGRC_Attrib {
    CGRC_NONE = -1,
    CGRC_RESET = LC_LogAttrib::LC_LOG_ATTR_RESET,
    CGRC_DEFAULT = LC_LogAttrib::LC_LOG_ATTR_RESET,
    CGRC_BRIGHT = LC_LogAttrib::LC_LOG_ATTR_BRIGHT,
    CGRC_UNDERLINE = LC_LogAttrib::LC_LOG_ATTR_UNDERLINE,
    CGRC_BLINK = LC_LogAttrib::LC_LOG_ATTR_BLINK,
    CGRC_REVERSE = LC_LogAttrib::LC_LOG_ATTR_REVERSE,
    CGRC_HIDDEN = LC_LogAttrib::LC_LOG_ATTR_HIDDEN,
    CGRC_DIM = LC_LogAttrib::LC_LOG_ATTR_DIM,
    CGRC_ITALIC = LC_LogAttrib::LC_LOG_ATTR_ITALIC,
    CGRC_RAPID_BLINK = LC_LogAttrib::LC_LOG_ATTR_RAPID_BLINK,
    CGRC_STRIKETHROUGH = LC_LogAttrib::LC_LOG_ATTR_STRIKETHROUGH
};

enum CGRC_ResetAttrib {
    CGRC_RESET_BRIGHT        = 22,
    CGRC_RESET_DIM           = 22,
    CGRC_RESET_ITALIC        = 23,
    CGRC_RESET_UNDERLINE     = 24,
    CGRC_RESET_BLINK         = 25,
    CGRC_RESET_RAPID_BLINK   = 26,
    CGRC_RESET_REVERSE       = 27,
    CGRC_RESET_HIDDEN        = 28,
    CGRC_RESET_STRIKETHROUGH = 29
};

enum CGRP_CountMode {
    CGRC_COUNT_ONCE,
    CGRC_COUNT_MORE,
    CGRC_COUNT_STOP,
    CGRC_COUNT_PREVIOUS,
    CGRC_COUNT_BLOCK,
    CGRC_COUNT_UNBLOCK
};

extern QMap<QString, CGRC_Attrib> COLORS_ATTRS;
extern QMap<CGRC_Attrib, CGRC_ResetAttrib> COLORS_ATTR_CLEAR;
extern QMap<QString, LC_LogColor> COLORS_FORG;
extern QMap<QString, LC_BackColor> COLORS_BACK;

struct ColorItem
{
    QSet<CGRC_Attrib> attrs;
    LC_LogColor forg = LC_LogColor::LC_FORG_COL_DEFAULT;
    LC_BackColor back = LC_BackColor::LC_BACK_COL_DEFAULT;

    // TODO: create only once
    QString escapeSequence() const {
        QString seq = QString("%1[%2;%3").arg(QChar(0x1b)).arg(forg).arg(back);
        for (const CGRC_Attrib& attr : attrs)
            seq += QString(";%1").arg(attr);
        seq += "m";
        return seq;
    }

    QString clearSequence() const {
        if (attrs.isEmpty())
            return QString();
        QString seq = QString("%1[%2;%3").arg(QChar(0x1b)).arg(LC_LogColor::LC_FORG_COL_DEFAULT).arg(LC_BackColor::LC_BACK_COL_DEFAULT);
        for (const CGRC_Attrib& attr : attrs)
            if (COLORS_ATTR_CLEAR.contains(attr))
                seq += QString(";%1").arg(COLORS_ATTR_CLEAR.value(attr));
        seq += "m";
        return seq;
    }
};

struct ConfItem
{
    QRegularExpression regexp;
    QList<ColorItem> colors;
    bool skip = false;
    CGRP_CountMode countMode = CGRP_CountMode::CGRC_COUNT_MORE;

    void clear() {
        regexp = QRegularExpression();
        colors = QList<ColorItem>();
        skip = false;
        countMode = CGRP_CountMode::CGRC_COUNT_MORE;
    }
};

inline bool operator==(const ConfItem& i1, const ConfItem& i2)
{
    return i1.regexp == i2.regexp;
}

#endif // CGRC_DATA_H
