use crate::prelude::*;
use std::iter;

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Sphere {
    pub transform: M4,
    pub material: Material,
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere { transform: M4::IDENTITY,
                 material: Material::default() }
    }
}

impl Sphere {
    pub fn new(transform: M4, material: Material) -> Self {
        Self { transform, material }
    }

    pub fn from_transform(transform: M4) -> Self {
        Self { transform, ..Self::default() }
    }

    pub fn intersect<'a>(&'a self, ray: Ray, is: &mut Intersections<'a>) {
        let ray = self.transform.inverse() * ray;
        let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0);

        let a = ray.direction * ray.direction;
        let b = 2.0 * (ray.direction * sphere_to_ray);
        let c = sphere_to_ray * sphere_to_ray - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 { return; }

        let sqrt = discriminant.sqrt();

        is.extend([
            Intersection::new((-b - sqrt) / (2.0 * a), self),
            Intersection::new((-b + sqrt) / (2.0 * a), self),
        ].iter().copied());
    }

    pub fn normal_at(&self, p: T4) -> T4 {
        let inv = self.transform.inverse();
        let object_point = inv * p;
        let object_normal = object_point - point(0.0, 0.0, 0.0);

        // Technically we should find the world normal by finding
        // the transpose inverse of a 3x3 matrix, but instead we
        // use the 4x4 and then manually set w to 0.
        let mut world_normal = inv.transpose() * object_normal;
        world_normal.w = 0.0;

        return world_normal.normalize();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ray_intersection_two_points() {
        let s = Sphere::default();
        let mut xs = Intersections::empty();
        s.intersect(
            Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0)), &mut xs);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersection_tangent() {
        let s = Sphere::default();
        let mut xs = Intersections::empty();
        s.intersect(
            Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0)), &mut xs);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_intersection_miss() {
        let s = Sphere::default();
        let mut xs = Intersections::empty();
        s.intersect(
            Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0)), &mut xs);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_intersection_origin_inside() {
        let s = Sphere::default();
        let mut xs = Intersections::empty();
        s.intersect(
            Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0)), &mut xs);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn ray_intersection_sphere_behind() {
        let s = Sphere::default();
        let mut xs = Intersections::empty();
        s.intersect(
            Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0)), &mut xs);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersection_sets_object() {
        let s = Sphere::default();
        let mut xs = Intersections::empty();
        s.intersect(
            Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0)), &mut xs);
        assert_eq!(xs.len(), 2);
        assert_eq!(*xs[0].object, s);
        assert_eq!(*xs[1].object, s);
    }

    #[test]
    fn sphere_default_transform() {
        let s = Sphere::default();
        assert_eq!(s.transform, M4::IDENTITY);
    }

    #[test]
    fn sphere_custom_transform() {
        let t = translation(2.0, 3.0, 4.0);
        let s = Sphere::from_transform(t);
        assert_eq!(s.transform, t);
    }

    #[test]
    fn ray_intersection_scaling() {
        let s = Sphere::from_transform(scaling(2.0, 2.0, 2.0));
        let mut xs = Intersections::empty();
        s.intersect(
            Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0)), &mut xs);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn ray_intersection_transloation() {
        let s = Sphere::from_transform(translation(5.0, 0.0, 0.0));
        let mut xs = Intersections::empty();
        s.intersect(
            Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0)), &mut xs);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn sphere_normal() {
        let s = Sphere::default();
        let s3_o3 = 1f32 / 3f32.sqrt();
        assert_eq!(s.normal_at(point(1.0, 0.0, 0.0)), vector(1.0, 0.0, 0.0));
        assert_eq!(s.normal_at(point(0.0, 1.0, 0.0)), vector(0.0, 1.0, 0.0));
        assert_eq!(s.normal_at(point(0.0, 0.0, 1.0)), vector(0.0, 0.0, 1.0));
        let n = s.normal_at(point(s3_o3, s3_o3, s3_o3));
        assert_eq!(n, vector(s3_o3, s3_o3, s3_o3));
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn sphere_normal_translation() {
        let s = Sphere::from_transform(translation(0.0, 1.0, 0.0));
        assert_eq!(s.normal_at(point(0.0, 1.70711, -0.70711)),
                   vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn sphere_normal_transformed() {
        use std::f32::consts::PI;
        use std::f32::consts::FRAC_1_SQRT_2 as S2O2;

        let m = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
        let s = Sphere::from_transform(m);
        assert_eq!(s.normal_at(point(0.0, S2O2, -S2O2)),
                   vector(0.0, 0.97014, -0.24254));
    }
}

/*
Feature: Spheres

Scenario: A sphere has a default material
  Given s â† sphere()
  When m â† s.material
  Then m = material()

Scenario: A sphere may be assigned a material
  Given s â† sphere()
    And m â† material()
    And m.ambient â† 1
  When s.material â† m
  Then s.material = m

Scenario: A helper for producing a sphere with a glassy material
  Given s â† glass_sphere()
  Then s.transform = identity_matrix
    And s.material.transparency = 1.0
    And s.material.refractive_index = 1.5
*/
