struct VertexInput {
    float3 position;
    float3 color;
};

struct VertexOutput {
    float4 position : SV_Position;
    float3 color;
};

struct Camera {
    float4x4 view_projection;
};

struct Object {
    float4x4 model;
};

ConstantBuffer<Camera> camera;

[push_constant]
ConstantBuffer<ImmutablePtr<Object, AddressSpace.Device>> object;

[shader("vertex")]
VertexOutput vert_main(VertexInput input) {
    VertexOutput output;
    float4x4 mvp = mul(camera.view_projection, object->model);
    output.position = mul(mvp, float4(input.position, 1.0));
    output.color = input.color;
    return output;
}

[shader("fragment")]
float4 frag_main(VertexOutput vertex) : SV_Target {
    return float4(vertex.color, 1.0);
}