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

CGRCParser::CGRCParser()
{}

QString CGRCParser::parseLogLine(const QList<ConfItem>& confItems, const QString& inLine)
{
    QString outLine = inLine;
    const ColorItem** charColors = new const ColorItem*[inLine.length()];
    memset(charColors, 0, sizeof(ColorItem*)*inLine.length());

#if DEBUG_LOG
    qInfo() << "-> Processing:" << inLine;
#endif

    bool stopProcessing = false;
    for (const ConfItem& confItem : confItems) {
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
                    if (!confItem.colors[i].attrs.contains(CGRC_NONE))
                        charColors[j] = &(confItem.colors[i]);
                }
            }

            if (confItem.countMode == CGRC_COUNT_ONCE) {
                break;
            }
        }
    }

    QString formattedLine;
    const ColorItem* lastColor = charColors[0];
    int lastIndex = 0;
    for (int i = 1; i < inLine.length() + 1; i++) {
        if (charColors[i] == lastColor && i != inLine.length())
            continue;
        int from = lastIndex;
        int to   = i;
        QString formattedSequence;
        if (!lastColor)
            formattedSequence = QString(QSL("%1[%2m%3")).arg(QChar(0x1b)).arg(0).arg(inLine.mid(lastIndex, i - lastIndex));
        else {
            formattedSequence = lastColor->escapeSequence() + (inLine.mid(lastIndex, i - lastIndex)) + lastColor->clearSequence();
        }

#if DEBUG_LOG
        qDebug() << inLine.mid(lastIndex, i - lastIndex) << "->" << formattedSequence;
#endif

        formattedLine.append(formattedSequence);

        lastIndex = i;
        lastColor = charColors[i];
    }

    formattedLine.append(QString(QSL("%1[0m")).arg(QChar(0x1b)));

    delete[] charColors;

    return formattedLine;
}

void CGRCParser::parseCmd(const QCoreApplication* app, QCommandLineParser* parser)
{
    QCommandLineOption optHelp(QSL("version"), QSL("Shows the version of the application"));
    QCommandLineOption optListLocations(QSL("list-locations"), QSL("Shows the location to use for the conf files"));
    QCommandLineOption optListConfs(QSL("list-configurations"), QSL("List available configurations"));

    parser->setApplicationDescription("cgrc");
    parser->addHelpOption();
    parser->addPositionalArgument("conf", "Configuration file");
    parser->addOption(optHelp);
    parser->addOption(optListLocations);
    parser->addOption(optListConfs);
    parser->process(app->arguments());
}

QList<ColorItem> CGRCParser::parseColors(const QString& colors)
{
    QList<ColorItem> items;
    const QStringList captures = colors.split(QSL(","));
    for (const QString& capture : captures) {
        ColorItem colorItem;
#if QT_VERSION >= QT_VERSION_CHECK(5, 15, 0)
        const QStringList options = capture.split(QSL(" "), Qt::SkipEmptyParts);
#else
        const QStringList options = capture.split(QSL(" "), QString::SkipEmptyParts);
#endif
        for (const QString& option : options) {
            // Attributes.
            QString lowerOption = option.toLower();
            if (COLORS_ATTRS.contains(lowerOption))
                colorItem.attrs.insert(COLORS_ATTRS.value(lowerOption));
            if (COLORS_BACK.contains(lowerOption))
                colorItem.back = COLORS_BACK.value(lowerOption);
            if (COLORS_FORG.contains(lowerOption))
                colorItem.forg = COLORS_FORG.value(lowerOption);
        }
        items.append(colorItem);
    }

    return items;
}

Conf CGRCParser::parseConf(QFile& confFile)
{
    Conf ret;
    QList<ConfItem>& retItems = ret.items;
    ConfItem currentItem;
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
