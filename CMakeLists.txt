cmake_minimum_required(VERSION 3.14)

project(siclibcpp LANGUAGES CXX)


set(CMAKE_CXX_STANDARD 20)
set(CMAKE_OSX_ARCHITECTURES x86_64)
set(BINARY_NAME ${PROJECT_NAME})
set(TEST_NAME ${PROJECT_NAME}_test)
set(LIB_NAME ${PROJECT_NAME})


include(FetchContent)
include(ExternalProject)



# find_package(BISON 3.8.2 REQUIRED)
# find_package(Doxygen 1.9.3 REQUIRED)



FetchContent_Declare(
  gtest
  GIT_REPOSITORY "https://github.com/google/googletest"
  GIT_TAG "release-1.11.0"
)

FetchContent_Declare(
  fmt
  GIT_REPOSITORY "https://github.com/fmtlib/fmt"
  GIT_TAG "8.0.1"
)

FetchContent_Declare(
  range
  GIT_REPOSITORY "https://github.com/ericniebler/range-v3"
  GIT_TAG "0.11.0"
)
FetchContent_Populate(range) # header only libraries
FetchContent_MakeAvailable(gtest fmt)


file(GLOB_RECURSE my_src CONFIGURE_DEPENDS "src/siclibcpp/**")
file(GLOB_RECURSE test_dir CONFIGURE_DEPENDS "tests/siclibcpp/**")

add_library(${LIB_NAME} INTERFACE)

target_include_directories(${LIB_NAME} INTERFACE "./src" ${CMAKE_SOURCE_DIR}/build/_deps/range-src/include)
target_link_libraries(${LIB_NAME} INTERFACE fmt)

add_executable(${TEST_NAME} ${test_dir})
target_link_libraries(${TEST_NAME} gtest fmt ${LIB_NAME})

IF(SICLIB_TEST)
  enable_testing()
  add_test(main_test ${TEST_NAME})

  # add_custom_command(
  #   TARGET ${TEST_NAME} POST_BUILD
  #   COMMAND ctest -C $<CONFIGURATION> --output-on-failure)

  add_custom_target(
    run_tests ALL
    COMMAND cmake ${PROJECT_SOURCE_DIR} -DSICLIB_TEST:BOOL=FALSE > /dev/null
    COMMAND ctest -C $<CONFIGURATION> --output-on-failure
    DEPENDS ${TEST_NAME} ${LIB_NAME}
  )
endif()
