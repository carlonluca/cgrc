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

#ifndef CGRCPARSER_H
#define CGRCPARSER_H

#include <QString>
#include <QFile>

#include "cgrc_data.h"

class QCoreApplication;
class QCommandLineParser;

class CGRCParser
{
public:
    CGRCParser();

    static QString parseLogLine(const QList<ConfItem>& confItems, const QString& inLine);
    static void parseCmd(const QCoreApplication* app, QCommandLineParser* parser);
    static QList<ColorItem> parseColors(const QString& colors);
    static Conf parseConf(QFile& confFile);
};

#endif // CGRCPARSER_H
