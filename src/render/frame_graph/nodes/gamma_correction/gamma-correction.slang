struct VertexOutput {
    float4 position : SV_Position;
    float2 uv : TEXCOORD0;
};

[[vk::binding(0, 0)]]
Sampler2D input;

float4 apply_gamma(float4 color) {
    return float4(pow(color.rgb, float3(1.0 / 2.2)), color.a);
}

[shader("fragment")]
float4 frag_main(VertexOutput vertex) : SV_Target {
    return apply_gamma(input.Sample(vertex.uv));
}