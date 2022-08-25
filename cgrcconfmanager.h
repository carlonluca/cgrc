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

#ifndef CGRCCONFMANAGER_H
#define CGRCCONFMANAGER_H

#include <QString>
#include <QFileInfo>

class CGRCConfManager
{
public:
    CGRCConfManager();

    static QString defaultSystemPath();
    static QString defaultUserPath();

    static QString pathForConf(const QString& conf, bool localPath);
    static void printAvailConfs();

private:
    static void printAvailableConfs(const QFileInfoList &infos);
};

#endif // CGRCCONFMANAGER_H
