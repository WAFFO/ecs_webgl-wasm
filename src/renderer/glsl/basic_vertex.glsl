// #version set in common_functions.glsl
layout(location = 0) in vec3 a_position;
layout(location = 1) in vec4 a_color;
layout(location = 2) in vec3 a_normal;

out vec3 v_position;
out vec3 v_color;
out float v_alpha;
out vec3 v_normal;

uniform mat4 u_projection;
uniform mat4 u_view;
uniform vec3 u_translation;
uniform vec3 u_rotation;
uniform vec3 u_scale;

void main() {
    mat4 model = translate(u_translation) * rotate_x(u_rotation.x) * rotate_y(u_rotation.y) * rotate_z(u_rotation.z) * scale(u_scale) ;
    v_position = vec3(model * vec4(a_position, 1.0));
    v_normal = mat3(transpose(inverse(model))) * a_normal;

    gl_Position = u_projection * u_view * vec4(v_position, 1.0);

    v_color = vec3(a_color);
    v_alpha = a_color.w;
}

