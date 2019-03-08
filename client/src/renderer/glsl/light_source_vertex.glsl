// #version set in common_functions.glsl
layout(location = 0) in vec3 a_position;

uniform mat4 u_projection;
//uniform mat4 u_view;
uniform vec3 u_view_pos;
uniform vec3 u_view_target;
uniform mat4 u_model;
//uniform vec3 u_translation;
//uniform vec3 u_rotation;
//uniform vec3 u_scale;
uniform vec4 u_color;

out vec4 v_color;

void main() {
//    mat4 model = translate(u_translation) * rotate_x(u_rotation.x) * rotate_y(u_rotation.y) * rotate_z(u_rotation.z) * scale(u_scale) ;
    mat4 view = look_at(u_view_pos, u_view_target, vec3(0.0, 1.0, 0.0));

    gl_Position = u_projection * view * u_model * vec4(a_position, 1.0);

    v_color = u_color;
}