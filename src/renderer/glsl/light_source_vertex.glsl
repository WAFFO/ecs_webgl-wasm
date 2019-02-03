#version 300 es
layout(location = 0) in vec3 a_position;

uniform mat4 u_projection;
uniform mat4 u_view;
uniform mat4 u_matrix;
uniform vec4 u_color;

out vec4 v_color;

void main() {

    gl_Position = u_projection * u_view * u_matrix * vec4(a_position, 1.0);

    v_color = u_color;
}