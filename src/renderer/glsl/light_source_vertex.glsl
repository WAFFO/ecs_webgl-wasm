#version 300 es
layout(location = 0) in vec3 a_position;

uniform mat4 u_projection;
uniform mat4 u_view;
uniform vec3 u_translation;
uniform vec3 u_rotation;
uniform vec3 u_scale;
uniform vec4 u_color;

out vec4 v_color;

mat4 translate(vec3 t){
    return mat4(
        vec4(1.0, 0.0, 0.0, 0.0),
        vec4(0.0, 1.0, 0.0, 0.0),
        vec4(0.0, 0.0, 1.0, 0.0),
        vec4(t.x, t.y, t.z, 1.0)
    );
}

mat4 rotate_x(float phi){
    return mat4(
        vec4(1.,0.,0.,0),
        vec4(0.,cos(phi),-sin(phi),0.),
        vec4(0.,sin(phi),cos(phi),0.),
        vec4(0.,0.,0.,1.));
}

mat4 rotate_y(float theta){
    return mat4(
        vec4(cos(theta),0.,-sin(theta),0),
        vec4(0.,1.,0.,0.),
        vec4(sin(theta),0.,cos(theta),0.),
        vec4(0.,0.,0.,1.));
}

mat4 rotate_z(float psi){
    return mat4(
        vec4(cos(psi),-sin(psi),0.,0),
        vec4(sin(psi),cos(psi),0.,0.),
        vec4(0.,0.,1.,0.),
        vec4(0.,0.,0.,1.));
}

mat4 scale(vec3 s){
    return mat4(
        vec4(s.x, 0.0, 0.0, 0.0),
        vec4(0.0, s.y, 0.0, 0.0),
        vec4(0.0, 0.0, s.z, 0.0),
        vec4(0.0, 0.0, 0.0, 1.0)
    );
}

void main() {
    mat4 model = translate(u_translation) * rotate_x(u_rotation.x) * rotate_y(u_rotation.y) * rotate_z(u_rotation.z) * scale(u_scale) ;

    gl_Position = u_projection * u_view * model * vec4(a_position, 1.0);

    v_color = u_color;
}