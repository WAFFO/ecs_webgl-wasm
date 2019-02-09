#version 300 es
precision mediump float;

in vec3 v_position;
in vec3 v_color;
in float v_alpha;
in vec3 v_normal;
in vec3 v_view_pos;

uniform vec3 u_light_pos;
uniform vec3 u_light_color;

out vec4 out_frag_color;

void main() {
    float ambient_strength = 0.1;
    vec3 ambient = v_color * u_light_color * ambient_strength;

    vec3 light_dir = normalize(u_light_pos - v_position);
    vec3 diffuse = max(dot(v_normal, light_dir), 0.0) * u_light_color;

    float specular_strength = 0.5;
    vec3  view_dir = normalize(v_view_pos - v_position);
    vec3  reflect_dir = reflect(-light_dir, v_normal);
    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);
    vec3  specular = specular_strength * spec * u_light_color;

    out_frag_color = vec4((ambient + diffuse + specular) * v_color, v_alpha);
}