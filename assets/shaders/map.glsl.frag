#version 450

const uint UNDEFINED =  0;
const uint FLOOR =      1;
const uint CEILING =    2;
const uint WALL =       3;
const uint GATE =       4;

in VS_OUT {
    vec2 uv;
    flat uint vtype;
} fs_in;

uniform sampler2D tx_floor;
uniform sampler2D tx_ceiling;
uniform sampler2D tx_wall;
uniform sampler2D tx_gate;
uniform int bPause;

out vec4 color;

void main() {
    if (fs_in.vtype == FLOOR) {
        color = texture2D(tx_floor, fs_in.uv);
    } else if (fs_in.vtype == CEILING) {
        color = texture2D(tx_ceiling, fs_in.uv);
    } else if (fs_in.vtype == WALL) {
        color = texture2D(tx_wall,fs_in.uv);
    } else if (fs_in.vtype == GATE) {
        color = texture2D(tx_gate,fs_in.uv);
    } else if (fs_in.vtype == UNDEFINED) {
        color = vec4(1.0, 1.0, 1.0, 1.0);
    }

    if (bPause == 1) {
        color *= vec4(0.2, 0.2, 0.2, 1.0);
    }
}
