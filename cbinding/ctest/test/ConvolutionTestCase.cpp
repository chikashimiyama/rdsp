#include <memory>

#include <rdsp.h>
#include <gmock/gmock.h>

using namespace testing;

class UnitTest_ConvolutionProcessor : public :: testing::Test
{
protected:
    void SetUp() override
    {
        irData_.resize(128, 0.0f);
        irData_[10] = 1.0;

    }
    void TearDown() override
    {
    }

    std::vector<float> irData_;
};

TEST_F(UnitTest_ConvolutionProcessor, dirac)
{
    auto input = std::vector<float>();
    input.resize(128, 0.0f);
    input[5] = 0.5f;

    auto* obj = rdsp_convolution_processor_create(input.size(), &irData_[0], irData_.size());
    rdsp_convolution_processor_process(obj, &input[0], input.size());

    for(auto i = 0; i < input.size(); ++i)
    {
        if(i == (10 + 5)){
            EXPECT_LE(fabs(0.5f - input[i]), 0.00001f );
        }else{
            EXPECT_LE(fabs(0.0f - input[i]), 0.00001f );
        }
    }

    rdsp_convolution_processor_destroy(obj);
}

TEST_F(UnitTest_ConvolutionProcessor, dirac_multiple_block)
{
    auto input1 = std::vector<float>();
    auto input2 = std::vector<float>();

    input1.resize(128, 0.0f);
    input2.resize(128, 0.0f);
    input1[127] = 0.5f;

    auto* obj = rdsp_convolution_processor_create(input1.size(), &irData_[0], irData_.size());
    rdsp_convolution_processor_process(obj, input1.data(), input1.size());
    rdsp_convolution_processor_process(obj, input2.data(), input2.size());

    for(auto i = 0; i < input2.size(); ++i)
    {
        if(i == 9){
            EXPECT_LE(fabs(0.5f - input2[i]), 0.00001f );
        }else{
            EXPECT_LE(fabs(0.0f - input2[i]), 0.00001f );
        }
    }

    rdsp_convolution_processor_destroy(obj);
}
