use crate::prelude::*;

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
}

impl Shape for Sphere {
    fn local_intersect<'a>(&'a self, ray: Ray, xs: &mut Intersections<'a>) {
        let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0);

        let a = ray.direction * ray.direction;
        let b = 2.0 * (ray.direction * sphere_to_ray);
        let c = sphere_to_ray * sphere_to_ray - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 { return; }

        let sqrt = discriminant.sqrt();

        xs.extend([
            Intersection::new((-b - sqrt) / (2.0 * a), self),
            Intersection::new((-b + sqrt) / (2.0 * a), self),
        ].iter().copied());
    }

    fn local_normal_at(&self, local_point: T4) -> T4 {
        local_point - point(0.0, 0.0, 0.0)
    }

    fn material(&self) -> Material { self.material }
    fn set_material(&mut self, material: Material) -> &mut Self {
        self.material = material;
        return self;
    }

    fn transform(&self) -> M4 { self.transform }
    fn set_transform(&mut self, transform: M4) -> &mut Self {
        self.transform = transform;
        return self;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ray_intersection_two_points() {
        let s = Sphere::default();
        let mut xs = Intersections::empty();
        s.local_intersect(
            Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0)), &mut xs);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersection_tangent() {
        let s = Sphere::default();
        let mut xs = Intersections::empty();
        s.local_intersect(
            Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0)), &mut xs);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_intersection_miss() {
        let s = Sphere::default();
        let mut xs = Intersections::empty();
        s.local_intersect(
            Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0)), &mut xs);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_intersection_origin_inside() {
        let s = Sphere::default();
        let mut xs = Intersections::empty();
        s.local_intersect(
            Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0)), &mut xs);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn ray_intersection_sphere_behind() {
        let s = Sphere::default();
        let mut xs = Intersections::empty();
        s.local_intersect(
            Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0)), &mut xs);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersection_sets_object() {
        let s = Sphere::default();
        let mut xs = Intersections::empty();
        s.local_intersect(
            Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0)), &mut xs);
        assert_eq!(xs.len(), 2);
        assert_eq!(*xs[0].object, s);
        assert_eq!(*xs[1].object, s);
    }

    #[test]
    fn sphere_normal() {
        let s = Sphere::default();
        let s3_o3 = 1f64 / 3f64.sqrt();
        assert_eq!(s.normal_at(point(1.0, 0.0, 0.0)), vector(1.0, 0.0, 0.0));
        assert_eq!(s.normal_at(point(0.0, 1.0, 0.0)), vector(0.0, 1.0, 0.0));
        assert_eq!(s.normal_at(point(0.0, 0.0, 1.0)), vector(0.0, 0.0, 1.0));
        let n = s.local_normal_at(point(s3_o3, s3_o3, s3_o3));
        assert_eq!(n, vector(s3_o3, s3_o3, s3_o3));
        assert_eq!(n, n.normalize());
    }
}

/*
Feature: Spheres

Scenario: A helper for producing a sphere with a glassy material
  Given s â† glass_sphere()
  Then s.transform = identity_matrix
    And s.material.transparency = 1.0
    And s.material.refractive_index = 1.5
*/
