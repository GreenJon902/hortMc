#version 330 core

layout (location = 0) in vec3 Position;

out VS_OUTPUT {
    vec3 Color;
} OUT;

void main()
{
    gl_Position = vec4(Position.xyz, 1.0);
    OUT.Color = Position.xyz;
}