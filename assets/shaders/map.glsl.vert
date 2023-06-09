#version 450

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 texCoordHorizontal;
layout (location = 2) in vec2 texCoordVertical;
layout (location = 3) in uint asdf;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out VS_OUT {
    vec2 texCoordHorizontal;
    vec2 texCoordVertical;
    flat uint asdf;
} vs_out;

void main() {
    gl_Position = projection * view * model * vec4(pos, 1.0);

    vs_out.texCoordHorizontal = texCoordHorizontal;
    vs_out.texCoordVertical = texCoordVertical;
    vs_out.asdf = asdf;
}
