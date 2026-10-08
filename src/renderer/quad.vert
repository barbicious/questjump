#version 330 core

layout (location = 0) in vec2 i_pos;

out vec2 v_uv;

void main() {
    gl_Position = vec4(i_pos, 0.0, 1.0);
    v_uv = vec2(clamp(i_pos, 0.0, 1.0));
}