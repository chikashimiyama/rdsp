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
    std::vector<float> input;
    input.resize(128, 0.0f);
    input[5] = 0.5f;

    auto* obj = rdsp_convolution_processor_create(input.size(), &irData_[0], irData_.size());
    rdsp_convolution_processor_process(obj, &input[0], input.size());

    for(auto i = 0; i < input.size(); ++i)
    {
        if(i == 15){
            EXPECT_LE(fabs(0.5f - input[i]), 0.00001f );
        }else{
            EXPECT_LE(fabs(0.0f - input[i]), 0.00001f );
        }
    }

    rdsp_convolution_processor_destroy(obj);
}

