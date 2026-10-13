struct VertexInput {
    float3 position;
};

struct VertexOutput {
    float4 position : SV_Position;
    float3 view_position;
};

struct Camera {
    float4x4 view_projection;
    float3 position;
};

struct PushConstants {
    float3 sky_color;
    float sun_size;
    float3 sun_direction;
    float sun_sharpness;
    float3 sun_color;
    float atmosphere_thickness;
    float3 ground_color;
};

[[vk::binding(0, 0)]]
ConstantBuffer<Camera> camera;

[push_constant]
ConstantBuffer<PushConstants> pc;

[shader("vertex")]
VertexOutput vert_main(VertexInput input) {
    VertexOutput output;
    output.position = mul(camera.view_projection, float4(input.position + camera.position, 1.0));
    output.view_position = input.position;
    return output;
}

[shader("fragment")]
float4 frag_main(VertexOutput vertex) : SV_Target {
    float3 view_direction = normalize(vertex.view_position);

    // Vertical angle and horizon
    float y = view_direction.y;
    float horizon_blend = smoothstep(-0.02, 0.02, y);

    // Sky gradient
    float vertical_t = saturate(y);
    float3 sky_gradient = lerp(pc.sky_color, pc.sky_color * 0.5, vertical_t);

    // Sun angle cosine
    float cos_sun_angle = max(dot(view_direction, -pc.sun_direction), 0.0);

    // Rayleigh scattering
    float rayleigh_phase = 0.75 * (1.0 + cos_sun_angle * cos_sun_angle);
    float3 rayleigh = sky_gradient * rayleigh_phase * pc.atmosphere_thickness;

    // Attenuation
    float sun_elevation = max(-pc.sun_direction.y, 0.0);
    float sun_path_factor = 1.0 / (sun_elevation + 0.05);
    float sun_attenuation = exp(-sun_path_factor * 0.04);

    // Mie scattering
    float mie_phase = pow(cos_sun_angle, pc.sun_sharpness);
    float3 mie = pc.sun_color * mie_phase * pc.atmosphere_thickness * 0.5 * sun_attenuation;

    // Sun disk
    float sun_disk = smoothstep(1.0 - pc.sun_size, 1.0 - pc.sun_size * (1.0 / pc.sun_sharpness), cos_sun_angle);
    float3 sun = pc.sun_color * sun_disk * 30.0 * sun_attenuation;

    // Combine sky
    float3 sky = rayleigh + mie + sun;

    // Blend sky and ground
    float3 final_color = lerp(pc.ground_color, sky, horizon_blend);

    return float4(final_color, 1.0);
}