struct VertexOutput {
    float4 position : SV_Position;
    float2 uv : TEXCOORD0;
};

static VertexOutput vertices[4] = VertexOutput[](
    VertexOutput(float4(-1.0, -1.0, 0.0, 1.0), float2(0.0, 1.0)), // bottom-left
    VertexOutput(float4(-1.0,  1.0, 0.0, 1.0), float2(0.0, 0.0)), // top-left
    VertexOutput(float4( 1.0, -1.0, 0.0, 1.0), float2(1.0, 1.0)), // bottom-right
    VertexOutput(float4( 1.0,  1.0, 0.0, 1.0), float2(1.0, 0.0))  // top-right
);

[shader("vertex")]
VertexOutput main(uint vertexID : SV_VertexID) {
    return vertices[vertexID];
}