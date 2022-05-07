cbuffer CameraBuffer {
    matrix projection;
}

cbuffer ObjectBuffer {
    matrix object;
    float4 tint;
}

struct VertexInputType {
    float4 position: POSITION;
    float4 color: COLOR;
    float2 uv: TEXCOORD;
};

struct PixelInputType {
    float4 position: SV_POSITION;
    float4 color: COLOR;
    float2 uv: TEXCOORD;
};

PixelInputType vertex_main(VertexInputType input) {
    PixelInputType output;

    output.position = mul(input.position, object);
    output.position = mul(output.position, projection);
    output.color = input.color * tint;
    output.uv = input.uv;

    return output;
}

float4 pixel_main(PixelInputType input) : SV_TARGET {
    return input.color;
}