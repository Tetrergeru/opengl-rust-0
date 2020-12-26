#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;
layout (location = 2) in vec2 Coords;

out VS_OUTPUT {
    vec3 Color;
    vec2 Coords;
} OUT;

uniform mat4 camera;

void main()
{
    gl_Position = camera * vec4(Position, 1.0);
    OUT.Color = Color;
    OUT.Coords = Coords;
}