cmake_minimum_required(VERSION 3.10)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

project(main)
set(CMAKE_CXX_FLAGS "-Wall -Werror -O3")
add_executable(main main.cpp)
