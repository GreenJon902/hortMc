#version 460 core

in VS_OUTPUT {
    vec2 TexturePosition;
} IN;

out vec4 Color;

uniform sampler2D Texture;

void main()
{
    Color = texture(Texture, IN.TexturePosition);
}