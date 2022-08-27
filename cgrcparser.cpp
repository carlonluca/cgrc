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
#include <QTextStream>
#include <QDebug>

#include "cgrcparser.h"
#include "cgrc_data.h"
#include "cgrc_conf.h"

#include "lqtutils/lqtutils_string.h"

const QString CGRCParser::RESET_SEQUENCE = QSL("\x1b[0m");

CGRCParser::CGRCParser()
{}

QString CGRCParser::parseLogLine(const QList<CGRCConfItem>& confItems, const QString& inLine)
{
#if QT_VERSION >= QT_VERSION_CHECK(5, 15, 0)
    qsizetype inLineLength = inLine.length();
#else
    int inLineLength = inLine.length();
#endif
    QString outLine = inLine;
    const CGRCColorItem** charColors = new const CGRCColorItem*[inLineLength];
    memset(charColors, 0, sizeof(CGRCColorItem*)*inLineLength);

#if DEBUG_LOG
    qInfo() << "-> Processing:" << inLine;
#endif

    bool stopProcessing = false;
    for (const CGRCConfItem& confItem : confItems) {
        QRegularExpressionMatchIterator matches = confItem.regexp.globalMatch(inLine);
        if (stopProcessing)
            break;
        if (matches.hasNext() && confItem.skip)
            return QString();
        stopProcessing = (confItem.countMode == CGRC_COUNT_STOP && matches.hasNext());
        while (matches.hasNext()) {
#if DEBUG_LOG
            qDebug() << "Regex matches:" << confItem.regexp.pattern();
#endif
            QRegularExpressionMatch match = matches.next();
            for (int i = 0; i < match.capturedLength() && i < confItem.colors.length(); i++) {
                int from = match.capturedStart(i);
                int to   = match.capturedEnd(i);
                for (int j = from; j < to; j++) {
                    if (!confItem.colors[i].attrs().contains(CGRC_NONE))
                        charColors[j] = &(confItem.colors[i]);
                }
            }

            if (confItem.countMode == CGRC_COUNT_ONCE) {
                break;
            }
        }
    }

    QString formattedLine;
    const CGRCColorItem* lastColor = charColors[0];
    int lastIndex = 0;
    for (int i = 1; i < inLineLength + 1; i++) {
        if (charColors[i] == lastColor && i != inLineLength)
            continue;
        int from = lastIndex;
        int to   = i;
        QString formattedSequence;
        if (!lastColor)
            formattedSequence = RESET_SEQUENCE + inLine.mid(lastIndex, i - lastIndex);
        else
            formattedSequence = lastColor->escapeSequence() + (inLine.mid(lastIndex, i - lastIndex)) + lastColor->clearSequence();

#if DEBUG_LOG
        qDebug() << inLine.mid(lastIndex, i - lastIndex) << "->" << formattedSequence;
#endif

        formattedLine.append(formattedSequence);

        lastIndex = i;
        lastColor = charColors[i];
    }

    formattedLine.append(RESET_SEQUENCE);

    delete[] charColors;

    return formattedLine;
}

void CGRCParser::parseCmd(const QCoreApplication* app, QCommandLineParser* parser)
{
    QCommandLineOption optHelp(QSL("version"),
                               QSL("Shows the version of the application."));
    QCommandLineOption optListLocations(QSL("list-locations"),
                                        QSL("Shows the location to use for the conf files."));
    QCommandLineOption optListConfs(QSL("list-configurations"),
                                    QSL("List available configurations."));
    QCommandLineOption optConfFilePath(QSL("conf-path"),
                                       QSL("The configuration argument is to be interpreted as a path to the configuration file."));
    QCommandLineOption optInputFilePath(QSL("input-path"),
                                        QSL("Path to input file."),
                                        QSL("input-path"));

#define APP_DESCRIPTION                                               \
    "Configurable terminal text formatter\n\n"                        \
    "cgrc formats text from stdin according to custom configuration " \
    "files and outputs the result with ANSI escape codes to stdout. " \
    "Configuration files includes a set of regular expressions with " \
    "the related format to be used to the match and the captures."

    parser->setApplicationDescription(QSL(APP_DESCRIPTION));
    parser->addHelpOption();
    parser->addPositionalArgument("conf", "Configuration file");
    parser->addOption(optHelp);
    parser->addOption(optListLocations);
    parser->addOption(optListConfs);
    parser->addOption(optConfFilePath);
    parser->addOption(optInputFilePath);
    parser->process(app->arguments());
}

QList<CGRCColorItem> CGRCParser::parseColors(const QString& colors)
{
    QList<CGRCColorItem> items;
    const QStringList captures = colors.split(QSL(","));
    for (const QString& capture : captures) {

#if QT_VERSION >= QT_VERSION_CHECK(5, 15, 0)
        const QStringList options = capture.split(QSL(" "), Qt::SkipEmptyParts);
#else
        const QStringList options = capture.split(QSL(" "), QString::SkipEmptyParts);
#endif

        QSet<CGRC_Attrib> attrs;
        LC_LogColor forg = LC_LogColor::LC_FORG_COL_DEFAULT;
        LC_BackColor back = LC_BackColor::LC_BACK_COL_DEFAULT;
        for (const QString& option : options) {
            // Attributes.
            QString lowerOption = option.toLower();
            if (COLORS_ATTRS.contains(lowerOption))
                attrs.insert(COLORS_ATTRS.value(lowerOption));
            if (COLORS_BACK.contains(lowerOption))
                back = COLORS_BACK.value(lowerOption);
            if (COLORS_FORG.contains(lowerOption))
                forg = COLORS_FORG.value(lowerOption);
        }

        CGRCColorItem colorItem(attrs, forg, back);
        items.append(colorItem);
    }

    return items;
}

CGRCConf CGRCParser::parseConf(QFile& confFile)
{
    CGRCConf ret;
    QList<CGRCConfItem>& retItems = ret.items;
    CGRCConfItem currentItem;
    QTextStream stream(&confFile);
    while (!stream.atEnd()) {
        QString line = stream.readLine();
        if (line.startsWith(QSL("desc="))) {
            ret.description = line.replace(QSL("desc="), QSL(""));
            continue;
        }

        if (line.startsWith(QSL("regexp="))) {
            currentItem.regexp = QRegularExpression(line.replace(QSL("regexp="), QSL("")));
            if (!currentItem.regexp.isValid()) {
                QString errorString = currentItem.regexp.errorString();
                int errorOffset = currentItem.regexp.patternErrorOffset();
                qWarning() << "Invalid regexp found in line:"
                           << line
                           << "->"
                           << errorString
                           << "at"
                           << errorOffset;
            }
            continue;
        }

        if (line.startsWith(QSL("colours="))) {
            currentItem.colors.append(parseColors(line.replace(QSL("colours="), QSL(""))));
            continue;
        }

        if (line.startsWith(QSL("skip="))) {
            currentItem.skip = (line.compare(QSL("skip=yes"), Qt::CaseInsensitive) == 0);
            continue;
        }

        if (line.startsWith(QSL("count="))) {
            if (line.compare(QSL("count=once"), Qt::CaseInsensitive) == 0) {
                currentItem.countMode = CGRC_COUNT_ONCE;
                continue;
            }
            else if (line.compare(QSL("count=more"), Qt::CaseInsensitive) == 0) {
                currentItem.countMode = CGRC_COUNT_MORE;
                continue;
            }
            else if (line.compare(QSL("count=stop"), Qt::CaseInsensitive) == 0) {
                currentItem.countMode = CGRC_COUNT_STOP;
                continue;
            }
            else if (line.compare(QSL("count=previous"), Qt::CaseInsensitive) == 0) {
                currentItem.countMode = CGRC_COUNT_PREVIOUS;
                continue;
            }
            else if (line.compare(QSL("count=block"), Qt::CaseInsensitive) == 0) {
                currentItem.countMode = CGRC_COUNT_BLOCK;
                continue;
            }
            else if (line.compare(QSL("count=unblock"), Qt::CaseInsensitive) == 0) {
                currentItem.countMode = CGRC_COUNT_UNBLOCK;
                continue;
            }
            else
                qWarning() << "Invalid count mode";
            break;
        }

        if (!currentItem.regexp.pattern().isEmpty()) {
            retItems.append(currentItem);
            currentItem.clear();
        }
    }

    if (!retItems.contains(currentItem))
        retItems.append(currentItem);

    return ret;
}
