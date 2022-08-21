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

struct CGRCColorItem
{
    CGRCColorItem(const QSet<CGRC_Attrib>& attrs, LC_LogColor forg, LC_BackColor back) :
        m_attrs(attrs)
      , m_forg(forg)
      , m_back(back)
      , m_escapeSequence(buildEscapeSequence())
      , m_clearSequence(buildClearSequence()) {}

    CGRCColorItem(const QSet<CGRC_Attrib>& attrs) :
        m_attrs(attrs)
      , m_forg(LC_LogColor::LC_FORG_COL_DEFAULT)
      , m_back(LC_BackColor::LC_BACK_COL_DEFAULT)
      , m_escapeSequence(buildEscapeSequence())
      , m_clearSequence(buildClearSequence()) {}

    const QSet<CGRC_Attrib>& attrs() const { return m_attrs; }
    const LC_LogColor& forg() const { return m_forg; }
    const LC_BackColor& back() const { return m_back; }
    const QString& escapeSequence() const { return m_escapeSequence; }
    const QString& clearSequence() const { return m_clearSequence; }

private:
    QString buildEscapeSequence() const {
        QString seq = QString("%1[%2;%3").arg(QChar(0x1b)).arg(m_forg).arg(m_back);
        for (const CGRC_Attrib& attr : m_attrs)
            seq += QString(";%1").arg(attr);
        seq += "m";
        return seq;
    }

    QString buildClearSequence() const {
        if (m_attrs.isEmpty())
            return QString();
        QString seq = QString("%1[%2;%3").arg(QChar(0x1b)).arg(LC_LogColor::LC_FORG_COL_DEFAULT).arg(LC_BackColor::LC_BACK_COL_DEFAULT);
        for (const CGRC_Attrib& attr : m_attrs)
            if (COLORS_ATTR_CLEAR.contains(attr))
                seq += QString(";%1").arg(COLORS_ATTR_CLEAR.value(attr));
        seq += "m";
        return seq;
    }

private:
    QSet<CGRC_Attrib> m_attrs;
    LC_LogColor m_forg;
    LC_BackColor m_back;

    QString m_escapeSequence;
    QString m_clearSequence;
};

struct CGRCConfItem
{
    QRegularExpression regexp;
    QList<CGRCColorItem> colors;
    bool skip = false;
    CGRP_CountMode countMode = CGRP_CountMode::CGRC_COUNT_MORE;

    void clear() {
        regexp = QRegularExpression();
        colors = QList<CGRCColorItem>();
        skip = false;
        countMode = CGRP_CountMode::CGRC_COUNT_MORE;
    }
};

inline bool operator==(const CGRCConfItem& i1, const CGRCConfItem& i2)
{
    return i1.regexp == i2.regexp;
}

struct CGRCConf
{
    QList<CGRCConfItem> items;
    QString description;
};

#endif // CGRC_DATA_H
