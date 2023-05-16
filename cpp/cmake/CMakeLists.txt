cmake_minimum_required(VERSION 3.26.0)

project(initia_proto VERSION 0.1.0 DESCRIPTION "proto for initia" LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

find_package(Protobuf REQUIRED)

# Find source files
file(GLOB_RECURSE SOURCES "../src/**.*.cc")
# Include header files
#include_directories("../src" "/opt/homebrew/include")
include_directories("../src" ${Protobuf_INCLUDE_DIRS})
#link_directories("/opt/homebrew/Cellar/protobuf/21.12/lib" "/opt/homebrew/lib" "/usr/local/lib" "/usr/lib")

# Create shared library
add_library(initia_proto SHARED ${SOURCES})

target_link_libraries(initia_proto ${Protobuf_LIBRARIES})

# Install library
install(TARGETS ${PROJECT_NAME} DESTINATION lib/${PROJECT_NAME})

# Install library headers
file(GLOB_RECURSE HEADERS "../src/**.*.h")
message(STATUS "HEADERS: ${HEADERS}")
install(DIRECTORY "${CMAKE_SOURCE_DIR}/" # source directory
        DESTINATION "../src" # target directory
        FILES_MATCHING # install only matched files
        PATTERN "*.h" # select header files
)
