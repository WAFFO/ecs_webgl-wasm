// #version set in common_functions.glsl
layout(location = 0) in vec3 a_position;

uniform mat4 u_projection;
uniform mat4 u_view;
uniform vec3 u_translation;
uniform vec3 u_rotation;
uniform vec3 u_scale;
uniform vec4 u_color;

out vec4 v_color;

void main() {
    mat4 model = translate(u_translation) * rotate_x(u_rotation.x) * rotate_y(u_rotation.y) * rotate_z(u_rotation.z) * scale(u_scale) ;

    gl_Position = u_projection * u_view * model * vec4(a_position, 1.0);

    v_color = u_color;
}