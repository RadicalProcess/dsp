message("Build rust core dsp and capi")

execute_process(
        COMMAND cargo build
        COMMAND cargo build --release
        COMMAND cbindgen . --output dsp.h
        WORKING_DIRECTORY ${CMAKE_CURRENT_LIST_DIR})

find_library(DSP_CORE_DEBUG
        NAMES capi.lib
        PATHS ${CMAKE_CURRENT_LIST_DIR}/target/debug
        NO_DEFAULT_PATH)

find_library(DSP_CORE_RELEASE
        NAMES capi.lib
        PATHS ${CMAKE_CURRENT_LIST_DIR}/target/release
        NO_DEFAULT_PATH)

add_library(DSP::DSP INTERFACE IMPORTED)

target_include_directories(DSP::DSP
        INTERFACE
        "${CMAKE_CURRENT_LIST_DIR}")

target_link_libraries(DSP::DSP
        INTERFACE
        "$<$<CONFIG:Debug>:${DSP_CORE_DEBUG}>"
        "$<$<CONFIG:Release>:${DSP_CORE_RELEASE}>")