cmake_minimum_required(VERSION 3.24)

project(sample
        VERSION 0.0.1
        DESCRIPTION "dsvchc sample plugin")

add_library(sample SHARED
        sample.c)

set_target_properties(sample
        PROPERTIES VERSION ${PROJECT_VERSION})
set_target_properties(sample
        PROPERTIES SOVERSION ${PROJECT_VERSION_MAJOR})

target_include_directories(sample
        PRIVATE ../include)
