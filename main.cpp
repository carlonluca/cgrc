#include <QCoreApplication>
#include <QFile>
#include <iostream>

#include "lightlogger/lc_logging.h"

inline void format(const char* s, lightlogger::LC_LogColor color, lightlogger::LC_LogAttrib attrib)
{
    lightlogger::lc_font_change(stdout, attrib, color);
    fprintf(stdout, "%s", qPrintable(s));
    lightlogger::lc_font_reset(stdout);
}

int main(int argc, char** argv)
{
    while (!std::cin.eof()) {
        std::string line;
        std::getline(std::cin, line);
        format(line.c_str(), lightlogger::LC_LOG_COL_RED, lightlogger::LC_LOG_ATTR_UNDERLINE);
    }

    return 0;
}
