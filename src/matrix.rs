use crate::prelude::*;
use float_cmp::approx_eq;

/////////////////////////////////////////////////////////
// M4
/////////////////////////////////////////////////////////
#[derive(Debug, Copy, Clone)]
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

    pub fn transpose(self) -> Self {
        matrix4(
            self.a00, self.a10, self.a20, self.a30,
            self.a01, self.a11, self.a21, self.a31,
            self.a02, self.a12, self.a22, self.a32,
            self.a03, self.a13, self.a23, self.a33,
        )
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use nom::{
        IResult,
        combinator::{map_res, map},
        sequence::{preceded},
        multi::{many1},
        bytes::complete::{take_while1},
    };

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

    fn parse_matrix4(i: &str) -> M4 {
        map(many1(preceded(take_while1(|b| !num_char(b)), num)),
            |v| matrix4(v[0], v[1], v[2], v[3],
                        v[4], v[5], v[6], v[7],
                        v[8], v[9], v[10], v[11],
                        v[12], v[13], v[14], v[15]))(i).unwrap().1
    }

    fn num(i: &str) -> IResult<&str, f32> {
        map_res(take_while1(num_char), |s: &str| s.parse::<f32>())(i)
    }

    fn num_char(c: char) -> bool {
        ('0' <= c && c <= '9') || c == '.' || c == '-'
    }
}
