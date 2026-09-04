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
    float3 sky_gradient = lerp(pc.sky_color * 0.5, pc.sky_color, vertical_t);

    // Scattering approximation
    float cos_sun_angle = dot(view_direction, -pc.sun_direction);
    float rayleigh = 1.0 + cos_sun_angle * cos_sun_angle;
    float mie = pow(max(cos_sun_angle, 0.0), pc.sun_sharpness);

    float3 scatter = pc.sky_color * rayleigh * pc.atmosphere_thickness;
    scatter += pc.sun_color * mie * pc.atmosphere_thickness;

    // Sun disk
    float sun_disk = smoothstep(1.0 - pc.sun_size, 1.0 - pc.sun_size * 0.5, cos_sun_angle);
    float3 sun = pc.sun_color * sun_disk;

    // Combine sky
    float3 sky = sky_gradient + scatter + sun;

    // Blend sky and ground
    float3 final_color = lerp(pc.ground_color, sky, horizon_blend);

    return float4(final_color, 1.0);
}