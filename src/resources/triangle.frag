#version 330 core

in VS_OUTPUT {
    vec3 Color;
    vec2 Coords;
} IN;

out vec4 Color;

uniform sampler2D Texture;

void main()
{
    float cl_coeff = 0.001;
    float tx_coeff = 0.999;
    Color = vec4(IN.Color, 1.0f) * cl_coeff + texture(Texture, IN.Coords) * tx_coeff;
}