use crate::prelude::*;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Sphere {
    transform: M4,
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere { transform: M4::IDENTITY }
    }
}

impl Sphere {
    pub fn from_transform(transform: M4) -> Self {
        Sphere { transform }
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let ray = self.transform.inverse() * ray;
        let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0);

        let a = ray.direction * ray.direction;
        let b = 2.0 * (ray.direction * sphere_to_ray);
        let c = sphere_to_ray * sphere_to_ray - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 { return Intersections::empty(); }

        let sqrt = discriminant.sqrt();

        return Intersections::new(&[
            Intersection::new((-b - sqrt) / (2.0 * a), self),
            Intersection::new((-b + sqrt) / (2.0 * a), self),
        ]);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ray_intersection_two_points() {
        let s = Sphere::default();
        let xs = s.intersect(
            Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0)));
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn ray_intersection_tangent() {
        let s = Sphere::default();
        let xs = s.intersect(
            Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0)));
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_intersection_miss() {
        let s = Sphere::default();
        let xs = s.intersect(
            Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0)));
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_intersection_origin_inside() {
        let s = Sphere::default();
        let xs = s.intersect(
            Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0)));
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn ray_intersection_sphere_behind() {
        let s = Sphere::default();
        let xs = s.intersect(
            Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0)));
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersection_sets_object() {
        let s = Sphere::default();
        let xs = s.intersect(
            Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0)));
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
        let xs = s.intersect(
            Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0)));
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn ray_intersection_transloation() {
        let s = Sphere::from_transform(translation(5.0, 0.0, 0.0));
        let xs = s.intersect(
            Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0)));
        assert_eq!(xs.len(), 0);
    }
}

/*
Feature: Spheres

Scenario: The normal on a sphere at a point on the x axis
  Given s â† sphere()
  When n â† normal_at(s, point(1, 0, 0))
  Then n = vector(1, 0, 0)

Scenario: The normal on a sphere at a point on the y axis
  Given s â† sphere()
  When n â† normal_at(s, point(0, 1, 0))
  Then n = vector(0, 1, 0)

Scenario: The normal on a sphere at a point on the z axis
  Given s â† sphere()
  When n â† normal_at(s, point(0, 0, 1))
  Then n = vector(0, 0, 1)

Scenario: The normal on a sphere at a nonaxial point
  Given s â† sphere()
  When n â† normal_at(s, point(âˆš3/3, âˆš3/3, âˆš3/3))
  Then n = vector(âˆš3/3, âˆš3/3, âˆš3/3)

Scenario: The normal is a normalized vector
  Given s â† sphere()
  When n â† normal_at(s, point(âˆš3/3, âˆš3/3, âˆš3/3))
  Then n = normalize(n)

Scenario: Computing the normal on a translated sphere
  Given s â† sphere()
    And set_transform(s, translation(0, 1, 0))
  When n â† normal_at(s, point(0, 1.70711, -0.70711))
  Then n = vector(0, 0.70711, -0.70711)

Scenario: Computing the normal on a transformed sphere
  Given s â† sphere()
    And m â† scaling(1, 0.5, 1) * rotation_z(Ï€/5)
    And set_transform(s, m)
  When n â† normal_at(s, point(0, âˆš2/2, -âˆš2/2))
  Then n = vector(0, 0.97014, -0.24254)

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
