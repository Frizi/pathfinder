// Automatically generated from files in pathfinder/shaders/. Do not edit!
#include <metal_stdlib>
#include <simd/simd.h>

using namespace metal;

struct spvDescriptorSetBuffer0
{
    constant float2* uTextureSize [[id(0)]];
    constant float2* uFramebufferSize [[id(1)]];
};

struct main0_out
{
    float2 vTexCoord [[user(locn0)]];
    float4 gl_Position [[position]];
};

struct main0_in
{
    int2 aPosition [[attribute(0)]];
    int2 aTexCoord [[attribute(1)]];
};

vertex main0_out main0(main0_in in [[stage_in]], constant spvDescriptorSetBuffer0& spvDescriptorSet0 [[buffer(0)]])
{
    main0_out out = {};
    out.vTexCoord = float2(in.aTexCoord) / (*spvDescriptorSet0.uTextureSize);
    float2 position = ((float2(in.aPosition) / (*spvDescriptorSet0.uFramebufferSize)) * 2.0) - float2(1.0);
    out.gl_Position = float4(position.x, -position.y, 0.0, 1.0);
    return out;
}

