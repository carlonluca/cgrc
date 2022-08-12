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
