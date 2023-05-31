#version 450

in vec2 texCoord;

out vec4 final_color;

uniform sampler2D texture0;

void main() {
    final_color = texture(texture0, texCoord);
    // final_color = vec4(1.0, 1.0, 1.0, 1.0);
}
