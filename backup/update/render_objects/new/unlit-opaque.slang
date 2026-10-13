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

struct Material {
    float4 color;
};

struct PushConstants {
    Material material;
    uint object_index;
};

[[vk::binding(0, 0)]]
ConstantBuffer<Camera> camera;

[[vk::binding(0, 1)]]
StructuredBuffer<Object> objects;

[push_constant]
ConstantBuffer<PushConstants> pc;

[shader("vertex")]
VertexOutput vert_main(VertexInput input) {
    Object object = objects[pc.object_index];
    
    VertexOutput output;
    float4x4 mvp = mul(camera.view_projection, object.model);
    output.position = mul(mvp, float4(input.position, 1.0));
    output.color = input.color * pc.material.color.rgb;
    return output;
}

[shader("fragment")]
float4 frag_main(VertexOutput vertex) : SV_Target {
    return float4(vertex.color, 1.0);
}