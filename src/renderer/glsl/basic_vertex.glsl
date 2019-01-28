#version 300 es
in vec3 a_position;
in vec4 a_color;
//in vec3 a_normal;

uniform mat4 u_projection;
uniform mat4 u_view;
uniform mat4 u_matrix;

//out vec3 v_position;
out vec4 v_color;
//out vec3 v_normal;

void main() {
    vec3 v_position = vec3(u_matrix * vec4(a_position, 1.0));
//    v_normal = mat3(transpose(inverse(u_matrix))) * a_normal;

    gl_Position = u_projection * u_view * vec4(v_position, 1.0);

    v_color = a_color;
}