#version 330 core

out vec4 o_color;

in vec2 v_uv;

uniform sampler2D u_sampler;

void main() {
    o_color = texture2D(u_sampler, v_uv);
}