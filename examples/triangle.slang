static float2 POSITIONS[3] = float2[](
    float2(0.0, -0.5),
    float2(0.5, 0.5),
    float2(-0.5, 0.5)
);

static float3 COLORS[3] = float3[](
    float3(1.0, 0.0, 0.0),
    float3(0.0, 1.0, 0.0),
    float3(0.0, 0.0, 1.0)
);

struct VertexOutput {
    float3 color;
    float4 position : SV_Position;
};

[shader("vertex")]
VertexOutput vert_main(uint id : SV_VertexID) {
    VertexOutput output;
    output.position = float4(POSITIONS[id], 0.0, 1.0);
    output.color = COLORS[id];
    return output;
}

[shader("fragment")]
float4 frag_main(VertexOutput vert) : SV_Target {
    return float4(vert.color, 1.0);
}