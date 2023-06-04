#version 450

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in float aRedValue;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec2 texCoord;
out float redValue;

void main() {
    gl_Position = projection * view * model * vec4(pos, 1.0);
    texCoord = aTexCoord;
    redValue = aRedValue;
}
