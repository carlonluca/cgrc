#include <QStandardPaths>

#include <lqtutils/lqtutils_string.h>

#include "cgrcconfmanager.h"

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
