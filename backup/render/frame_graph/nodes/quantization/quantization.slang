struct VertexOutput {
    float4 position : SV_Position;
    float2 uv : TEXCOORD0;
};

struct Constants {
    float2 image_size;
    float2 texel_size;
    float sharpness;
}

[push_constant]
ConstantBuffer<Constants> constants;

[[vk::binding(0, 0)]]
Sampler2D input;

float3 cas(float3 c, float2 uv) {
    float3 a = input.Sample(uv + float2(0.0, constants.texel_size.y)).rgb;
    float3 b = input.Sample(uv - float2(constants.texel_size.x, 0.0)).rgb;
    float3 d = input.Sample(uv + float2(constants.texel_size.x, 0.0)).rgb;
    float3 e = input.Sample(uv - float2(0.0, constants.texel_size.y)).rgb;

    float al = a.g + 0.5 * (a.r + a.b);
    float bl = b.g + 0.5 * (b.r + b.b);
    float cl = c.g + 0.5 * (c.r + c.b);
    float dl = d.g + 0.5 * (d.r + d.b);
    float el = e.g + 0.5 * (e.r + e.b);

    float nz = 0.25 + (al + bl + dl + el) - cl;
    nz = clamp(nz, -1.0, 1.0);
    nz = abs(nz);

    float min_l = min(min(al, bl), min(dl, el));
    float max_l = max(max(al, bl), max(dl, el));

    float range = max_l - min_l;

    float hit_min = abs(min_l - cl) / (4.0 * max_l);
    float hit_max = abs(max_l - cl) / (4.0 * min_l);
    float lobe = max(-hit_max, hit_min);

    lobe *= (1.0 - nz);

    lobe *= constants.sharpness;

    float3 sharpened = c + lobe * (a + b + c + d - 4.0 * c);

    float3 min_rgb = min(min(a, b), min(d, e));
    float3 max_rgb = max(max(a, b), max(d, e));
    return clamp(sharpened, min_rgb, max_rgb);
}

float ign(float2 pos) {
    const float a = 52.9829189;
    const float b = 0.06711056;
    const float c = 0.00583715;

    return (a * (b * pos.x + c * pos.y) % 1.0) % 1.0;
}

[shader("fragment")]
float4 frag_main(VertexOutput vertex) : SV_Target {
    float4 sample = input.Sample(vertex.uv);

    float3 color = cas(sample.rgb, vertex.uv);
    float dither = ign(vertex.uv * constants.image_size) * (1.0 / 255.0);

    return float4(color + dither, 1.0);
}