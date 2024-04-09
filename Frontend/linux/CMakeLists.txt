cmake_minimum_required(VERSION 3.20)
project(Summarize LANGUAGES C)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK REQUIRED IMPORTED_TARGET "gtk4")

add_executable(summarize src/main.c)
target_link_libraries(summarize PRIVATE PkgConfig::GTK)
