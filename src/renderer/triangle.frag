#version 330 core

in VS_OUTPUT {
    vec3 Color;
} IN;

out vec4 Color;

void main()
{
    Color = vec4(1.0f, 0.0f, 0.0f, 1.0f);
}