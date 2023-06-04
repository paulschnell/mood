#version 450

in vec2 texCoord;
in float redValue;

out vec4 final_color;

uniform sampler2D texture0;
uniform sampler2D texture1;

void main() {
    final_color = mix(texture(texture0, texCoord), texture(texture0, texCoord), 0.5) * vec4(redValue, 1.0, 1.0, 1.0);
    // final_color = vec4(1.0, 1.0, 1.0, 1.0);
}
