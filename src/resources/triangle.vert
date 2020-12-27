#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;
layout (location = 2) in vec2 TextureCoords;
layout (location = 3) in vec3 Normal;

out VS_OUTPUT {
    vec3 Position;
    vec3 Color;
    vec2 TextureCoords;
    vec3 Normal;
} OUT;

uniform mat4 camera;
uniform mat4 transform;
uniform mat4 transform_normal;

void main()
{
    gl_Position = camera * (transform * vec4(Position, 1.0));
    OUT.Position = vec3(transform * vec4(Position, 1.0));
    OUT.Color = Color;
    OUT.TextureCoords = TextureCoords;
    OUT.Normal = vec3(transform_normal * vec4(Normal, 1.0) - transform_normal * vec4(0.0));
}