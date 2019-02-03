#version 300 es
precision mediump float;

in vec3 v_position;
in vec3 v_color;
in float v_alpha;
in vec3 v_normal;

uniform vec3 u_light_pos;
uniform vec3 u_light_color;
//uniform vec3 u_view_pos;

out vec4 out_frag_color;

void main() {
    float ambient_strength = 0.1;
    vec3 ambient = v_color * u_light_color * ambient_strength;

    vec3 light_dir = normalize(u_light_pos - v_position);
    vec3 diffuse = max(dot(v_normal, light_dir), 0.0) * u_light_color;

//    float specular_strength = 0.5;
//    vec3 view_dir = normalize(u_view_pos - v_position);

    out_frag_color = vec4((ambient + diffuse) * v_color, v_alpha);
}