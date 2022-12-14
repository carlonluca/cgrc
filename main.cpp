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

#include <QCoreApplication>
#include <QCommandLineParser>
#include <QRegularExpression>
#include <QFile>
#include <QDebug>

#include <iostream>
#include <csignal>

#include "cgrc_data.h"
#include "cgrcparser.h"
#include "cgrcconfmanager.h"

#include "lqtutils/lqtutils_string.h"

int main(int argc, char** argv)
{
    signal(SIGINT, SIG_IGN);

#ifdef QT_DEBUG
    qInstallMessageHandler(lightlogger::log_handler);
#endif

    QCoreApplication app(argc, argv);
    app.setApplicationVersion(APP_VERSION);

    QCommandLineParser parser;
    CGRCParser::parseCmd(&app, &parser);

    if (parser.isSet(QSL("version"))) {
        parser.showVersion();
        return 0;
    }

    if (parser.isSet(QSL("list-locations"))) {
        qInfo() << "Locations on your system used by cgrc:";
        qInfo().noquote() << "\tSystem location:" << CGRCConfManager::defaultSystemPath();
        qInfo().noquote() << "\tUser location  :" << CGRCConfManager::defaultUserPath();
        return 0;
    }

    if (parser.isSet(QSL("location-user"))) {
        qInfo().noquote() << CGRCConfManager::defaultUserPath();
        return 0;
    }

    if (parser.isSet(QSL("location-system"))) {
        qInfo().noquote() << CGRCConfManager::defaultSystemPath();
        return 0;
    }

    if (parser.isSet(QSL("list-configurations"))) {
        CGRCConfManager::printAvailConfs();
        return 0;
    }

    const QStringList args = parser.positionalArguments();
    if (args.size() < 1)
        parser.showHelp(-1);

    const bool isConfLocalPath = parser.isSet(QSL("conf-path"));
    const QString& confPath = CGRCConfManager::pathForConf(args[0], isConfLocalPath);
    if (confPath.isEmpty())
        return -1;

    QFile confFile(confPath);
    if (!confFile.open(QIODevice::ReadOnly)) {
        qCritical("Cannot open conf file: %s", qPrintable(confPath));
        return -1;
    }

    const CGRCConf conf = CGRCParser::parseConf(confFile);
    const QList<CGRCConfItem>& confItems = conf.items;
    const QString inputLocalPath = parser.value(QSL("input-path"));

    QScopedPointer<QTextStream> stream;
    QScopedPointer<QFile> inputFile;
    if (inputLocalPath.isEmpty())
        stream.reset(new QTextStream(stdin));
    else {
        inputFile.reset(new QFile(inputLocalPath));
        if (!inputFile->open(QIODevice::ReadOnly)) {
            qCritical() << "Could not open file:" << inputLocalPath;
            return -1;
        }

        stream.reset(new QTextStream(inputFile.data()));
    }
    while (!stream->atEnd()) {
        const QString line = stream->readLine();
        const QString formattedLine = CGRCParser::parseLogLine(confItems, line);
        if (Q_UNLIKELY(formattedLine.isEmpty()))
            continue;

        fprintf(stdout, "%s\n", qPrintable(formattedLine));
    }

    fflush(stdout);
    return 0;
}
