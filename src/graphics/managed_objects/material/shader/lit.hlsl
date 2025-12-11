struct DirectionalLight {
    float3 direction;
    float3 color;
    float brightness;
};

struct PointLight {
    float3 position;
    float radius;
    float3 color;
    float brightness;
};

struct SpotLight {
    float3 position;
    float distance;
    float3 direction;
    float inner_angle;
    float outer_angle;
    float3 color;
    float brightness;
};

struct VIn {
    float3 position : POSITION;
    float3 color : COLOR;
    float3 normal : NORMAL;

    float4 object0 : INST_OBJECT0;
    float4 object1 : INST_OBJECT1;
    float4 object2 : INST_OBJECT2;
    float4 object3 : INST_OBJECT3;
};

struct VOut {
    float4 position : SV_POSITION;
    float3 color : COLOR0;
    float3 pixel_position : FRAG_POSITION;
    float3 normal : NORMAL;
};

cbuffer Frame : register(b0) {
    uint frame;
    float time;
    float delta_time;
    float frame_reserved;
}

cbuffer Camera : register(b1) {
    row_major float4x4 camera_projection;
    float3 camera_position;
    float render_scale;
    float2 render_size;
    float2 inverse_render_size;
}

cbuffer Material : register(b2) {
    float3 material_color;
    float material_specular_strength;
}

cbuffer Lighting : register(b3) {
    float3 ambient_color;
    float ambient_intensity;
    uint num_directional_lights;
    uint num_point_lights;
    uint num_spot_lights;
    uint lighting_reserved;
}

StructuredBuffer<DirectionalLight> directional_lights : register(t0);
StructuredBuffer<PointLight> point_lights : register(t1);
StructuredBuffer<SpotLight> spot_lights : register(t2);

VOut vertex_main(VIn vin) {
    float4x4 object4 = float4x4(vin.object0, vin.object1, vin.object2, vin.object3);
    float3x3 object3 = float3x3(vin.object0.xyz, vin.object1.xyz, vin.object2.xyz);

    float4 position = mul(float4(vin.position, 1.0), object4);

    VOut vout;
    vout.position = mul(position, camera_projection);
    vout.color = vin.color * material_color;
    vout.pixel_position = position.xyz;
    vout.normal = mul(vin.normal, object3);
    return vout;
}

float3 calculate_diffuse(float3 normal, float3 light_direction, float3 light_color) {
    float diffuse_strength = max(dot(normal, light_direction), 0.0);
    return diffuse_strength * light_color;
}

float3 calculate_specular(float3 view_direction, float3 reflect_direction, float3 light_color) {
    float specular_strength = pow(max(dot(view_direction, reflect_direction), 0.0), 32);
    return material_specular_strength * specular_strength * light_color;
}

float3 calculate_directional_light(float3 normal, float3 position, DirectionalLight light) {
    float3 light_direction = -light.direction;
    float3 view_direction = normalize(camera_position - position);
    float3 reflect_direction = reflect(-light_direction, normal);

    float3 diffuse = calculate_diffuse(normal, light_direction, light.color);
    float3 specular = calculate_specular(view_direction, reflect_direction, light.color);

    return (diffuse + specular) * light.brightness;
}

float calculate_point_light_attenuation(float distance, float radius, float brightness) {
    float normalized_distance = distance / radius;

    return saturate(5.0 * (1.0 - normalized_distance)) * brightness / (1.0 + 25.0 * normalized_distance * normalized_distance);
}

float3 calculate_point_light(float3 normal, float3 position, PointLight light) {
    float3 light_vector = light.position - position;
    float distance = length(light_vector);
    float attenuation = calculate_point_light_attenuation(distance, light.radius, light.brightness);
    
    float3 light_direction = normalize(light_vector);
    float3 view_direction = normalize(camera_position - position);
    float3 reflect_direction = reflect(-light_direction, normal);
    
    float3 diffuse = calculate_diffuse(normal, light_direction, light.color);
    float3 specular = calculate_specular(view_direction, reflect_direction, light.color);

    return (diffuse + specular) * attenuation;
}

float3 calculate_spot_light(float3 normal, float3 position, SpotLight light) {
    float3 light_vector = light.position - position;
    float dot_dir = dot(normalize(light_vector), -light.direction);
    float spot_factor = saturate((dot_dir - cos(light.outer_angle)) / (cos(light.inner_angle) - cos(light.outer_angle)));

    float spot_attenuation = spot_factor * spot_factor;
    float distance = length(light_vector);
    float attenuation = calculate_point_light_attenuation(distance, light.distance, light.brightness) * spot_attenuation;
    
    float3 light_direction = normalize(light_vector);
    float3 view_direction = normalize(camera_position - position);
    float3 reflect_direction = reflect(-light_direction, normal);
    
    float3 diffuse = calculate_diffuse(normal, light_direction, light.color);
    float3 specular = calculate_specular(view_direction, reflect_direction, light.color);

    return (diffuse + specular) * attenuation;
}

float3 calculate_ambient() {
    return ambient_intensity * ambient_color;
}

float3 calculate_all_lights(float3 normal, float3 position) {
    float3 lighting = calculate_ambient();

    for (uint i = 0; i < num_directional_lights; i++)
        lighting += calculate_directional_light(normal, position, directional_lights[i]);

    for (uint i = 0; i < num_point_lights; i++)
        lighting += calculate_point_light(normal, position, point_lights[i]);

    for (uint i = 0; i < num_spot_lights; i++)
        lighting += calculate_spot_light(normal, position, spot_lights[i]);

    return lighting;
}

float4 pixel_main(VOut vout) : SV_TARGET {
    float3 normal = normalize(vout.normal);

    float3 lighting = calculate_all_lights(normal, vout.pixel_position);

    return float4(vout.color * lighting, 1.0);
}