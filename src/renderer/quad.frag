#version 330 core

in vec2 v_uv;

out vec4 o_color;

uniform sampler2D u_sampler;

void main() {
    o_color = texture2D(u_sampler, v_uv);
}