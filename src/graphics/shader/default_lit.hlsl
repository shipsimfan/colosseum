struct DirectionalLight {
    float3 direction;
    float3 color;
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

cbuffer Camera : register(b0) {
    row_major float4x4 camera_projection;
    float3 camera_position;
    float camera_reserved;
}

cbuffer Material : register(b1) {
    float3 material_color;
    float material_specular_strength;
}

cbuffer Lighting : register(b2) {
    float3 ambient_color;
    float ambient_intensity;
    uint num_directional_lights;
    uint3 lighting_reserved;
}

StructuredBuffer<DirectionalLight> directional_lights : register(t0);

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

    return diffuse + specular;
}

float3 calculate_ambient() {
    return ambient_intensity * ambient_color;
}

float3 calculate_all_lights(float3 normal, float3 position) {
    float3 lighting = calculate_ambient();

    for (uint i = 0; i < num_directional_lights; i++)
        lighting += calculate_directional_light(normal, position, directional_lights[i]);

    return lighting;
}

float4 pixel_main(VOut vout) : SV_TARGET {
    float3 normal = normalize(vout.normal);

    float3 lighting = calculate_all_lights(normal, vout.pixel_position);

    return float4(vout.color * lighting, 1.0);
}