struct Vertex {
    float4 position : SV_POSITION;
    float2 tex : TEXCOORD0;
};

Texture2D input_texture : register(t0);
SamplerState input_sampler : register(s0);

float4 main(Vertex vertex) : SV_TARGET {
    return input_texture.Sample(input_sampler, vertex.tex);
}