#version 450

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 uv;

uniform mat4 model;

out VS_OUT {
    vec2 uv;
} vs_out;

void main() {
    gl_Position = model * vec4(pos, 1.0);
    vs_out.uv = uv;
}