cmake_minimum_required(VERSION 3.26.0)

project(initia_proto VERSION 0.1.0 DESCRIPTION "proto for initia" LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

find_package(Protobuf REQUIRED)

# Find source files
file(GLOB_RECURSE SOURCES "../src/**.*.cc")
file(GLOB_RECURSE HEADERS "../src/**.*.h")
# Include header files
#include_directories("../src" "/opt/homebrew/include")
include_directories("../src" ${Protobuf_INCLUDE_DIRS})
#link_directories("/opt/homebrew/Cellar/protobuf/21.12/lib" "/opt/homebrew/lib" "/usr/local/lib" "/usr/lib")

# Create shared library
add_library(initia_proto SHARED ${SOURCES})

target_link_libraries(initia_proto ${Protobuf_LIBRARIES})
#target_include_directories(initia_proto PUBLIC ${Protobuf_INCLUDE_DIRS} ${HEADERS})

# Install library
install(TARGETS ${PROJECT_NAME} DESTINATION lib/${PROJECT_NAME})

# Install library headers
file(GLOB_RECURSE HEADERS "../src/**.*.h")
# message(STATUS "HEADERS: ${HEADERS}")
# install(DIRECTORY "${CMAKE_SOURCE_DIR}/" # source directory
#         DESTINATION "include" # target directory
#         FILES_MATCHING # install only matched files
#         PATTERN "*.h" # select header files
# )
macro(install_with_directory)
    set(optionsArgs "")
    set(oneValueArgs "DESTINATION")
    set(multiValueArgs "FILES")
    cmake_parse_arguments(CAS "${optionsArgs}" "${oneValueArgs}" "${multiValueArgs}" ${ARGN} )

    message(STATUS "CAS_DESTINATION: ${CAS_DESTINATION}")

    foreach(FILE ${CAS_FILES})
        get_filename_component(DIR ${FILE} DIRECTORY)
        cmake_path(RELATIVE_PATH DIR BASE_DIRECTORY ${CMAKE_SOURCE_DIR}/../src)
        #message(STATUS "DIR: ${FILE} -> ${DIR} : ${RELDIR}")
        INSTALL(FILES ${FILE} DESTINATION ${CAS_DESTINATION}/${DIR})
    endforeach()
endmacro(install_with_directory)
install_with_directory(   DESTINATION ${CMAKE_INSTALL_PREFIX}/include/${CMAKE_PROJECT_NAME}/   FILES ${HEADERS})