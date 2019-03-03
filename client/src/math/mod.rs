
pub mod glm;


type FSize = f32;

#[derive(Debug, Copy, Clone)]
pub struct Vert3 ( pub(self) [FSize; 3] );

#[derive(Debug, Copy, Clone)]
pub struct Vert4 ( pub(self) [FSize; 4] );

#[derive(Debug, Copy, Clone)]
pub struct Mat3 ( pub(self) [FSize; 9] );

#[derive(Debug, Copy, Clone)]
pub struct Mat4 ( pub(self) [FSize; 16] );

impl Vert3 {
    pub fn new(x: FSize, y: FSize, z: FSize) -> Vert3 { Vert3 ( [x, y, z] ) }
    pub fn zero() -> Vert3 { Vert3 ( [0.0, 0.0, 0.0] ) }
    pub fn data(&self) -> [FSize;3] { self.0 }
    pub fn data_ref(&self) -> &[FSize;3] { &self.0 }
    pub fn data_ref_mut(&mut self) -> &mut [FSize;3] { &mut self.0 }
    pub fn x(&self) -> FSize { self[0] }
    pub fn y(&self) -> FSize { self[1] }
    pub fn z(&self) -> FSize { self[2] }
    pub fn dot(&self, other: &Vert3) -> FSize {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }
    pub fn cross(&self, other: &Vert3) -> Vert3 {
        Vert3 ( [
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        ] )
    }
    pub fn mag(&self) -> FSize { ( self[0].powi(2) + self[1].powi(2) + self[2].powi(2) ).sqrt() }
    pub fn length(&self) -> FSize { self.mag() }
    pub fn normalize(&self) -> Vert3 {
        let mag = self.mag();
        if mag != 0.0 {
            *self / mag
        }
        else {
            *self
        }
    }
}

impl Vert4 {
    pub fn new(x: FSize, y: FSize, z: FSize, w: FSize) -> Vert4 { Vert4 ( [x, y, z, w] ) }
    pub fn zero() -> Vert4 { Vert4 ( [0.0, 0.0, 0.0, 0.0] ) }
    pub fn vert4(v: Vert3, f: FSize) -> Vert4 { Vert4 ( [v[0], v[1], v[2], f] ) }
    pub fn data(&self) -> [FSize;4] { self.0 }
    pub fn data_ref(&self) -> &[FSize;4] { &self.0 }
    pub fn data_ref_mut(&mut self) -> &mut [FSize;4] { &mut self.0 }
    pub fn x(&self) -> FSize { self[0] }
    pub fn y(&self) -> FSize { self[1] }
    pub fn z(&self) -> FSize { self[2] }
    pub fn w(&self) -> FSize { self[3] }
    pub fn dot(&self, other: &Vert4) -> FSize {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2] + self[3] * other[3]
    }
    /// note! cross in 4 dimensions doesn't work, 4th component is set to parameter w!
    pub fn cross(&self, other: &Vert4, w: FSize) -> Vert4 {
        Vert4 ( [
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
            w,
        ] )
    }
    pub fn mag(&self) -> FSize { ( self[0].powi(2) + self[1].powi(2) + self[2].powi(2) + self[3].powi(2) ).sqrt() }
    pub fn length(&self) -> FSize { self.mag() }
    pub fn normalize(&self) -> Vert4 {
        let mag = self.mag();
        if mag != 0.0 {
            *self / mag
        }
        else {
            *self
        }
    }
}

impl Mat3 {
    pub fn new(col1: Vert3, col2: Vert3, col3: Vert3) -> Mat3 {
        Mat3 ( [
            col1[0], col1[1], col1[2],
            col2[0], col2[1], col2[2],
            col3[0], col3[1], col3[2],
        ] )
    }
    pub fn zero() -> Mat3 { Mat3([0.0;9]) }
    pub fn identity() -> Mat3 {
        Mat3 ( [
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ] )
    }
    pub fn data(&self) -> [FSize;9] { self.0 }
    pub fn data_ref(&self) -> &[FSize;9] { &self.0 }
    pub fn data_ref_mut(&mut self) -> &mut [FSize;9] { &mut self.0 }
}

impl Mat4 {
    pub fn new(col1: Vert4, col2: Vert4, col3: Vert4, col4: Vert4) -> Mat4 {
        Mat4 ( [
            col1[0], col1[1], col1[2], col1[3],
            col2[0], col2[1], col2[2], col2[3],
            col3[0], col3[1], col3[2], col3[3],
            col4[0], col4[1], col4[2], col4[3],
        ] )
    }
    pub fn zero() -> Mat4 { Mat4([0.0;16]) }
    pub fn data(&self) -> [FSize;16] { self.0 }
    pub fn identity() -> Mat4 {
        Mat4 ( [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ] )
    }
    pub fn data_ref(&self) -> &[FSize;16] { &self.0 }
    pub fn data_ref_mut(&mut self) -> &mut [FSize;16] { &mut self.0 }
}


impl std::ops::Index<usize> for Vert3 {
    type Output = FSize;

    fn index(&self, index: usize) -> &FSize {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Vert3 {
    fn index_mut(&mut self, index: usize) -> &mut FSize {
        &mut self.0[index]
    }
}

impl std::ops::Index<usize> for Vert4 {
    type Output = FSize;

    fn index(&self, index: usize) -> &FSize {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Vert4 {
    fn index_mut(&mut self, index: usize) -> &mut FSize {
        &mut self.0[index]
    }
}

impl std::ops::Index<usize> for Mat3 {
    type Output = FSize;

    fn index(&self, index: usize) -> &FSize {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Mat3 {
    fn index_mut(&mut self, index: usize) -> &mut FSize {
        &mut self.0[index]
    }
}

impl std::ops::Index<usize> for Mat4 {
    type Output = FSize;

    fn index(&self, index: usize) -> &FSize {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut FSize {
        &mut self.0[index]
    }
}

impl std::ops::Index<(usize,usize)> for Mat3 {
    type Output = FSize;

    fn index(&self, index: (usize,usize)) -> &FSize {
        &self.0[index.0 * 3 + index.1]
    }
}

impl std::ops::IndexMut<(usize,usize)> for Mat3 {
    fn index_mut(&mut self, index: (usize,usize)) -> &mut FSize {
        &mut self.0[index.0 * 3 + index.1]
    }
}

impl std::ops::Index<(usize,usize)> for Mat4 {
    type Output = FSize;

    fn index(&self, index: (usize,usize)) -> &FSize {
        &self.0[index.0 * 4 + index.1]
    }
}

impl std::ops::IndexMut<(usize,usize)> for Mat4 {
    fn index_mut(&mut self, index: (usize,usize)) -> &mut FSize {
        &mut self.0[index.0 * 4 + index.1]
    }
}

impl From<Vert3> for Mat3 {
    fn from(f: Vert3) -> Self {
        Mat3 ( [
            f[0],  0.0,  0.0,
             0.0, f[1],  0.0,
             0.0,  0.0, f[2],
        ] )
    }
}

impl From<Vert4> for Mat4 {
    fn from(f: Vert4) -> Self {
        Mat4 ( [
            f[0],  0.0,  0.0,  0.0,
             0.0, f[1],  0.0,  0.0,
             0.0,  0.0, f[2],  0.0,
             0.0,  0.0,  0.0, f[3],
        ] )
    }
}

impl std::ops::Add for Vert3 {
    type Output = Vert3;

    fn add(self, other: Vert3) -> Vert3 {
        Vert3 ( [
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],
        ] )
    }
}

impl std::ops::Add for Vert4 {
    type Output = Vert4;

    fn add(self, other: Vert4) -> Vert4 {
        Vert4 ( [
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],
            self[3] + other[3],
        ] )
    }
}

impl std::ops::AddAssign for Vert3 {
    fn add_assign(&mut self, other: Vert3) {
        *self = Vert3 ( [
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],
        ] )
    }
}

impl std::ops::AddAssign for Vert4 {
    fn add_assign(&mut self, other: Vert4) {
        *self = Vert4 ( [
            self[0] + other[0],
            self[1] + other[1],
            self[2] + other[2],
            self[3] + other[3],
        ] )
    }
}

impl std::ops::Sub for Vert3 {
    type Output = Vert3;

    fn sub(self, other: Vert3) -> Vert3 {
        Vert3 ( [
            self[0] - other[0],
            self[1] - other[1],
            self[2] - other[2],
        ] )
    }
}

impl std::ops::Sub for Vert4 {
    type Output = Vert4;

    fn sub(self, other: Vert4) -> Vert4 {
        Vert4 ( [
            self[0] - other[0],
            self[1] - other[1],
            self[2] - other[2],
            self[3] - other[3],
        ] )
    }
}

impl std::ops::SubAssign for Vert3 {
    fn sub_assign(&mut self, other: Vert3) {
        *self = Vert3 ( [
            self[0] - other[0],
            self[1] - other[1],
            self[2] - other[2],
        ] )
    }
}

impl std::ops::SubAssign for Vert4 {
    fn sub_assign(&mut self, other: Vert4) {
        *self = Vert4 ( [
            self[0] - other[0],
            self[1] - other[1],
            self[2] - other[2],
            self[3] - other[3],
        ] )
    }
}

impl std::ops::Mul<FSize> for Vert3 {
    type Output = Vert3;

    fn mul(self, rhs: FSize) -> Vert3 {
        Vert3 ( [
            self[0] * rhs,
            self[1] * rhs,
            self[2] * rhs,
        ] )
    }
}

impl std::ops::Mul<FSize> for Vert4 {
    type Output = Vert4;

    fn mul(self, rhs: FSize) -> Vert4 {
        Vert4 ( [
            self[0] * rhs,
            self[1] * rhs,
            self[2] * rhs,
            self[3] * rhs,
        ] )
    }
}

impl std::ops::Div<FSize> for Vert3 {
    type Output = Vert3;

    fn div(self, rhs: FSize) -> Vert3 {
        if rhs == 0.0 { panic!("Cannot divide by zero. (Vert3 / 0.0)"); }
        Vert3 ( [
            self[0] / rhs,
            self[1] / rhs,
            self[2] / rhs,
        ] )
    }
}

impl std::ops::Div<FSize> for Vert4 {
    type Output = Vert4;

    fn div(self, rhs: FSize) -> Vert4 {
        if rhs == 0.0 { panic!("Cannot divide by zero. (Vert4 / 0.0)"); }
        Vert4 ( [
            self[0] / rhs,
            self[1] / rhs,
            self[2] / rhs,
            self[3] / rhs,
        ] )
    }
}

impl std::ops::Mul<Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: Mat3) -> Mat3 {
        let m1 = &self;
        let m2 = &rhs;
        Mat3 ( [
            m1[0]*m2[0]+m1[3]*m2[1]+m1[6]*m2[2], m1[0]*m2[3]+m1[3]*m2[4]+m1[6]*m2[5], m1[0]*m2[6]+m1[3]*m2[7]+m1[6]*m2[8],
            m1[1]*m2[0]+m1[4]*m2[1]+m1[7]*m2[2], m1[1]*m2[3]+m1[4]*m2[4]+m1[7]*m2[5], m1[1]*m2[6]+m1[4]*m2[7]+m1[7]*m2[8],
            m1[2]*m2[0]+m1[5]*m2[1]+m1[8]*m2[2], m1[2]*m2[3]+m1[5]*m2[4]+m1[8]*m2[5], m1[2]*m2[6]+m1[5]*m2[7]+m1[8]*m2[8],
        ] )
    }
}

impl std::ops::Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Mat4 {
        let m1 = &self;
        let m2 = &rhs;
        Mat4 ( [
            m1[0]*m2[ 0]+m1[4]*m2[ 1]+m1[ 8]*m2[ 2]+m1[12]*m2[ 3],
            m1[0]*m2[ 4]+m1[4]*m2[ 5]+m1[ 8]*m2[ 6]+m1[12]*m2[ 7],
            m1[0]*m2[ 8]+m1[4]*m2[ 9]+m1[ 8]*m2[10]+m1[12]*m2[11],
            m1[0]*m2[12]+m1[4]*m2[13]+m1[ 8]*m2[14]+m1[12]*m2[15],
            m1[1]*m2[ 0]+m1[5]*m2[ 1]+m1[ 9]*m2[ 2]+m1[13]*m2[ 3],
            m1[1]*m2[ 4]+m1[5]*m2[ 5]+m1[ 9]*m2[ 6]+m1[13]*m2[ 7],
            m1[1]*m2[ 8]+m1[5]*m2[ 9]+m1[ 9]*m2[10]+m1[13]*m2[11],
            m1[1]*m2[12]+m1[5]*m2[13]+m1[ 9]*m2[14]+m1[13]*m2[15],
            m1[2]*m2[ 0]+m1[6]*m2[ 1]+m1[10]*m2[ 2]+m1[14]*m2[ 3],
            m1[2]*m2[ 4]+m1[6]*m2[ 5]+m1[10]*m2[ 6]+m1[14]*m2[ 7],
            m1[2]*m2[ 8]+m1[6]*m2[ 9]+m1[10]*m2[10]+m1[14]*m2[11],
            m1[2]*m2[12]+m1[6]*m2[13]+m1[10]*m2[14]+m1[14]*m2[15],
            m1[3]*m2[ 0]+m1[7]*m2[ 1]+m1[11]*m2[ 2]+m1[15]*m2[ 3],
            m1[3]*m2[ 4]+m1[7]*m2[ 5]+m1[11]*m2[ 6]+m1[15]*m2[ 7],
            m1[3]*m2[ 8]+m1[7]*m2[ 9]+m1[11]*m2[10]+m1[15]*m2[11],
            m1[3]*m2[12]+m1[7]*m2[13]+m1[11]*m2[14]+m1[15]*m2[15],
        ] )
    }
}