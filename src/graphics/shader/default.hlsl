cbuffer Camera {
    float4x4 camera_projection;
}

cbuffer Material {
    float4 material_color;
}

struct VIn {
    float3 position : POSITION;
    float3 color : COLOR;
};

struct VOut {
    float4 position : SV_POSITION;
    float4 color : COLOR0;
};

VOut vertex_main(VIn vin) {
    VOut vout;
    vout.position = mul(camera_projection, float4(vin.position, 1.0));
    vout.color = float4(vin.color, 1.0) * material_color;
    return vout;
}

float4 pixel_main(VOut vout) : SV_TARGET {
    return vout.color;
}