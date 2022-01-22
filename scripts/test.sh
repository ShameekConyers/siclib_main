#!/bin/sh

# rm -rf build
cd "$(dirname "$0")"
cd ..
mkdir build
cd build &&
cmake .. \
  -DCMAKE_CXX_COMPILER=clang++ -DCMAKE_BUILD_TYPE="Debug" \
  -DCMAKE_CXX_FLAGS_DEBUG="-O0 -g -fsanitize=address" \
  -DSICLIB_TEST:BOOL=TRUE &&
cmake --build . &&
./siclibcpp_test
