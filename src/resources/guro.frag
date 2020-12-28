#version 330 core

in VS_OUTPUT {
    vec3 Color;
    vec2 TextureCoords;
    float Brightness;
} IN;

out vec4 Color;

uniform sampler2D Texture;
uniform sampler2D Texture_2;

uniform float color_coeff;
uniform float texture_coeff;

void main() {
    Color = (vec4(IN.Color, 1.0f) * color_coeff + mix(texture(Texture, IN.TextureCoords), texture(Texture_2, IN.TextureCoords), texture_coeff) * (1.0 - color_coeff)) * IN.Brightness;
}