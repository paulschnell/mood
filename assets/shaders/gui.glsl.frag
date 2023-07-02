#version 450

in VS_OUT {
    vec2 uv;
} fs_in;

uniform sampler2D atlas;

out vec4 color;

void main() {
    color = texture2D(atlas, fs_in.uv);
}
