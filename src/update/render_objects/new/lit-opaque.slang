struct VertexInput {
    float3 position;
    float3 color;
    float3 normal;
};

struct VertexOutput {
    float4 position : SV_Position;
    float3 world_position;
    float3 color;
    float3 normal;
};

struct Camera {
    float4x4 view_projection;
    float3 position;
};

struct Object {
    float4x4 model;
};

struct LightingMetadata {
    float4 ambient_light;
    uint directional_light_count;
    uint point_light_count;
};

struct DirectionalLight {
    float4 color;
    float3 direction;
};

struct PointLight {
    float4 color;
    float3 position;
    float range;
};

struct PushConstants {
    float4 color;
    float specular_strength;
    float shininess;
    ImmutablePtr<Object, AddressSpace.Device> object;
};

[[vk::binding(0, 0)]]
ConstantBuffer<Camera> camera;

[[vk::binding(0, 1)]]
ConstantBuffer<LightingMetadata> lighting_metadata;

[[vk::binding(1, 1)]]
StructuredBuffer<DirectionalLight> directional_lights;

[[vk::binding(2, 1)]]
StructuredBuffer<PointLight> point_lights;

[push_constant]
ConstantBuffer<PushConstants> pc;

[shader("vertex")]
VertexOutput vert_main(VertexInput input) {
    VertexOutput output;
    float4x4 mvp = mul(camera.view_projection, pc.object->model);
    output.position = mul(mvp, float4(input.position, 1.0));
    output.world_position = mul(pc.object->model, float4(input.position, 1.0)).xyz;
    output.color = input.color * pc.color.rgb;
    output.normal = mul(float3x3(pc.object->model), input.normal);
    return output;
}

float diffuse(float3 light_direction, float3 normal) {
    return max(dot(normal, -light_direction), 0.0);
}

float specular(float3 light_direction, float3 normal, float3 view_direction) {
    float3 reflect_direction = reflect(light_direction, normal);

    return pow(max(dot(view_direction, reflect_direction), 0.0), pc.shininess);
}

float3 directional_light(DirectionalLight light, float3 normal, float3 view_direction) {
    // Calculate diffuse contribution
    float3 diffuse = diffuse(light.direction, normal) * light.color.rgb;

    // Calculate specular contribution
    float3 specular = specular(light.direction, normal, view_direction) * pc.specular_strength * light.color.rgb;

    return diffuse + specular;
}

float attenuation(float distance, float range) {
    float a = distance / range;
    float a2 = a * a;
    float a4 = a2 * a2;

    float c = saturate(1.0 - a4);

    return (c * c) / (distance * distance + 1.0);
}

float3 point_light(PointLight light, float3 normal, float3 view_direction, float3 position) {
    float3 difference = position - light.position;
    float3 direction = normalize(difference); 

    // Calculate diffuse contribution
    float3 diffuse = diffuse(direction, normal) * light.color.rgb;

    // Calculate specular contribution
    float3 specular = specular(direction, normal, view_direction) * pc.specular_strength * light.color.rgb;

    // Calculate attenuation
    float distance = length(light.position - position);
    float attenuation = attenuation(distance, light.range);

    return (diffuse + specular) * attenuation;
}

[shader("fragment")]
float4 frag_main(VertexOutput vertex) : SV_Target {
    // Calculate common properties
    float3 normal = normalize(vertex.normal);
    float3 view_direction = normalize(camera.position - vertex.world_position);

    // Calculate light contributions
    float3 lighting = lighting_metadata.ambient_light.rgb;

    for (uint i = 0; i < lighting_metadata.directional_light_count; i++)
        lighting += directional_light(directional_lights[i], normal, view_direction);

    for (uint i = 0; i < lighting_metadata.point_light_count; i++)
        lighting += point_light(point_lights[i], normal, view_direction, vertex.world_position);

    return float4(vertex.color.rgb * lighting, 1.0);
}