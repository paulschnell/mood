#version 450

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 color;

out vec4 per_vertex_color;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * view * model * vec4(pos, 1.0);
    per_vertex_color = vec4(color, 1.0);
}
