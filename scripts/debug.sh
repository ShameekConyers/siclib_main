rm -rf build
mkdir build
cd build &&
cmake .. -DCMAKE_CXX_COMPILER=clang++ -GNinja -DCMAKE_BUILD_TYPE="Debug" -DCMAKE_CXX_FLAGS_DEBUG="-O0 -g  -fsanitize=address" &&
cmake --build . &&
lldb -o run ./siclibcpp_test
cd ..
