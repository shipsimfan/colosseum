struct VertexInput {
    float3 position;
};

struct VertexOutput {
    float4 position : SV_Position;
};

struct Light {
    float4x4 view_projection;
};

struct Object {
    float4x4 model;
};

struct PushConstants {
    uint light_index;
    uint object_index;
};

[[vk::binding(0, 0)]]
StructuredBuffer<Light> lights;

[[vk::binding(1, 0)]]
StructuredBuffer<Object> objects;

[push_constant]
ConstantBuffer<PushConstants> pc;

[shader("vertex")]
VertexOutput vert_main(VertexInput input) {
    Light light = lights[pc.light_index];
    Object object = objects[pc.object_index];

    VertexOutput output;
    float4x4 mvp = mul(light.view_projection, object.model);
    output.position = mul(mvp, float4(input.position, 1.0));
    return output;
}