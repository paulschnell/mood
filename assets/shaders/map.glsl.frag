#version 450

const uint FLOOR =      1;
const uint CEILING =    2;

in VS_OUT {
    vec2 texCoordHorizontal;
    vec2 texCoordVertical;
    flat uint asdf;
} fs_in;

uniform sampler2D tx_floor;
uniform sampler2D tx_ceiling;
uniform sampler2D tx_wall;

out vec4 color;

void main() {
    if (fs_in.asdf == FLOOR) {
        // color = texture2D(tx_floor, fs_in.texCoordHorizontal);
    }

    if (fs_in.asdf == CEILING) {
        // color = texture2D(tx_ceiling, fs_in.texCoordHorizontal);
    }
    color = texture2D(tx_wall,fs_in.texCoordVertical);
}
