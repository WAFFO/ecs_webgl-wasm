#version 300 es

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
        vec4(1.,0.,0.,0.),
        vec4(0.,cos(phi),-sin(phi),0.),
        vec4(0.,sin(phi),cos(phi),0.),
        vec4(0.,0.,0.,1.));
}

mat4 rotate_y(float theta){
    return mat4(
        vec4(cos(theta),0.,-sin(theta),0.),
        vec4(0.,1.,0.,0.),
        vec4(sin(theta),0.,cos(theta),0.),
        vec4(0.,0.,0.,1.));
}

mat4 rotate_z(float psi){
    return mat4(
        vec4(cos(psi),-sin(psi),0.,0.),
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

mat4 look_at(vec3 pos, vec3 target, vec3 up){
  vec3 zaxis = normalize(pos - target);
  vec3 xaxis = normalize(cross(up, zaxis));
  vec3 yaxis = normalize(cross(zaxis, xaxis));

  mat4 viewMatrix = mat4(
    vec4( xaxis.x,  yaxis.x,  zaxis.x, 0),
    vec4( xaxis.y,  yaxis.y,  zaxis.y, 0),
    vec4( xaxis.z,  yaxis.z,  zaxis.z, 0),
    vec4( -dot(xaxis, pos), -dot(yaxis, pos), -dot(zaxis, pos), 1)
  );

  return viewMatrix;
}