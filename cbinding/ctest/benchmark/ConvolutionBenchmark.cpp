#include <benchmark/benchmark.h>
#include <rdsp.h>

static void BM_Convolution(benchmark::State& state)
{
    std::vector<float> irData;
    std::vector<float> input;

    const auto blockSize = state.range(0);
    input.resize(blockSize);
    irData.resize(1024);

    auto* obj = rdsp_convolution_processor_create(blockSize, irData.data(), irData.size());
    for (auto _ : state) {
        rdsp_convolution_processor_process(obj, input.data(), blockSize);
    }
    rdsp_convolution_processor_destroy(obj);
}

BENCHMARK(BM_Convolution)->RangeMultiplier(2)->Range(64, 4096);
BENCHMARK_MAIN();
