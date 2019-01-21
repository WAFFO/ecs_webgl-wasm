#version 300 es
in vec3 a_position;
in vec4 a_color;

uniform mat4 u_projection;
uniform mat4 u_view;
uniform mat4 u_matrix;

out vec4 v_color;

void main() {
  // Multiply the position by the matrix.
  gl_Position = u_projection * u_view * u_matrix * vec4(a_position, 1.0f);

  // Pass the color to the fragment shader.
  v_color = a_color;
}