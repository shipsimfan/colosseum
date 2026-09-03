struct VertexInput {
    float3 position;
};

struct VertexOutput {
    float4 position : SV_Position;
};

struct Camera {
    float4x4 view_projection;
};

struct PushConstants {};

[[vk::binding(0, 0)]]
ConstantBuffer<Camera> camera;

[shader("vertex")]
VertexOutput vert_main(VertexInput input) {
    VertexOutput output;
    output.position = mul(camera.view_projection, float4(input.position, 1.0));
    return output;
}

[shader("fragment")]
float4 frag_main(VertexOutput vertex) : SV_Target {
    return 1.0;
}