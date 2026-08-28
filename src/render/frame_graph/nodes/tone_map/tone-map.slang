struct VertexOutput {
    float4 position : SV_Position;
    float2 uv : TEXCOORD0;
};

struct Constants {
    float contrast;
    float saturation;
    float exposure;
    float gamma;
}

[push_constant]
ConstantBuffer<Constants> constants;

[[vk::binding(0, 0)]]
Sampler2D input;

float3 contrast(float3 c) {
    return (c - 0.5) * constants.contrast + 0.5;
}

float3 saturation(float3 c) {
    float luminance = dot(c, float3(0.2126, 0.7152, 0.0722));
    return saturate(lerp(float3(luminance), c, constants.saturation));
}

float3 aces_film(float3 x) {
    const float a = 2.51;
    const float b = 0.03;
    const float c = 2.43;
    const float d = 0.59;
    const float e = 0.14;
    return saturate((x * (a * x + b)) / (x * (c * x + d) + e));
}

float3 apply_gamma(float3 x) {
    return pow(x, 1.0 / constants.gamma);
}

[shader("fragment")]
float4 frag_main(VertexOutput vertex) : SV_Target {
    float4 sample = input.Sample(vertex.uv);

    float3 color = aces_film(sample.rgb * constants.exposure);
    color = contrast(color);
    color = saturation(color);
    color = apply_gamma(color);

    return float4(color, sample.a);
}