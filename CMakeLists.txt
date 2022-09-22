#
# This file is part of cgrc.
#
# Copyright (c) 2022 Luca Carlon
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, version 3.
#
# This program is distributed in the hope that it will be useful, but
# WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
# General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <http://www.gnu.org/licenses/>.
#

cmake_minimum_required(VERSION 3.14)

project(cgrc VERSION 0.6.0 LANGUAGES CXX)
add_definitions(-DAPP_VERSION="0.5.0")

set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(QT NAMES Qt6 Qt5 REQUIRED COMPONENTS Core)
find_package(Qt${QT_VERSION_MAJOR} REQUIRED COMPONENTS Core)

add_executable(cgrc
  main.cpp
  cgrc_data.h
  cgrc_conf.h
  cgrcparser.h cgrcparser.cpp
  cgrcconfmanager.h cgrcconfmanager.cpp
  cgrc_data.cpp
  conf.qrc
)
target_link_libraries(cgrc Qt${QT_VERSION_MAJOR}::Core)

install(TARGETS cgrc RUNTIME DESTINATION bin)
