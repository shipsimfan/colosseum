struct VertexOutput {
    float4 position : SV_Position;
    float2 uv : TEXCOORD0;
};

struct Luminance {
    float m, n, e, s, w;
    float ne, nw, se, sw;
    float highest, lowest, contrast;
};

struct Edge {
    bool is_horizontal;
    float pixel_step;
    float opposite_luminance, gradient;
};

static const float contrast_threshold = 0.0312;
static const float relative_threshold = 0.125;
static const float subpixel_blending = 0.75;

static const float edge_steps[10] = { 1.0, 1.5, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 4.0 };

[push_constant]
ConstantBuffer<float2> texel_size;

[[vk::binding(0, 0)]]
Sampler2D input;

float4 sample(float2 uv) {
    return input.SampleLevelZero(uv);
}

float sample_luminance(float2 uv) {
    return sample(uv).a;
}

float sample_luminance(float2 uv, float offset_x, float offset_y) {
    uv += texel_size * float2(offset_x, offset_y);
    return sample_luminance(uv);
}

Luminance sample_luminance_region(float2 uv) {
    Luminance l;
    
    l.m = sample_luminance(uv);
    l.n = sample_luminance(uv, 0.0, 1.0);
    l.e = sample_luminance(uv, 1.0, 0.0);
    l.s = sample_luminance(uv, 0.0, -1.0);
    l.w = sample_luminance(uv, -1.0, 0.0);

    l.ne = sample_luminance(uv, 1.0, 1.0);
    l.nw = sample_luminance(uv, -1.0, 1.0);
    l.se = sample_luminance(uv, 1.0, -1.0);
    l.sw = sample_luminance(uv, -1.0, -1.0);

    l.highest = max(l.m, max(max(l.n, l.s), max(l.e, l.w)));
    l.lowest = min(l.m, min(min(l.n, l.s), min(l.e, l.w)));
    l.contrast = l.highest - l.lowest;

    return l;
}

bool should_skip(Luminance l) {
    float threshold = max(contrast_threshold, l.highest * relative_threshold);
    return l.contrast < threshold;
}

float calculate_pixel_blend_factor(Luminance l) {
    float filter = 2.0 * (l.n + l.e + l.s + l.w);
    filter += l.ne + l.nw + l.se + l.sw;
    filter /= 12.0;
    filter = abs(filter - l.m);
    filter = saturate(filter / l.contrast);

    float blend_factor = smoothstep(0.0, 1.0, filter);

    return blend_factor * blend_factor * subpixel_blending;
}

Edge find_edge(Luminance l) {
    Edge e;

    float horizontal = 
        abs(l.n + l.s - 2.0 * l.m) * 2.0 +
        abs(l.ne + l.se - 2.0 * l.e) +
        abs(l.nw + l.sw - 2.0 * l.w);
    float vertical =
        abs(l.e + l.w - 2.0 * l.m) * 2.0 +
        abs(l.ne + l.nw - 2.0 * l.n) +
        abs(l.se + l.sw - 2.0 * l.s);
    
    e.is_horizontal = horizontal >= vertical;

    float p_luminance = e.is_horizontal ? l.n : l.e;
    float n_luminance = e.is_horizontal ? l.s : l.w;
    float p_gradient = abs(p_luminance - l.m);
    float n_gradient = abs(n_luminance - l.m);

    e.pixel_step = e.is_horizontal ? texel_size.y : texel_size.x;

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

float calculate_edge_blend_factor(Luminance l, Edge e, float2 uv) {
    float2 uv_edge = uv;
    float2 edge_step;
    if (e.is_horizontal) {
        uv_edge.y += e.pixel_step * 0.5;
        edge_step = float2(texel_size.x, 0.0);
    } else {
        uv_edge.x += e.pixel_step * 0.5;
        edge_step = float2(0.0, texel_size.y);
    }

    float edge_luminance = (l.m + e.opposite_luminance) * 0.5;
    float gradient_threshold = e.gradient * 0.25;

    float2 p_uv = uv_edge + edge_step;
    float p_luminance_delta = sample_luminance(p_uv) - edge_luminance;
    bool p_at_end = abs(p_luminance_delta) >= gradient_threshold;

    for (int i = 0; i < 9 && !p_at_end; i++) {
        p_uv += edge_step * edge_steps[i];
        p_luminance_delta = sample_luminance(p_uv) - edge_luminance;
        p_at_end = abs(p_luminance_delta) >= gradient_threshold;
    }

    if (!p_at_end)
        p_uv += edge_step * 8.0;

    float2 n_uv = uv_edge - edge_step;
    float n_luminance_delta = sample_luminance(n_uv) - edge_luminance;
    bool n_at_end = abs(n_luminance_delta) >= gradient_threshold;

    for (int i = 0; i < 9 && !n_at_end; i++) {
        n_uv -= edge_step * edge_steps[i];
        n_luminance_delta = sample_luminance(n_uv) - edge_luminance;
        n_at_end = abs(n_luminance_delta) >= gradient_threshold;
    }

    if (!n_at_end)
        n_uv -= edge_step * 8.0;

    float p_distance, n_distance;
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
        delta_sign = p_luminance_delta >= 0.0;
    } else {
        shortest_distance = n_distance;
        delta_sign = n_luminance_delta >= 0.0;
    }

    if (delta_sign == (l.m - edge_luminance >= 0.0))
        return 0;

    return 0.5 - shortest_distance / (p_distance + n_distance);
}

float4 apply_fxaa(float2 uv) {
    Luminance l = sample_luminance_region(uv);
    if (should_skip(l))
        return sample(uv);

    float pixel_blend = calculate_pixel_blend_factor(l);
    Edge e = find_edge(l);
    float edge_blend = calculate_edge_blend_factor(l, e, uv);
    float final_blend = max(pixel_blend, edge_blend);

    if (e.is_horizontal)
        uv.y += e.pixel_step * final_blend;
    else
        uv.x += e.pixel_step * final_blend;
    
    return float4(sample(uv).rgb, l.m);
}

[shader("fragment")]
float4 frag_main(VertexOutput vertex) : SV_Target {
    return apply_fxaa(vertex.uv);
}