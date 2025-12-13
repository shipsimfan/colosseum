struct Vertex {
    float4 position : SV_POSITION;
    float2 tex : TEXCOORD0;
};

struct LuminanceData {
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

struct EdgeData {
    bool is_horizontal;
    float pixel_step;
    float opposite_luminance;
    float gradient;
};

cbuffer Camera : register(b1) {
    float4x4 camera_unused1;
    float4 camera_unused2;
    float2 camera_unused3;
    float2 inverse_render_size;
}

static const float contrast_threshold = 0.0312;
static const float relative_contrast_threshold = 0.063;

static const int edge_step_count = 10;
static const float edge_steps[edge_step_count] = { 1, 1.5, 2, 2, 2, 2, 2, 2, 2, 4 };
static const float edge_guess = 8;

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

EdgeData detect_edge(LuminanceData l) {
    EdgeData e;
    float horizontal = abs(l.n + l.s - 2 * l.t) * 2 + 
                       abs(l.ne + l.se - 2 * l.e) +
                       abs(l.nw + l.sw - 2 * l.w);
    float vertical = abs(l.e + l.w - 2 * l.t) * 2 + 
                     abs(l.ne + l.nw - 2 * l.n) +
                     abs(l.se + l.sw - 2 * l.s);
    e.is_horizontal = horizontal >= vertical;

    e.pixel_step = e.is_horizontal ? inverse_render_size.y : inverse_render_size.x;

    float p_luminance = e.is_horizontal ? l.n : l.e;
    float n_luminance = e.is_horizontal ? l.s : l.w;
    float p_gradient = abs(p_luminance - l.t);
    float n_gradient = abs(n_luminance - l.t);

    if (p_gradient < n_gradient) {
        e.pixel_step = -e.pixel_step;
        e.opposite_luminance = n_luminance;
        e.gradient = n_gradient;
    } else {
        e.opposite_luminance = p_luminance;
        e.gradient = p_gradient;
    }

    return e;
}

float determine_edge_blend_factor(LuminanceData l, EdgeData e, float2 uv) {
    float2 uv_edge = uv;
    float2 edge_step;
    if (e.is_horizontal) {
        uv_edge.y += e.pixel_step * 0.5;
        edge_step = float2(inverse_render_size.x, 0.0);
    } else {
        uv_edge.x += e.pixel_step * 0.5;
        edge_step = float2(0.0, inverse_render_size.y);
    }

    float edge_luminance = (l.t + e.opposite_luminance) * 0.5;
    float gradient_threshold = e.gradient * 0.25;

    float2 p_uv = uv_edge;
    float p_luminance_delta;
    bool p_at_end = false;
    for (int i = 0; i < edge_step_count && !p_at_end; i++) {
        p_uv += edge_step * edge_steps[i];
        p_luminance_delta = sample_luminance(p_uv) - edge_luminance;
        p_at_end = abs(p_luminance_delta) >= gradient_threshold;
    }
    if (!p_at_end) {
        p_uv += edge_step * edge_guess;
    }

    float2 n_uv = uv_edge;
    float n_luminance_delta;
    bool n_at_end = false;
    for (int i = 0; i < edge_step_count && !n_at_end; i++) {
        n_uv -= edge_step * edge_steps[i];
        n_luminance_delta = sample_luminance(n_uv) - edge_luminance;
        n_at_end = abs(n_luminance_delta) >= gradient_threshold;
    }
    if (!n_at_end) {
        n_uv -= edge_step * edge_guess;
    }

    float p_distance;
    float n_distance;
    if (e.is_horizontal) {
        p_distance = p_uv.x - uv.x;
        n_distance = uv.x - n_uv.x;
    } else {
        p_distance = p_uv.y - uv.y;
        n_distance = uv.y - n_uv.y;
    }

    float shortest_distance;
    bool delta_sign;
    if (p_distance <= n_distance) {
        shortest_distance = p_distance;
        delta_sign = p_luminance_delta >= 0;
    } else {
        shortest_distance = n_distance;
        delta_sign = n_luminance_delta >= 0;
    }

    if (delta_sign == (l.t - edge_luminance >= 0)) {
        return 0;
    }

    return 0.5 - shortest_distance / (p_distance + n_distance);
}

float4 main(Vertex vertex) : SV_TARGET {
    LuminanceData l = sample_nearby_luminance(vertex.tex);

    if (should_skip_pixel(l)) {
        return float4(input_texture.Sample(input_sampler, vertex.tex).xyz, 1.0);
    }

    float blend_factor = calculate_blend_factor(l);

    EdgeData e = detect_edge(l);

    float edge_blend_factor = determine_edge_blend_factor(l, e, vertex.tex);
    float final_blend_factor = max(blend_factor, edge_blend_factor);

    float2 uv = vertex.tex;
    if (e.is_horizontal) {
        uv.y += e.pixel_step * final_blend_factor;
    } else {
        uv.x += e.pixel_step * final_blend_factor;
    }

    return float4(input_texture.Sample(input_sampler, uv).xyz, 1.0);
}