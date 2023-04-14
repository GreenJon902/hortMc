#version 330 core

layout (location = 0) in vec2 Position;
layout (location = 1) in vec2 TexturePosition;

out VS_OUTPUT {
    vec2 TexturePosition;
} OUT;

void main()
{
    gl_Position = vec4(Position.xy, 1, 1.0);
    OUT.TexturePosition = TexturePosition;
}