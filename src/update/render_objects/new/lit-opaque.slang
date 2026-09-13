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
    uint spot_light_count;
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

struct SpotLight {
    float4 color;
    float3 position;
    float range;
    float3 direction;
    float cutoff_angle;
    float falloff_angle;
};

struct PushConstants {
    float4 color;
    float specular_strength;
    float shininess;
    uint object_index;
};

[[vk::binding(0, 0)]]
ConstantBuffer<Camera> camera;

[[vk::binding(0, 1)]]
StructuredBuffer<Object> objects;

[[vk::binding(0, 2)]]
ConstantBuffer<LightingMetadata> lighting_metadata;

[[vk::binding(1, 2)]]
StructuredBuffer<DirectionalLight> directional_lights;

[[vk::binding(2, 2)]]
StructuredBuffer<float4x4> directional_light_matrices;

[[vk::binding(3, 2)]]
Sampler2DArrayShadow directional_light_shadow_maps;

[[vk::binding(4, 2)]]
StructuredBuffer<PointLight> point_lights;

[[vk::binding(5, 2)]]
StructuredBuffer<SpotLight> spot_lights;

[[vk::binding(6, 2)]]
StructuredBuffer<float4x4> spot_light_matrices;

[[vk::binding(7, 2)]]
Sampler2DArrayShadow spot_light_shadow_maps;

[push_constant]
ConstantBuffer<PushConstants> pc;

[shader("vertex")]
VertexOutput vert_main(VertexInput input) {
    Object object = objects[pc.object_index];

    VertexOutput output;
    float4x4 mvp = mul(camera.view_projection, object.model);
    output.position = mul(mvp, float4(input.position, 1.0));
    output.world_position = mul(object.model, float4(input.position, 1.0)).xyz;
    output.color = input.color * pc.color.rgb;
    output.normal = mul(float3x3(object.model), input.normal);
    return output;
}

float diffuse(float3 light_direction, float3 normal) {
    return max(dot(normal, light_direction), 0.0);
}

float specular(float3 light_direction, float3 normal, float3 view_direction) {
    float3 reflect_direction = reflect(light_direction, normal);

    return pow(max(dot(view_direction, reflect_direction), 0.0), pc.shininess) * pc.specular_strength;
}

float direct_shadow_factor(float4 clip_position, Sampler2DArrayShadow shadow_map, uint layer) {
    float3 ndc = clip_position.xyz / clip_position.w;

    float2 tex_coords = ndc.xy * 0.5 + 0.5;
    float ref_depth = ndc.z;

    float depth = shadow_map.Sample(float3(tex_coords, layer));
    if (ref_depth > depth)
        return 0.0;
    return 1.0;
}

float shadow_factor(float3 position, float4x4 view_projection, Sampler2DArrayShadow shadow_map, uint index) {
    float4 clip_position = mul(view_projection, float4(position, 1.0));
    return direct_shadow_factor(clip_position, shadow_map, index);
}

float cascade_shadow_factor(float3 position, uint index)
{
    static const int NUM_CASCADES = 4;

    uint base = index * NUM_CASCADES;

    for (uint cascade = 0; cascade < NUM_CASCADES; cascade++)
    {
        float4x4 view_projection = directional_light_matrices[base + cascade];
        float4 clip_pos = mul(view_projection, float4(position, 1.0));

        // Check if position is within this cascade's frustum
        // Comparing against clipPos.w directly avoids the perspective divide
        if (clip_pos.w > 0.0 && all(abs(clip_pos.xy) <= clip_pos.w) && clip_pos.z >= 0.0 && clip_pos.z <= clip_pos.w)
        {
            return direct_shadow_factor(clip_pos, directional_light_shadow_maps, base + cascade);
        }
    }

    // Position not covered by any cascade - fully lit
    return 1.0;
}

float3 directional_light(
    DirectionalLight light,
    uint index,
    float3 normal,
    float3 view_direction,
    float3 position, 
    float4x4 view_projection
) {
    // Calculate diffuse contribution
    float diffuse = diffuse(-light.direction, normal);

    // Calculate specular contribution
    float specular = specular(-light.direction, normal, view_direction);

    // Calculate shadow factor
    float shadow_factor = cascade_shadow_factor(position, index);

    return (diffuse + specular) * shadow_factor * light.color.rgb;
}

float attenuation(float distance, float range) {
    float a = distance / range;
    float a2 = a * a;
    float a4 = a2 * a2;

    float c = saturate(1.0 - a4);

    return (c * c) / (distance * distance + 1.0);
}

float3 point_light(PointLight light, float3 normal, float3 view_direction, float3 position) {
    float3 difference = light.position - position;
    float3 direction = normalize(difference); 

    // Calculate diffuse contribution
    float diffuse = diffuse(direction, normal);

    // Calculate specular contribution
    float specular = specular(direction, normal, view_direction);

    // Calculate attenuation
    float distance = length(light.position - position);
    float attenuation = attenuation(distance, light.range);

    return (diffuse + specular) * attenuation * light.color.rgb;
}

float3 spot_light(
    SpotLight light, 
    uint index,
    float3 normal, 
    float3 view_direction, 
    float3 position, 
    float4x4 view_projection
) { 
    float3 difference = light.position - position;
    float3 direction = normalize(difference);

    // Calculate angle intensity
    float theta = max(dot(direction, -light.direction), 0.0);
    float epsilon = light.cutoff_angle - light.falloff_angle;
    float intensity = clamp((light.cutoff_angle - theta) / epsilon, 0.0, 1.0);

    // Calculate diffuse contribution
    float diffuse = diffuse(direction, normal);

    // Calculate specular contribution
    float specular = specular(direction, normal, view_direction);

    // Calculate attenuation
    float distance = length(light.position - position);
    float attenuation = attenuation(distance, light.range);

    // Calculate shadow factor
    float shadow_factor = shadow_factor(position, view_projection, spot_light_shadow_maps, index);

    return (diffuse + specular) * attenuation * intensity * shadow_factor * light.color.rgb;
}

[shader("fragment")]
float4 frag_main(VertexOutput vertex) : SV_Target {
    // Calculate common properties
    float3 normal = normalize(vertex.normal);
    float3 view_direction = normalize(vertex.world_position - camera.position);

    // Calculate light contributions
    float3 lighting = lighting_metadata.ambient_light.rgb;

    for (uint i = 0; i < lighting_metadata.directional_light_count; i++) {
        lighting += directional_light(
            directional_lights[i],
            i,
            normal,
            view_direction,
            vertex.world_position,
            directional_light_matrices[i],
        );
    }

    for (uint i = 0; i < lighting_metadata.point_light_count; i++)
        lighting += point_light(point_lights[i], normal, view_direction, vertex.world_position);

    for (uint i = 0; i < lighting_metadata.spot_light_count; i++) {
        lighting += spot_light(
            spot_lights[i],
            i,
            normal,
            view_direction,
            vertex.world_position,
            spot_light_matrices[i]
        );
    }

    return float4(vertex.color.rgb * lighting, 1.0);
}