struct VIn {
    float2 position : POSITION;
    float2 uv : TEXCOORD0;
};

struct VOut {
    float4 position : SV_POSITION;
    float2 uv : TEXCOORD0;
};

VOut main(VIn vin) {
    VOut vout;
    vout.position = float4(vin.position, 0.0, 1.0);
    vout.uv = vin.uv;
    return vout;
}