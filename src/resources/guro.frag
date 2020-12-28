#version 330 core

in VS_OUTPUT {
    vec3 Color;
    vec2 TextureCoords;
    float Brightness;
} IN;

out vec4 Color;

uniform sampler2D Texture;

uniform float color_coeff;

void main() {
    Color = (vec4(IN.Color, 1.0f) * color_coeff + texture(Texture, IN.TextureCoords) * (1.0 - color_coeff)) * IN.Brightness;
}