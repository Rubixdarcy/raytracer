use std::cell::Cell;

use crate::prelude::*;

pub trait Shape: Default {
    fn local_intersect<'a>(&'a self, local_ray: Ray, xs: &mut Intersections<'a>);
    fn local_normal_at(&self, local_point: T4) -> T4;

    fn material(&self) -> Material;
    fn set_material(&mut self, material: Material) -> &mut Self;

    fn transform(&self) -> M4;
    fn set_transform(&mut self, transform: M4) -> &mut Self;

    fn intersect<'a>(&'a self, ray: Ray, xs: &mut Intersections<'a>) {
        let local_ray = self.transform().inverse() * ray;
        self.local_intersect(local_ray, xs);
    }
    fn normal_at(&self, point: T4) -> T4 {
        let tf_inv = self.transform().inverse();
        let local_point = tf_inv * point;
        let local_normal = self.local_normal_at(local_point);

        // Technically we should find the world normal by finding
        // the transpose inverse of a 3x3 matrix, but instead we
        // use the 4x4 and then manually set w to 0.
        let mut world_normal = tf_inv.transpose() * local_normal;
        world_normal.w = 0.0;
        return world_normal.normalize();
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TestShape {
    pub transform: M4,
    pub material: Material,
    pub saved_ray: Cell<Ray>,
}

impl Default for TestShape {
    fn default() -> Self {
        TestShape {
            transform: M4::IDENTITY,
            material: Material::default(),
            saved_ray: Cell::new(Ray::new(T4::ZERO, T4::ZERO)),
        }
    }
}

impl Shape for TestShape {
    fn local_intersect<'a>(&self, local_ray: Ray, _xs: &mut Intersections<'a>) {
        self.saved_ray.set(local_ray);
    }
    fn local_normal_at(&self, local_point: T4) -> T4 {
        vector(local_point.x, local_point.y, local_point.z)
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
    use std::f64::consts::FRAC_1_SQRT_2 as S2O2;
    use std::f64::consts::PI;

    #[test]
    fn default_transformation() {
        assert_eq!(TestShape::default().transform(), M4::IDENTITY);
    }

    #[test]
    fn assigning_transformation() {
        let t = translation(2.0, 3.0, 4.0);
        assert_eq!(TestShape::default().set_transform(t).transform(), t);
    }

    #[test]
    fn default_material() {
        assert_eq!(TestShape::default().material(), Material::default());
    }

    #[test]
    fn assigning_material() {
        let m = Material {
            ambient: 1.0,
            ..Material::default()
        };
        assert_eq!(TestShape::default().set_material(m).material(), m);
    }

    #[test]
    fn intersecting_scaled_shape_with_ray() {
        let mut s = TestShape::default();
        s.set_transform(scaling(2.0, 2.0, 2.0)); 
        s.intersect(Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0)), &mut Intersections::empty());

        assert_eq!(s.saved_ray.get().origin, point(0.0, 0.0, -2.5));
        assert_eq!(s.saved_ray.get().direction, vector(0.0, 0.0, 0.5));
    }

    #[test]
    fn intersecting_translated_shape_with_ray() {
        let mut s = TestShape::default();
        s.set_transform(translation(5.0, 0.0, 0.0)); 
        s.intersect(Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0)), &mut Intersections::empty());

        assert_eq!(s.saved_ray.get().origin, point(-5.0, 0.0, -5.0));
        assert_eq!(s.saved_ray.get().direction, vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn computing_normal_on_translated_shape() {
        let mut s = TestShape::default();
        s.set_transform(translation(0.0, 1.0, 0.0));
        assert_eq!(s.normal_at(point(0.0, 1.70711, -0.70711)), vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_normal_on_transformed_shape() {
        let mut s = TestShape::default();
        s.set_transform(scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0));
        assert_eq!(s.normal_at(point(0.0, S2O2, -S2O2)), vector(0.0, 0.97014, -0.24254));
    }
}

/*
Feature: Abstract Shapes

Scenario: A shape has a parent attribute
  Given s â† test_shape()
  Then s.parent is nothing

Scenario: Converting a point from world to object space
  Given g1 â† group()
    And set_transform(g1, rotation_y(Ï€/2))
    And g2 â† group()
    And set_transform(g2, scaling(2, 2, 2))
    And add_child(g1, g2)
    And s â† sphere()
    And set_transform(s, translation(5, 0, 0))
    And add_child(g2, s)
  When p â† world_to_object(s, point(-2, 0, -10))
  Then p = point(0, 0, -1)

Scenario: Converting a normal from object to world space
  Given g1 â† group()
    And set_transform(g1, rotation_y(Ï€/2))
    And g2 â† group()
    And set_transform(g2, scaling(1, 2, 3))
    And add_child(g1, g2)
    And s â† sphere()
    And set_transform(s, translation(5, 0, 0))
    And add_child(g2, s)
  When n â† normal_to_world(s, vector(âˆš3/3, âˆš3/3, âˆš3/3))
  Then n = vector(0.2857, 0.4286, -0.8571)

Scenario: Finding the normal on a child object
  Given g1 â† group()
    And set_transform(g1, rotation_y(Ï€/2))
    And g2 â† group()
    And set_transform(g2, scaling(1, 2, 3))
    And add_child(g1, g2)
    And s â† sphere()
    And set_transform(s, translation(5, 0, 0))
    And add_child(g2, s)
  When n â† normal_at(s, point(1.7321, 1.1547, -5.5774))
  Then n = vector(0.2857, 0.4286, -0.8571)
*/