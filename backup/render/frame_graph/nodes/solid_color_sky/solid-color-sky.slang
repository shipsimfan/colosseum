struct VertexOutput {
    float4 position : SV_Position;
    float2 uv : TEXCOORD0;
};

[push_constant]
ConstantBuffer<float4> color;

[shader("fragment")]
float4 frag_main(VertexOutput vertex) : SV_Target {
    return color;
}