struct Vertex {
    float4 position : SV_POSITION;
    float2 tex : TEXCOORD0;
};

struct LuminanceData {
    /// The color of the target pixel
    float3 color;

    /// The luminance of the target pixel
    float t;

    /// The luminance of the the pixel at y + 1
    float n;

    /// The luminance of the the pixel at y - 1
    float s;

    /// The luminance of the the pixel at x + 1
    float e;

    /// The luminance of the the pixel at x - 1
    float w;

    /// The luminance of the the pixel at (x + 1, y + 1)
    float ne;

    /// The luminance of the the pixel at (x - 1, y + 1)
    float nw;

    /// The luminance of the the pixel at (x + 1, y + 1)
    float se;

    /// The luminance of the the pixel at (x - 1, y - 1)
    float sw;

    /// The highest difference between non-diagonal neighbouring luminances
    float highest;

    /// The lowest difference between non-diagonal neighbouring luminances
    float lowest;

    /// The difference between the highest and lowest non-diagonal neighbouring luminances
    float contrast;
};

cbuffer Camera : register(b1) {
    float4x4 camera_unused1;
    float4 camera_unused2;
    float2 camera_unused3;
    float2 inverse_render_size;
}

static const float contrast_threshold = 0.0312;
static const float relative_contrast_threshold = 0.063;

Texture2D input_texture : register(t0);
SamplerState input_sampler : register(s0);

float sample_luminance(float2 uv) {
    return input_texture.Sample(input_sampler, uv).w;
}

float sample_luminance(float2 uv, float u_offset, float v_offset) {
    uv += inverse_render_size * float2(u_offset, v_offset);
    return sample_luminance(uv);
}

LuminanceData sample_nearby_luminance(float2 uv) {
    LuminanceData l;
    
    float4 target_sample = input_texture.Sample(input_sampler, uv);

    l.color = target_sample.xyz;
    l.t = target_sample.w;
    l.n = sample_luminance(uv, 0, 1);
    l.s = sample_luminance(uv, 0, -1);
    l.e = sample_luminance(uv, 1, 0);
    l.w = sample_luminance(uv, -1, 0);

    l.ne = sample_luminance(uv, 1, 1);
    l.nw = sample_luminance(uv, -1, 1);
    l.se = sample_luminance(uv, 1, -1);
    l.sw = sample_luminance(uv, -1, -1);

    l.highest = max(max(max(max(l.n, l.s), l.e), l.w), l.t);
    l.lowest = min(min(min(min(l.n, l.s), l.e), l.w), l.t);
    l.contrast = l.highest - l.lowest;

    return l;
}

bool should_skip_pixel(LuminanceData l) {
    float threshold = max(contrast_threshold, relative_contrast_threshold * l.highest);
    return l.contrast < threshold;
}

float calculate_blend_factor(LuminanceData l) {
    float filter = 2 * (l.n + l.s + l.e + l.w);
    filter += l.ne + l.nw + l.se + l.sw;
    filter /= 12.0;
    filter = abs(filter - l.t);
    filter = saturate(filter / l.contrast);
    filter = smoothstep(0.0, 1.0, filter);
    return filter * filter;
}

float4 main(Vertex vertex) : SV_TARGET {
    LuminanceData l = sample_nearby_luminance(vertex.tex);

    if (should_skip_pixel(l)) {
        return float4(0.0, 0.0, 0.0, 1.0);
        //return float4(l.color, 1.0);
    }

    float blend_factor = calculate_blend_factor(l);

    return float4(blend_factor, blend_factor, blend_factor, 1.0);
}