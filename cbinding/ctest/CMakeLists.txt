cmake_minimum_required(VERSION 3.14)

project(cbinding_test)
add_subdirectory(Thirdparty)

add_library(rdsp INTERFACE)
target_include_directories(rdsp INTERFACE ${CMAKE_CURRENT_LIST_DIR}/../../target)
target_link_libraries(rdsp INTERFACE ${CMAKE_CURRENT_LIST_DIR}/../../target/release/librdsp_cbinding.a)


add_executable(rdsp_TestHost
        test/ConvolutionTestCase.cpp)
target_link_libraries(rdsp_TestHost
        PRIVATE
        rdsp
        gtest
        gmock
        gmock_main)
set_target_properties(rdsp_TestHost PROPERTIES
        CXX_STANDARD 17
        CXX_STANDARD_REQUIRED YES
        CXX_EXTENSIONS YES)

add_executable(rdsp_Benchmark
        benchmark/ConvolutionBenchmark.cpp)
target_link_libraries(rdsp_Benchmark
        PRIVATE
        rdsp
        benchmark::benchmark)
set_target_properties(rdsp_Benchmark PROPERTIES
        CXX_STANDARD 17
        CXX_STANDARD_REQUIRED YES
        CXX_EXTENSIONS YES)




