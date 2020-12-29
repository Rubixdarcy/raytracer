use float_cmp::approx_eq;

#[derive(Copy, Clone, Debug, Default)]
pub struct T4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl PartialEq for T4 {
    fn eq(&self, other: &Self) -> bool {
        approx_eq!(f32, self.x, other.x, epsilon = 0.00001)
        && approx_eq!(f32, self.y, other.y, epsilon = 0.00001)
        && approx_eq!(f32, self.z, other.z, epsilon = 0.00001)
        && approx_eq!(f32, self.w, other.w, epsilon = 0.00001)
    }
}

impl Eq for T4 {}

impl std::ops::Add for T4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl std::ops::Sub for T4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl std::ops::Mul for T4 {
    type Output = f32;
    fn mul(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

impl std::ops::Mul<f32> for T4 {
    type Output = Self;

    fn mul(self, k: f32) -> Self {
        Self {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
            w: self.w * k,
        }
    }
}

impl std::ops::Mul<T4> for f32 {
    type Output = T4;
    fn mul(self, t: T4) -> Self::Output { t * self }
}

impl std::ops::Div<f32> for T4 {
    type Output = Self;

    fn div(self, k: f32) -> Self {
        Self {
            x: self.x / k,
            y: self.y / k,
            z: self.z / k,
            w: self.w / k,
        }
    }
}

impl std::ops::Neg for T4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        tuple(0.0, 0.0, 0.0, 0.0) - self
    }
}

impl T4 {
    pub fn is_point(self) -> bool {
        approx_eq!(f32, self.w, 1f32, epsilon = 0.00001)
    }

    pub fn is_vector(self) -> bool {
        approx_eq!(f32, self.w, 0f32, epsilon = 0.00001)
    }

    pub fn mag(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
    
    pub fn normalize(self) -> Self {
        let m = self.mag();

        T4 {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
            w: self.w / m,
        }
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(self, other: Self) -> Self {
        vector(self.y * other.z - self.z * other.y,
               self.z * other.x - self.x * other.z,
               self.x * other.y - self.y * other.x)
    }

    pub fn reflect(self, normal: Self) -> Self {
        self - normal * 2.0 * (self * normal)
    }
}

pub fn tuple(x: f32, y: f32, z: f32, w: f32, ) -> T4 {
    T4 { x, y, z, w, }
}

pub fn point(x: f32, y: f32, z: f32,) -> T4 {
    T4 { x, y, z, w: 1.0}
}

pub fn vector(x: f32, y: f32, z: f32,) -> T4 {
    T4 { x, y, z, w: 0.0}
}

#[cfg(test)]
mod test {
    use super::*;
    
    fn appr_eq(l: f32, r: f32) -> bool {
        approx_eq!(f32, l, r, epsilon = 0.00001)
    }

    #[test]
    fn tuple_w_1_is_point() {
        let a = tuple(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);

        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_w_0_is_vector() {
        let a = tuple(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);

        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn point_func() {
        assert_eq!(point(4.0, -4.0, 3.0), tuple(4.0, -4.0, 3.0, 1.0))
    }

    #[test]
    fn vector_func() {
        assert_eq!(vector(4.0, -4.0, 3.0), tuple(4.0, -4.0, 3.0, 0.0))
    }

    #[test]
    fn tuple_add() {
        assert_eq!(tuple(3.0, -2.0, 5.0, 1.0) + tuple(-2.0, 3.0, 1.0, 0.0), tuple(1.0, 1.0, 6.0, 1.0))
    }

    #[test]
    fn sub_point_from_point() {
        assert_eq!(point(3.0, 2.0, 1.0) - point(5.0, 6.0, 7.0), vector(-2.0, -4.0, -6.0))
    }

    #[test]
    fn sub_vector_from_point() {
        assert_eq!(point(3.0, 2.0, 1.0) - vector(5.0, 6.0, 7.0), point(-2.0, -4.0, -6.0))
    }

    #[test]
    fn sub_vector_from_vector() {
        assert_eq!(vector(3.0, 2.0, 1.0) - vector(5.0, 6.0, 7.0), vector(-2.0, -4.0, -6.0))
    }

    #[test]
    fn tuple_negate() {
        assert_eq!(-tuple(1.0, -2.0, 3.0, -4.0), tuple(-1.0, 2.0, -3.0, 4.0))
    }

    #[test]
    fn tuple_mul() {
        assert_eq!(3.5 * tuple(1.0, -2.0, 3.0, -4.0), tuple(3.5, -7.0, 10.5, -14.0))
    }

    #[test]
    fn tuple_mul_fraction() {
        assert_eq!(0.5 * tuple(1.0, -2.0, 3.0, -4.0), tuple(0.5, -1.0, 1.5, -2.0))
    }

    #[test]
    fn tuple_div() {
        assert_eq!(tuple(1.0, -2.0, 3.0, -4.0) / 2.0, tuple(0.5, -1.0, 1.5, -2.0))
    }

    #[test]
    fn vector_mag1() {
        assert!(appr_eq(vector(1.0, 0.0, 0.0).mag(), 1.0))
    }

    #[test]
    fn vector_mag2() {
        assert!(appr_eq(vector(0.0, 1.0, 0.0).mag(), 1.0))
    }

    #[test]
    fn vector_mag3() {
        assert!(appr_eq(vector(0.0, 0.0, 1.0).mag(), 1.0))
    }

    #[test]
    fn vector_mag4() {
        assert!(appr_eq(vector(1.0, 2.0, 3.0).mag(), (14f32).sqrt()))
    }

    #[test]
    fn vector_mag5() {
        assert!(appr_eq(vector(-1.0, -2.0, -3.0).mag(), (14f32).sqrt()))
    }

    #[test]
    fn vector_normalize1() {
        assert_eq!(vector(4.0, 0.0, 0.0).normalize(), vector(1.0, 0.0, 0.0))
    }

    #[test]
    fn vector_normalize2() {
        assert_eq!(vector(1.0, 2.0, 3.0).normalize(), vector(0.26726, 0.53452, 0.80178))
    }

    #[test]
    fn vector_normalize3() {
        assert!(appr_eq(vector(1.0, 2.0, 3.0).normalize().mag(), 1.0))
    }

    #[test]
    fn vector_dot() {
        assert!(appr_eq(vector(1.0, 2.0, 3.0).dot(vector(2.0, 3.0, 4.0)), 20.0))
    }

    #[test]
    fn vector_cross() {
        assert_eq!(vector(1.0, 2.0, 3.0).cross(vector(2.0, 3.0, 4.0)), vector(-1.0, 2.0, -1.0))
    }

    #[test]
    fn vector_reflect() {
        use std::f32::consts::FRAC_1_SQRT_2 as S2O2;
        assert_eq!(vector(1.0, -1.0, 0.0).reflect(vector(0.0, 1.0, 0.0)),
                   vector(1.0, 1.0, 0.0));
        assert_eq!(vector(0.0, -1.0, 0.0).reflect(vector(S2O2, S2O2, 0.0)),
                   vector(1.0, 0.0, 0.0));
    }
}
