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
 * Date:    2022.08.13
 * Company: -
 */

#include <QStandardPaths>
#include <QDir>
#include <QFileInfo>
#include <QString>

#include <lqtutils/lqtutils_string.h>

#include "cgrcconfmanager.h"
#include "cgrcparser.h"

CGRCConfManager::CGRCConfManager()
{}

QString CGRCConfManager::defaultSystemPath()
{
    QByteArray snapLocation = qgetenv("SNAP_DATA");
    if (!snapLocation.isEmpty())
        return snapLocation;
    return QSL("/etc/cgrc");
}

QString CGRCConfManager::defaultUserPath()
{
    QByteArray snapLocation = qgetenv("SNAP_USER_DATA");
    if (!snapLocation.isEmpty())
        return snapLocation;
    return QStandardPaths::writableLocation(QStandardPaths::AppConfigLocation);
}

QString CGRCConfManager::pathForConf(const QString& conf)
{
    // Try Qt resources first.
    QString proposedPath;

    proposedPath = QString(":%1%2%3")
            .arg(QSL("conf"))
            .arg(QDir::separator())
            .arg(conf);
    if (QFile(proposedPath).exists())
        return proposedPath;

    proposedPath = QString("%1%2%3")
            .arg(defaultUserPath())
            .arg(QDir::separator())
            .arg(conf);
    if (QFile(proposedPath).exists())
        return proposedPath;

    proposedPath = QString("%1%2%3")
            .arg(defaultSystemPath())
            .arg(QDir::separator())
            .arg(conf);
    if (QFile(proposedPath).exists())
        return proposedPath;

    qFatal("Cannot find conf file: %s", qPrintable(conf));
    return QString();
}

void CGRCConfManager::printAvailConfs()
{
    qInfo() << "Embedded configurations:";
    printAvailableConfs(QDir(QSL(":/conf")).entryInfoList());

    qInfo() << "User configurations:";
    printAvailableConfs(QDir(defaultUserPath()).entryInfoList());

    qInfo() << "System configurations:";
    printAvailableConfs(QDir(defaultSystemPath()).entryInfoList());
}

void CGRCConfManager::printAvailableConfs(const QFileInfoList& infos)
{
    for (const QFileInfo& info : infos) {
        QFile f(info.absoluteFilePath());
        if (!f.open(QIODevice::ReadOnly))
            continue;
        Conf c = CGRCParser::parseConf(f);
        qInfo().noquote() << "\t" << info.absoluteFilePath() << "->" << c.description;
    }
}
