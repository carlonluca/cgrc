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

    qInstallMessageHandler(lightlogger::log_handler);
    QCoreApplication app(argc, argv);
    app.setApplicationVersion(APP_VERSION);

    QCommandLineParser parser;
    CGRCParser::parseCmd(&app, &parser);

    if (parser.isSet(QSL("version"))) {
        parser.showVersion();
        return 0;
    }

    if (parser.isSet(QSL("list-locations"))) {
        qInfo() << "System location:" << CGRCConfManager::defaultSystemPath();
        qInfo() << "User location:  " << CGRCConfManager::defaultUserPath();
        return 0;
    }

    const QStringList args = parser.positionalArguments();
    if (args.size() < 1)
        parser.showHelp(-1);

    const QString& confPath = CGRCConfManager::pathForConf(args[0]);
    QFile confFile(confPath);
    if (!confFile.exists()) {
        qCritical("File does not exist: %s", qPrintable(confPath));
        return -1;
    }

    if (!confFile.open(QIODevice::ReadOnly)) {
        qCritical("Cannot open conf file: %s", qPrintable(confPath));
        return -1;
    }

    const QList<ConfItem> confItems = CGRCParser::parseConf(confFile);

    while (!std::cin.eof()) {
        std::string line;
        std::getline(std::cin, line);

        QString formattedLine = CGRCParser::parseLogLine(confItems, QString::fromStdString(line));
        if (formattedLine.isEmpty())
            continue;

        QByteArray data = formattedLine.toLocal8Bit();
        fprintf(stdout, "%s\n", data.data());
        fflush(stdout);
    }

    return 0;
}
