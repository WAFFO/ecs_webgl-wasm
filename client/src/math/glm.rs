
use math::{FSize, Vert3, Vert4, Mat4};

pub fn perspective(aspect: FSize, fov: FSize, near: FSize, far: FSize) -> Mat4 {
    let xy_max = near * fov.to_radians();

    let depth = far - near;
    let q = -(far + near) / depth;
    let qn = -2.0 * (far * near) / depth;
    
    let w = 2.0 * near / xy_max;
    let w = w / aspect;
    let h = 2.0 * near / xy_max;

    let mut m = Mat4::zero();

    m[0] =   w;   m[4] = 0.0;   m[8]  =  0.0;   m[12] = 0.0;
    m[1] = 0.0;   m[5] =   h;   m[9]  =  0.0;   m[13] = 0.0;
    m[2] = 0.0;   m[6] = 0.0;   m[10] =    q;   m[14] =  qn;
    m[3] = 0.0;   m[7] = 0.0;   m[11] = -1.0;   m[15] = 0.0;

    m
}

pub fn look_at(pos: Vert3, target: Vert3, up: Vert3) -> Mat4 {
    let zaxis: Vert3 = (pos - target).normalize();
    let xaxis: Vert3 = up.cross(&target).normalize();
    let yaxis: Vert3 = zaxis.cross(&xaxis).normalize();

    Mat4::new(
        Vert4::new(xaxis[0], yaxis[0], zaxis[0], 0.0),
        Vert4::new(yaxis[1], yaxis[1], zaxis[1], 0.0),
        Vert4::new(zaxis[2], yaxis[2], zaxis[2], 0.0),
        Vert4::new(xaxis.dot(&pos) * -1.0, yaxis.dot(&pos) * -1.0, zaxis.dot(&pos) * -1.0, 1.0),
    )
}