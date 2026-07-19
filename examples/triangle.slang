struct VertexInput {
    float3 position;
    float3 color;
};

struct VertexOutput {
    float4 position : SV_Position;
    float3 color;
};

[shader("vertex")]
VertexOutput vert_main(VertexInput input) {
    VertexOutput output;
    output.position = float4(input.position, 1.0);
    output.color = input.color;
    return output;
}

[shader("fragment")]
float4 frag_main(VertexOutput vertex) : SV_Target {
    return float4(vertex.color, 1.0);
}