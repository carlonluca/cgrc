#ifndef CGRCCONFMANAGER_H
#define CGRCCONFMANAGER_H

#include <QString>

class CGRCConfManager
{
public:
    CGRCConfManager();

    static QString defaultSystemPath();
    static QString defaultUserPath();
};

#endif // CGRCCONFMANAGER_H
