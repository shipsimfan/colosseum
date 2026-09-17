struct VertexInput {
    float3 position;
};

struct VertexOutput {
    float4 position : SV_Position;
    float3 world_position;
    uint layer : SV_RenderTargetArrayIndex;
};

struct Object {
    float4x4 model;
};

struct PointLight {
    float4 color;
    float3 position;
    float range;
};

struct PushConstants {
    uint light_index;
    uint object_index;
};

[[vk::binding(0, 0)]]
StructuredBuffer<float4x4> light_matrices;

[[vk::binding(1, 0)]]
StructuredBuffer<Object> objects;

[[vk::binding(2, 0)]]
StructuredBuffer<PointLight> lights;

[push_constant]
ConstantBuffer<PushConstants> pc;

[shader("vertex")]
VertexOutput vert_main(VertexInput input, uint instance_id: SV_InstanceID) {
    float4x4 view_projection = light_matrices[pc.light_index + instance_id];
    Object object = objects[pc.object_index];

    float4 world_position = mul(object.model, float4(input.position, 1.0));

    VertexOutput output;
    output.world_position = world_position.xyz;
    output.position = mul(view_projection, world_position);
    output.layer = instance_id;
    return output;
}

[shader("fragment")]
float frag_main(VertexOutput input) : SV_Depth {
    PointLight light = lights[pc.light_index / 6];

    float3 light_to_fragment = input.world_position - light.position;
    float distance = length(light_to_fragment) / light.range;

    return distance;
}