struct VertexOutput {
    float4 position : SV_Position;
    float2 uv : TEXCOORD0;
};

[shader("vertex")]
VertexOutput vert_main(uint vertex_id : SV_VertexID) {
    VertexOutput vertex;
    vertex.uv = float2((vertex_id << 1) & 2, vertex_id & 2);
    vertex.position = float4(vertex.uv * 2.0f - 1.0f, 0.0f, 1.0f);
    return vertex;
}