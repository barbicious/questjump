#version 330 core

layout (location = 0) in vec2 i_pos;
layout (location = 1) in vec2 i_uv;

out vec2 v_uv;

uniform mat4 u_proj;

void main() {
    gl_Position = u_proj * vec4(i_pos, -0.5, 1.0);

    v_uv = i_uv;
}