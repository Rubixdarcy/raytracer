use crate::prelude::*;
use float_cmp::approx_eq;

/////////////////////////////////////////////////////////
// M2
/////////////////////////////////////////////////////////
#[derive(Debug, Copy, Clone, Default)]
pub struct M2 {
    pub a00: f32, pub a01: f32,
    pub a10: f32, pub a11: f32,
}

pub fn matrix2(a00: f32, a01: f32,
               a10: f32, a11: f32,) -> M2 {
    M2 { a00, a01,
         a10, a11, }
}

impl PartialEq for M2 {
    fn eq(&self, other: &Self) -> bool {
           approx_eq!(f32, self.a00, other.a00, epsilon = 0.00001)
        && approx_eq!(f32, self.a01, other.a01, epsilon = 0.00001)
        && approx_eq!(f32, self.a10, other.a10, epsilon = 0.00001)
        && approx_eq!(f32, self.a11, other.a11, epsilon = 0.00001)
    }
}

impl Eq for M2 {}

impl std::ops::Index<(usize, usize)> for M2 {
    type Output = f32;
    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        match idx {
            (0, 0) => &self.a00,
            (0, 1) => &self.a01,
            (1, 0) => &self.a10,
            (1, 1) => &self.a11,
            _      => panic!("Invalid index for M2: {:?}", idx),
        }
    }
}

impl std::ops::IndexMut<(usize, usize)> for M2 {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        match idx {
            (0, 0) => &mut self.a00,
            (0, 1) => &mut self.a01,
            (1, 0) => &mut self.a10,
            (1, 1) => &mut self.a11,
            _      => panic!("Invalid index for M2: {:?}", idx),
        }
    }
}

impl std::ops::Add for M2 {
    type Output = Self;

    fn add(self, n: Self) -> Self {
        let m = self;
        Self {
            a00: m.a00 + n.a00, a01: m.a01 + n.a01,
            a10: m.a10 + n.a10, a11: m.a11 + n.a11,
        }
    }
}

impl std::ops::Sub for M2 {
    type Output = Self;

    fn sub(self, n: Self) -> Self {
        let m = self;
        Self {
            a00: m.a00 - n.a00, a01: m.a01 - n.a01,
            a10: m.a10 - n.a10, a11: m.a11 - n.a11,
        }
    }
}

impl std::ops::Neg for M2 {
    type Output = Self;

    fn neg(self) -> Self {
        M2::ZERO - self
    }
}

impl std::ops::Mul for M2 {
    type Output = Self;

    fn mul(self, n: Self) -> Self {
        let m = self;
        Self {
            a00: m.a00 * n.a00 + m.a01 * n.a10,
            a01: m.a00 * n.a01 + m.a01 * n.a11,
            a10: m.a10 * n.a00 + m.a11 * n.a10,
            a11: m.a10 * n.a01 + m.a11 * n.a11,
        }
    }
}

// <no Mul<T4> implementation>

impl std::ops::Mul<f32> for M2 {
    type Output = Self;

    fn mul(self, k: f32) -> Self {
        let m = self;
        Self {
            a00: m.a00 * k, a01: m.a01 * k,
            a10: m.a10 * k, a11: m.a11 * k,
        }
    }
}

impl std::ops::Mul<M2> for f32 {
    
    type Output = M2; 

    fn mul(self, m: Self::Output) -> Self::Output {
        m * self
    }
}


impl M2 {
    pub const ZERO: Self =
        Self { a00: 0.0, a01: 0.0,
               a10: 0.0, a11: 0.0, };

    pub const IDENTITY: Self =
        Self { a00: 1.0, a01: 0.0,
               a10: 0.0, a11: 1.0, };

    pub const SIZE: usize = 2;

    pub fn transpose(self) -> Self {
        matrix2(
            self.a00, self.a10,
            self.a01, self.a11,
        )
    }

    pub fn determinant(self) -> f32 {
        self.a00 * self.a11 - self.a10 * self.a01
    }
}

/////////////////////////////////////////////////////////
// M3
/////////////////////////////////////////////////////////
#[derive(Debug, Copy, Clone, Default)]
pub struct M3 {
    pub a00: f32, pub a01: f32, pub a02: f32,
    pub a10: f32, pub a11: f32, pub a12: f32,
    pub a20: f32, pub a21: f32, pub a22: f32,
}

pub fn matrix3(a00: f32, a01: f32, a02: f32,
               a10: f32, a11: f32, a12: f32,
               a20: f32, a21: f32, a22: f32,) -> M3 {
    M3 { a00, a01, a02,
         a10, a11, a12,
         a20, a21, a22, }
}

impl PartialEq for M3 {
    fn eq(&self, other: &Self) -> bool {
           approx_eq!(f32, self.a00, other.a00, epsilon = 0.00001)
        && approx_eq!(f32, self.a01, other.a01, epsilon = 0.00001)
        && approx_eq!(f32, self.a02, other.a02, epsilon = 0.00001)
        && approx_eq!(f32, self.a10, other.a10, epsilon = 0.00001)
        && approx_eq!(f32, self.a11, other.a11, epsilon = 0.00001)
        && approx_eq!(f32, self.a12, other.a12, epsilon = 0.00001)
        && approx_eq!(f32, self.a20, other.a20, epsilon = 0.00001)
        && approx_eq!(f32, self.a21, other.a21, epsilon = 0.00001)
        && approx_eq!(f32, self.a22, other.a22, epsilon = 0.00001)
    }
}

impl Eq for M3 {}

impl std::ops::Index<(usize, usize)> for M3 {
    type Output = f32;
    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        match idx {
            (0, 0) => &self.a00,
            (0, 1) => &self.a01,
            (0, 2) => &self.a02,
            (1, 0) => &self.a10,
            (1, 1) => &self.a11,
            (1, 2) => &self.a12,
            (2, 0) => &self.a20,
            (2, 1) => &self.a21,
            (2, 2) => &self.a22,
            _      => panic!("Invalid index for M3: {:?}", idx),
        }
    }
}

impl std::ops::IndexMut<(usize, usize)> for M3 {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        match idx {
            (0, 0) => &mut self.a00,
            (0, 1) => &mut self.a01,
            (0, 2) => &mut self.a02,
            (1, 0) => &mut self.a10,
            (1, 1) => &mut self.a11,
            (1, 2) => &mut self.a12,
            (2, 0) => &mut self.a20,
            (2, 1) => &mut self.a21,
            (2, 2) => &mut self.a22,
            _      => panic!("Invalid index for M3: {:?}", idx),
        }
    }
}

impl std::ops::Add for M3 {
    type Output = Self;

    fn add(self, n: Self) -> Self {
        let m = self;
        M3 {
            a00: m.a00 + n.a00, a01: m.a01 + n.a01, a02: m.a02 + n.a02,
            a10: m.a10 + n.a10, a11: m.a11 + n.a11, a12: m.a12 + n.a12,
            a20: m.a20 + n.a20, a21: m.a21 + n.a21, a22: m.a22 + n.a22,
        }
    }
}

impl std::ops::Sub for M3 {
    type Output = Self;

    fn sub(self, n: Self) -> Self {
        let m = self;
        Self {
            a00: m.a00 - n.a00, a01: m.a01 - n.a01, a02: m.a02 - n.a02,
            a10: m.a10 - n.a10, a11: m.a11 - n.a11, a12: m.a12 - n.a12,
            a20: m.a20 - n.a20, a21: m.a21 - n.a21, a22: m.a22 - n.a22,
        }
    }
}

impl std::ops::Neg for M3 {
    type Output = Self;

    fn neg(self) -> Self {
        M3::ZERO - self
    }
}

impl std::ops::Mul for M3 {
    type Output = Self;

    fn mul(self, n: Self) -> Self {
        let m = self;
        Self {
            a00: m.a00 * n.a00 + m.a01 * n.a10 + m.a02 * n.a20,
            a01: m.a00 * n.a01 + m.a01 * n.a11 + m.a02 * n.a21,
            a02: m.a00 * n.a02 + m.a01 * n.a12 + m.a02 * n.a22,
            a10: m.a10 * n.a00 + m.a11 * n.a10 + m.a12 * n.a20,
            a11: m.a10 * n.a01 + m.a11 * n.a11 + m.a12 * n.a21,
            a12: m.a10 * n.a02 + m.a11 * n.a12 + m.a12 * n.a22,
            a20: m.a20 * n.a00 + m.a21 * n.a10 + m.a22 * n.a20,
            a21: m.a20 * n.a01 + m.a21 * n.a11 + m.a22 * n.a21,
            a22: m.a20 * n.a02 + m.a21 * n.a12 + m.a22 * n.a22,
        }
    }
}

// <no Mul<T4> implementation>

impl std::ops::Mul<f32> for M3 {
    type Output = Self;

    fn mul(self, k: f32) -> Self {
        let m = self;
        M3 {
            a00: m.a00 * k, a01: m.a01 * k, a02: m.a02 * k,
            a10: m.a10 * k, a11: m.a11 * k, a12: m.a12 * k,
            a20: m.a20 * k, a21: m.a21 * k, a22: m.a22 * k,
        }
    }
}

impl std::ops::Mul<M3> for f32 {
    
    type Output = M3; 

    fn mul(self, m: Self::Output) -> Self::Output {
        m * self
    }
}


impl M3 {
    pub const ZERO: Self =
        Self { a00: 0.0, a01: 0.0, a02: 0.0,
               a10: 0.0, a11: 0.0, a12: 0.0,
               a20: 0.0, a21: 0.0, a22: 0.0, };

    pub const IDENTITY: Self =
        Self { a00: 1.0, a01: 0.0, a02: 0.0,
               a10: 0.0, a11: 1.0, a12: 0.0,
               a20: 0.0, a21: 0.0, a22: 1.0, };

    pub const SIZE: usize = 3;

    pub fn transpose(self) -> Self {
        matrix3(
            self.a00, self.a10, self.a20,
            self.a01, self.a11, self.a21,
            self.a02, self.a12, self.a22,
        )
    }

    pub fn submatrix(self, row: usize, column: usize) -> M2 {
        let mut result = M2::default();
        for i in 0..M2::SIZE {
            for j in 0..M2::SIZE {
                let src_i = if i < row { i } else { i + 1};
                let src_j = if j < column { j } else { j + 1};
                result[(i, j)] = self[(src_i, src_j)];
            }
        }
        return result;
    }

    pub fn minor(self, row: usize, column: usize) -> f32 {
        self.submatrix(row, column).determinant()
    }

    pub fn cofactor(self, row: usize, column: usize) -> f32 {
        match (row + column) % 2 {
            0 => self.minor(row, column),
            _ => -self.minor(row, column),
        }
    }

    pub fn determinant(self) -> f32 {
        self.a00 * self.cofactor(0, 0)
        + self.a01 * self.cofactor(0, 1)
        + self.a02 * self.cofactor(0, 2)
    }
}

/////////////////////////////////////////////////////////
// M4
/////////////////////////////////////////////////////////
#[derive(Debug, Copy, Clone, Default)]
pub struct M4 {
    pub a00: f32, pub a01: f32, pub a02: f32, pub a03: f32,
    pub a10: f32, pub a11: f32, pub a12: f32, pub a13: f32,
    pub a20: f32, pub a21: f32, pub a22: f32, pub a23: f32,
    pub a30: f32, pub a31: f32, pub a32: f32, pub a33: f32,
}

pub fn matrix4(a00: f32, a01: f32, a02: f32, a03: f32,
               a10: f32, a11: f32, a12: f32, a13: f32,
               a20: f32, a21: f32, a22: f32, a23: f32,
               a30: f32, a31: f32, a32: f32, a33: f32) -> M4 {
    M4 { a00, a01, a02, a03,
         a10, a11, a12, a13,
         a20, a21, a22, a23,
         a30, a31, a32, a33 }
}

impl PartialEq for M4 {
    fn eq(&self, other: &Self) -> bool {
           approx_eq!(f32, self.a00, other.a00, epsilon = 0.00001)
        && approx_eq!(f32, self.a01, other.a01, epsilon = 0.00001)
        && approx_eq!(f32, self.a02, other.a02, epsilon = 0.00001)
        && approx_eq!(f32, self.a03, other.a03, epsilon = 0.00001)
        && approx_eq!(f32, self.a10, other.a10, epsilon = 0.00001)
        && approx_eq!(f32, self.a11, other.a11, epsilon = 0.00001)
        && approx_eq!(f32, self.a12, other.a12, epsilon = 0.00001)
        && approx_eq!(f32, self.a13, other.a13, epsilon = 0.00001)
        && approx_eq!(f32, self.a20, other.a20, epsilon = 0.00001)
        && approx_eq!(f32, self.a21, other.a21, epsilon = 0.00001)
        && approx_eq!(f32, self.a22, other.a22, epsilon = 0.00001)
        && approx_eq!(f32, self.a23, other.a23, epsilon = 0.00001)
        && approx_eq!(f32, self.a30, other.a30, epsilon = 0.00001)
        && approx_eq!(f32, self.a31, other.a31, epsilon = 0.00001)
        && approx_eq!(f32, self.a32, other.a32, epsilon = 0.00001)
        && approx_eq!(f32, self.a33, other.a33, epsilon = 0.00001)
    }
}

impl Eq for M4 {}

impl std::ops::Index<(usize, usize)> for M4 {
    type Output = f32;
    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        match idx {
            (0, 0) => &self.a00,
            (0, 1) => &self.a01,
            (0, 2) => &self.a02,
            (0, 3) => &self.a03,
            (1, 0) => &self.a10,
            (1, 1) => &self.a11,
            (1, 2) => &self.a12,
            (1, 3) => &self.a13,
            (2, 0) => &self.a20,
            (2, 1) => &self.a21,
            (2, 2) => &self.a22,
            (2, 3) => &self.a23,
            (3, 0) => &self.a30,
            (3, 1) => &self.a31,
            (3, 2) => &self.a32,
            (3, 3) => &self.a33,
            _      => panic!("Invalid index for M4: {:?}", idx),
        }
    }
}

impl std::ops::IndexMut<(usize, usize)> for M4 {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        match idx {
            (0, 0) => &mut self.a00,
            (0, 1) => &mut self.a01,
            (0, 2) => &mut self.a02,
            (0, 3) => &mut self.a03,
            (1, 0) => &mut self.a10,
            (1, 1) => &mut self.a11,
            (1, 2) => &mut self.a12,
            (1, 3) => &mut self.a13,
            (2, 0) => &mut self.a20,
            (2, 1) => &mut self.a21,
            (2, 2) => &mut self.a22,
            (2, 3) => &mut self.a23,
            (3, 0) => &mut self.a30,
            (3, 1) => &mut self.a31,
            (3, 2) => &mut self.a32,
            (3, 3) => &mut self.a33,
            _      => panic!("Invalid index for M4: {:?}", idx),
        }
    }
}

impl std::ops::Add for M4 {
    type Output = Self;

    fn add(self, n: Self) -> Self {
        let m = self;
        M4 {
            a00: m.a00 + n.a00, a01: m.a01 + n.a01, a02: m.a02 + n.a02, a03: m.a03 + n.a03,
            a10: m.a10 + n.a10, a11: m.a11 + n.a11, a12: m.a12 + n.a12, a13: m.a13 + n.a13,
            a20: m.a20 + n.a20, a21: m.a21 + n.a21, a22: m.a22 + n.a22, a23: m.a23 + n.a23,
            a30: m.a30 + n.a30, a31: m.a31 + n.a31, a32: m.a32 + n.a32, a33: m.a33 + n.a33,
        }
    }
}

impl std::ops::Sub for M4 {
    type Output = Self;

    fn sub(self, n: Self) -> Self {
        let m = self;
        M4 {
            a00: m.a00 - n.a00, a01: m.a01 - n.a01, a02: m.a02 - n.a02, a03: m.a03 - n.a03,
            a10: m.a10 - n.a10, a11: m.a11 - n.a11, a12: m.a12 - n.a12, a13: m.a13 - n.a13,
            a20: m.a20 - n.a20, a21: m.a21 - n.a21, a22: m.a22 - n.a22, a23: m.a23 - n.a23,
            a30: m.a30 - n.a30, a31: m.a31 - n.a31, a32: m.a32 - n.a32, a33: m.a33 - n.a33,
        }
    }
}

impl std::ops::Neg for M4 {
    type Output = Self;

    fn neg(self) -> Self {
        M4::ZERO - self
    }
}

impl std::ops::Mul for M4 {
    type Output = Self;

    fn mul(self, n: Self) -> Self {
        let m = self;
        M4 {
            a00: m.a00 * n.a00 + m.a01 * n.a10 + m.a02 * n.a20 + m.a03 * n.a30,
            a01: m.a00 * n.a01 + m.a01 * n.a11 + m.a02 * n.a21 + m.a03 * n.a31,
            a02: m.a00 * n.a02 + m.a01 * n.a12 + m.a02 * n.a22 + m.a03 * n.a32,
            a03: m.a00 * n.a03 + m.a01 * n.a13 + m.a02 * n.a23 + m.a03 * n.a33,
            a10: m.a10 * n.a00 + m.a11 * n.a10 + m.a12 * n.a20 + m.a13 * n.a30,
            a11: m.a10 * n.a01 + m.a11 * n.a11 + m.a12 * n.a21 + m.a13 * n.a31,
            a12: m.a10 * n.a02 + m.a11 * n.a12 + m.a12 * n.a22 + m.a13 * n.a32,
            a13: m.a10 * n.a03 + m.a11 * n.a13 + m.a12 * n.a23 + m.a13 * n.a33,
            a20: m.a20 * n.a00 + m.a21 * n.a10 + m.a22 * n.a20 + m.a23 * n.a30,
            a21: m.a20 * n.a01 + m.a21 * n.a11 + m.a22 * n.a21 + m.a23 * n.a31,
            a22: m.a20 * n.a02 + m.a21 * n.a12 + m.a22 * n.a22 + m.a23 * n.a32,
            a23: m.a20 * n.a03 + m.a21 * n.a13 + m.a22 * n.a23 + m.a23 * n.a33,
            a30: m.a30 * n.a00 + m.a31 * n.a10 + m.a32 * n.a20 + m.a33 * n.a30,
            a31: m.a30 * n.a01 + m.a31 * n.a11 + m.a32 * n.a21 + m.a33 * n.a31,
            a32: m.a30 * n.a02 + m.a31 * n.a12 + m.a32 * n.a22 + m.a33 * n.a32,
            a33: m.a30 * n.a03 + m.a31 * n.a13 + m.a32 * n.a23 + m.a33 * n.a33,
        }
    }
}

impl std::ops::Mul<T4> for M4 {
    type Output = T4;
    fn mul(self, t: T4) -> Self::Output {
        let m = self;
        T4 {
            x: m.a00 * t.x + m.a01 * t.y + m.a02 * t.z + m.a03 * t.w,
            y: m.a10 * t.x + m.a11 * t.y + m.a12 * t.z + m.a13 * t.w,
            z: m.a20 * t.x + m.a21 * t.y + m.a22 * t.z + m.a23 * t.w,
            w: m.a30 * t.x + m.a31 * t.y + m.a32 * t.z + m.a33 * t.w,
        }
    }
}

impl std::ops::Mul<f32> for M4 {
    type Output = Self;

    fn mul(self, k: f32) -> Self {
        let m = self;
        M4 {
            a00: m.a00 * k, a01: m.a01 * k, a02: m.a02 * k, a03: m.a03 * k,
            a10: m.a10 * k, a11: m.a11 * k, a12: m.a12 * k, a13: m.a13 * k,
            a20: m.a20 * k, a21: m.a21 * k, a22: m.a22 * k, a23: m.a23 * k,
            a30: m.a30 * k, a31: m.a31 * k, a32: m.a32 * k, a33: m.a33 * k,
        }
    }
}

impl std::ops::Mul<M4> for f32 {
    type Output = M4;

    fn mul(self, m: M4) -> M4 {
        m * self
    }
}


impl M4 {
    pub const ZERO: Self =
        Self { a00: 0.0, a01: 0.0, a02: 0.0, a03: 0.0,
               a10: 0.0, a11: 0.0, a12: 0.0, a13: 0.0,
               a20: 0.0, a21: 0.0, a22: 0.0, a23: 0.0,
               a30: 0.0, a31: 0.0, a32: 0.0, a33: 0.0 };

    pub const IDENTITY: Self =
        Self { a00: 1.0, a01: 0.0, a02: 0.0, a03: 0.0,
               a10: 0.0, a11: 1.0, a12: 0.0, a13: 0.0,
               a20: 0.0, a21: 0.0, a22: 1.0, a23: 0.0,
               a30: 0.0, a31: 0.0, a32: 0.0, a33: 1.0 };

    pub const SIZE: usize = 4;

    pub fn transpose(self) -> Self {
        matrix4(
            self.a00, self.a10, self.a20, self.a30,
            self.a01, self.a11, self.a21, self.a31,
            self.a02, self.a12, self.a22, self.a32,
            self.a03, self.a13, self.a23, self.a33,
        )
    }

    pub fn submatrix(self, row: usize, column: usize) -> M3 {
        let mut result = M3::default();
        for i in 0..M3::SIZE {
            for j in 0..M3::SIZE {
                let src_i = if i < row { i } else { i + 1};
                let src_j = if j < column { j } else { j + 1};
                result[(i, j)] = self[(src_i, src_j)];
            }
        }
        return result;
    }

    pub fn minor(self, row: usize, column: usize) -> f32 {
        self.submatrix(row, column).determinant()
    }

    pub fn cofactor(self, row: usize, column: usize) -> f32 {
        match (row + column) % 2 {
            0 => self.minor(row, column),
            _ => -self.minor(row, column),
        }
    }

    pub fn determinant(self) -> f32 {
        self.a00 * self.cofactor(0, 0)
        + self.a01 * self.cofactor(0, 1)
        + self.a02 * self.cofactor(0, 2)
        + self.a03 * self.cofactor(0, 3)
    }

    pub fn invertible(self) -> bool {
        !approx_eq!(f32, self.determinant(), 0.0, epsilon = 0.00001)
    }

    pub fn inverse(self) -> Self {
        let det = self.determinant();
        if approx_eq!(f32, self.determinant(), 0.0, epsilon = 0.00001) {
            panic!("Attempt to compute inverse of non-invertible matrix: {:?}", self);
        }

        let mut result = Self::default();
        for row in 0..Self::SIZE {
            for col in 0..Self::SIZE {
                result[(col, row)] = self.cofactor(row, col) / det;
            }
        }
        return result;
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::test_prelude::*;

    #[test]
    fn matrix_eq_identical() {
        let m1 = parse_matrix4("
            | 1 | 2 | 3 | 4 |
            | 5 | 6 | 7 | 8 |
            | 9 | 8 | 7 | 6 |
            | 5 | 4 | 3 | 2 |
        ");
        let m2 = parse_matrix4("
            | 1 | 2 | 3 | 4 |
            | 5 | 6 | 7 | 8 |
            | 9 | 8 | 7 | 6 |
            | 5 | 4 | 3 | 2 |
        ");
        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix_eq_different() {
        let m1 = parse_matrix4("
            | 1 | 2 | 3 | 4 |
            | 5 | 6 | 7 | 8 |
            | 9 | 8 | 7 | 6 |
            | 5 | 4 | 3 | 2 |
        ");
        let m2 = parse_matrix4("
            | 2 | 3 | 4 | 5 |
            | 6 | 7 | 8 | 9 |
            | 8 | 7 | 6 | 5 |
            | 4 | 3 | 2 | 1 |
        ");
        assert!(m1 != m2);
    }

    #[test]
    fn matrix_add_and_scale() {
        let m1 = parse_matrix4("
            | 1 | 2 | 3 | 4 |
            | 5 | 6 | 7 | 8 |
            | 9 | 8 | 7 | 6 |
            | 5 | 4 | 3 | 2 |
        ");
        let m2 = m1 * (-1.0);

        assert_eq!(m1 + m2, M4::ZERO);
    }

    #[test]
    fn matrix_matrix_mul() {
        let m1 = parse_matrix4("
            | 1 | 2 | 3 | 4 |
            | 5 | 6 | 7 | 8 |
            | 9 | 8 | 7 | 6 |
            | 5 | 4 | 3 | 2 |
        ");
        let m2 = parse_matrix4("
            | -2 | 1 | 2 |  3 |
            |  3 | 2 | 1 | -1 |
            |  4 | 3 | 6 |  5 |
            |  1 | 2 | 7 |  8 |
        ");
        let m3 = parse_matrix4("
            | 20|  22 |  50 |  48 |
            | 44|  54 | 114 | 108 |
            | 40|  58 | 110 | 102 |
            | 16|  26 |  46 |  42 |
        ");
        assert_eq!(m1 * m2, m3);
    }

    #[test]
    fn matrix_tuple_mul() {
        let m1 = parse_matrix4("
            | 1 | 2 | 3 | 4 |
            | 2 | 4 | 4 | 2 |
            | 8 | 6 | 4 | 1 |
            | 0 | 0 | 0 | 1 |
        ");
        let t = tuple(1.0, 2.0, 3.0, 1.0);
        assert_eq!(m1 * t, tuple(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn matrix_identity_mul() {
        let m1 = parse_matrix4("
            | 0 | 1 |  2 |  4 |
            | 1 | 2 |  4 |  8 |
            | 2 | 4 |  8 | 16 |
            | 4 | 8 | 16 | 32 |
        ");
        assert_eq!(m1 * M4::IDENTITY, m1);
    }

    #[test]
    fn matrix_identity_tuple_mul() {
        let t = tuple(1.0, 2.0, 3.0, 4.0);
        assert_eq!(M4::IDENTITY * t, t);
    }

    #[test]
    fn matrix_transpose() {
        let m1 = parse_matrix4("
            | 0 | 9 | 3 | 0 |
            | 9 | 8 | 0 | 8 |
            | 1 | 8 | 5 | 3 |
            | 0 | 0 | 5 | 8 |
        ");
        let m2 = parse_matrix4("
            | 0 | 9 | 1 | 0 |
            | 9 | 8 | 8 | 0 |
            | 3 | 0 | 5 | 5 |
            | 0 | 8 | 3 | 8 |
        ");
        assert_eq!(m1.transpose(), m2);
    }

    #[test]
    fn matrix_identity_transpose() {
        assert_eq!(M4::IDENTITY.transpose(), M4::IDENTITY.transpose())
    }

    #[test]
    fn matrix2_determinant() {
        assert_eq!(matrix2(1.0, 5.0, -3.0, 2.0).determinant(), 17.0);
    }

    #[test]
    fn matrix3_determinant() {
        let m = matrix3(1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0);
        assert_eq!(m.cofactor(0, 0), 56.0);
        assert_eq!(m.cofactor(0, 1), 12.0);
        assert_eq!(m.cofactor(0, 2), -46.0);
        assert_eq!(m.determinant(), -196.0);
    }

    #[test]
    fn matrix4_determinant() {
        let m = parse_matrix4("
            | -2 | -8 |  3 |  5 |
            | -3 |  1 |  7 |  3 |
            |  1 |  2 | -9 |  6 |
            | -6 |  7 |  7 | -9 |
        ");
        assert_eq!(m.cofactor(0, 0), 690.0);
        assert_eq!(m.cofactor(0, 1), 447.0);
        assert_eq!(m.cofactor(0, 2), 210.0);
        assert_eq!(m.cofactor(0, 3), 51.0);
        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn matrix3_submatrix() {
        assert_eq!(matrix3(1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0)
                       .submatrix(0, 2),
                   matrix2(-3.0, 2.0, 0.0, 6.0));
    }

    #[test]
    fn matrix3_minor() {
        let m = matrix3(3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0);
        assert_eq!(m.submatrix(1, 0).determinant(), 25.0);
        assert_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn matrix3_cofactor() {
        let m = matrix3(3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0);
        assert_eq!(m.minor(0, 0), -12.0);
        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }
    
    #[test]
    fn matrix4_invertible() {
        let m = parse_matrix4("
            |  6 |  4 |  4 |  4 |
            |  5 |  5 |  7 |  6 |
            |  4 | -9 |  3 | -7 |
            |  9 |  1 |  7 | -6 |
        ");
        assert_eq!(m.determinant(), -2120.0);
        assert!(m.invertible());
    }

    #[test]
    fn matrix4_invertible2() {
        let m = parse_matrix4("
            | -4 |  2 | -2 | -3 |
            |  9 |  6 |  2 |  6 |
            |  0 | -5 |  1 | -5 |
            |  0 |  0 |  0 |  0 |
        ");
        assert_eq!(m.determinant(), 0.0);
        assert!(!m.invertible());
    }

    #[test]
    fn matrix4_inverse() {
        let a = parse_matrix4("
             | -5 |  2 |  6 | -8 |
             |  1 | -5 |  1 |  8 |
             |  7 |  7 | -6 | -7 |
             |  1 | -3 |  7 |  4 |
        ");
        let b = a.inverse();

        assert_eq!(a.determinant() ,532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert_eq!(b[(3, 2)], -160.0 / 532.0);
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert_eq!(b[(2, 3)], 105.0 / 532.0);

        assert_eq!(b, parse_matrix4("
            |  0.21805 |  0.45113 |  0.24060 | -0.04511 |
            | -0.80827 | -1.45677 | -0.44361 |  0.52068 |
            | -0.07895 | -0.22368 | -0.05263 |  0.19737 |
            | -0.52256 | -0.81391 | -0.30075 |  0.30639 |
        "));
    }

    #[test]
    fn matrix4_inverse2() {
        let a = parse_matrix4("
            |  8 | -5 |  9 |  2 |
            |  7 |  5 |  6 |  1 |
            | -6 |  0 |  9 |  6 |
            | -3 |  0 | -9 | -4 |
        ");
        let b = a.inverse();

        assert_eq!(b, parse_matrix4("
            | -0.15385 | -0.15385 | -0.28205 | -0.53846 |
            | -0.07692 |  0.12308 |  0.02564 |  0.03077 |
            |  0.35897 |  0.35897 |  0.43590 |  0.92308 |
            | -0.69231 | -0.69231 | -0.76923 | -1.92308 |
        "));
    }

    #[test]
    fn matrix4_inverse3() {
        let a = parse_matrix4("
            |  9 |  3 |  0 |  9 |
            | -5 | -2 | -6 | -3 |
            | -4 |  9 |  6 |  4 |
            | -7 |  6 |  6 |  2 |
        ");
        let b = a.inverse();

        assert_eq!(b, parse_matrix4("
            | -0.04074 | -0.07778 |  0.14444 | -0.22222 |
            | -0.07778 |  0.03333 |  0.36667 | -0.33333 |
            | -0.02901 | -0.14630 | -0.10926 |  0.12963 |
            |  0.17778 |  0.06667 | -0.26667 |  0.33333 |
        "));
    }

    #[test]
    fn matrix4_inverse_product() {
        let a = parse_matrix4("
            |  3 | -9 |  7 |  3 |
            |  3 | -8 |  2 | -9 |
            | -4 |  4 |  4 |  1 |
            | -6 |  5 | -1 |  1 |
        ");
        let b = parse_matrix4("
            |  8 |  2 |  2 |  2 |
            |  3 | -1 |  7 |  0 |
            |  7 |  0 |  5 |  4 |
            |  6 | -2 |  0 |  5 |
        ");
        let c = a * b;
        let d = c * b.inverse();

        assert_eq!(d, a);
    }
}
