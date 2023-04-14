#version 460 core

in VS_OUTPUT {
    vec2 TexturePosition;
} IN;

uniform sampler2D Texture;
out vec4 Color;

void main()
{
    Color = texture(Texture, IN.TexturePosition);
}