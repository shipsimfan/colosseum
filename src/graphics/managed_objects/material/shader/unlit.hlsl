cbuffer Camera : register(b0) {
    row_major float4x4 camera_projection;
}

cbuffer Material : register(b1) {
    float4 material_color;
}

struct VIn {
    float3 position : POSITION;
    float3 color : COLOR;

    float4 object0 : INST_OBJECT0;
    float4 object1 : INST_OBJECT1;
    float4 object2 : INST_OBJECT2;
    float4 object3 : INST_OBJECT3;
};

struct VOut {
    float4 position : SV_POSITION;
    float4 color : COLOR0;
};

VOut vertex_main(VIn vin) {
    float4x4 object = float4x4(vin.object0, vin.object1, vin.object2, vin.object3);

    VOut vout;
    vout.position = mul(mul(float4(vin.position, 1.0), object), camera_projection);
    vout.color = float4(vin.color, 1.0) * material_color;
    return vout;
}

float4 pixel_main(VOut vout) : SV_TARGET {
    return vout.color;
}