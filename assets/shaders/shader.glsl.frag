#version 450

in vec4 per_vertex_color;

out vec4 final_color;

void main() {
    final_color = per_vertex_color;
}
