struct VertexInput {
    float3 position;
};

struct VertexOutput {
    float4 position : SV_Position;
};

struct Light {
    float4x4 view_projection;
};

[[vk::binding(0, 0)]]
StructuredBuffer<Light> lights;

[push_constant]
ConstantBuffer<uint> index;

[shader("vertex")]
VertexOutput vert_main(VertexInput input) {
    VertexOutput output;
    output.position = mul(lights[index].view_projection, float4(input.position, 1.0));
    return output;
}