use crate::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Ray {
    pub origin: T4,
    pub direction: T4,
}

impl std::ops::Mul<Ray> for M4 {
    type Output = Ray;
    fn mul(self, ray: Ray) -> Self::Output {
        let origin = self * ray.origin;
        let direction = self * ray.direction;
        Self::Output { origin, direction }
    }
}

impl Ray {
    pub fn new(origin: T4, direction: T4) -> Self {
        Self { origin, direction }
    }

    pub fn at(self, t: f32) -> T4 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ray_create() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn ray_at() {
        let r = Ray::new(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));
        assert_eq!(r.at(0.0), point(2.0, 3.0, 4.0));
        assert_eq!(r.at(1.0), point(3.0, 3.0, 4.0));
        assert_eq!(r.at(-1.0), point(1.0, 3.0, 4.0));
        assert_eq!(r.at(2.5), point(4.5, 3.0, 4.0));
    }

    #[test]
    fn ray_translate() {
        let r = translation(3.0, 4.0, 5.0)
            * Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        assert_eq!(r.origin, point(4.0, 6.0, 8.0));
        assert_eq!(r.direction, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn ray_scale() {
        let r = scaling(2.0, 3.0, 4.0)
            * Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        assert_eq!(r.origin, point(2.0, 6.0, 12.0));
        assert_eq!(r.direction, vector(0.0, 3.0, 0.0));
    }
}
