#version 450

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 uv;
layout (location = 3) in uint vtype;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out VS_OUT {
    vec2 uv;
    flat uint vtype;
} vs_out;

void main() {
    gl_Position = projection * view * model * vec4(pos, 1.0);
    
    vs_out.uv = uv;
    vs_out.vtype = vtype;
}
