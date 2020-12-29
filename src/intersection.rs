use crate::prelude::*;
use smallvec::SmallVec;
use smallvec::smallvec;
use std::borrow::Borrow;

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, object: &'a Sphere) -> Self {
        Self { t, object }
    }
}

#[derive(Clone, Debug)]
pub struct Intersections<'a>(SmallVec<[Intersection<'a>; 4]>);

impl<'a, T> Borrow<T> for Intersections<'a> where
        SmallVec<[Intersection<'a>; 4]>: Borrow<T> {
    fn borrow(&self) -> &T { self.0.borrow() }
}

impl<'a> std::ops::Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<'a> Intersections<'a> {
    pub fn empty() -> Self { Self(smallvec![]) }

    pub fn new(xs: &[Intersection<'a>]) -> Self {
        let mut xs: SmallVec<[Intersection<'a>; 4]> = SmallVec::from_slice(xs);
        xs.sort_unstable_by(|x, y| x.t.partial_cmp(&y.t).unwrap());
        Self(xs)
    }

    pub fn len(&self) -> usize { self.0.len() }

    pub fn hit(&self) -> Option<Intersection<'a>> {
        self.0.iter().find(|x| x.t > 0.0).copied()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn intersection_t_object() {
        let s = Sphere::default();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert_eq!(*i.object, s);
    }

    #[test]
    fn intersection_hit_all_positive() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        assert_eq!(Intersections::new(&[i1, i2]).hit(), Some(i1));
    }
    
    #[test]
    fn intersection_hit_some_negative() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        assert_eq!(Intersections::new(&[i1, i2]).hit(), Some(i2));
    }
    
    #[test]
    fn intersection_hit_all_negative() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        assert_eq!(Intersections::new(&[i1, i2]).hit(), None);
    }

    #[test]
    fn intersection_hit_lowest_nonnegative() {
        let s = Sphere::default();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        assert_eq!(Intersections::new(&[i1, i2, i3, i4]).hit(), Some(i4));
    }
}
/*
Feature: Intersections
Scenario: Precomputing the state of an intersection
  Given r â† ray(point(0, 0, -5), vector(0, 0, 1))
    And shape â† sphere()
    And i â† intersection(4, shape)
  When comps â† prepare_computations(i, r)
  Then comps.t = i.t
    And comps.object = i.object
    And comps.point = point(0, 0, -1)
    And comps.eyev = vector(0, 0, -1)
    And comps.normalv = vector(0, 0, -1)

Scenario: Precomputing the reflection vector
  Given shape â† plane()
    And r â† ray(point(0, 1, -1), vector(0, -âˆš2/2, âˆš2/2)) 
    And i â† intersection(âˆš2, shape)                      
  When comps â† prepare_computations(i, r)
  Then comps.reflectv = vector(0, âˆš2/2, âˆš2/2)                

Scenario: The hit, when an intersection occurs on the outside
  Given r â† ray(point(0, 0, -5), vector(0, 0, 1))
    And shape â† sphere()
    And i â† intersection(4, shape)
  When comps â† prepare_computations(i, r)
  Then comps.inside = false

Scenario: The hit, when an intersection occurs on the inside
  Given r â† ray(point(0, 0, 0), vector(0, 0, 1))
    And shape â† sphere()
    And i â† intersection(1, shape)
  When comps â† prepare_computations(i, r)
  Then comps.point = point(0, 0, 1)
    And comps.eyev = vector(0, 0, -1)
    And comps.inside = true
      # normal would have been (0, 0, 1), but is inverted!
    And comps.normalv = vector(0, 0, -1)

Scenario: The hit should offset the point
  Given r â† ray(point(0, 0, -5), vector(0, 0, 1))
    And shape â† sphere() with:
      | transform | translation(0, 0, 1) |
    And i â† intersection(5, shape)
  When comps â† prepare_computations(i, r)
  Then comps.over_point.z < -EPSILON/2
    And comps.point.z > comps.over_point.z

Scenario: The under point is offset below the surface
  Given r â† ray(point(0, 0, -5), vector(0, 0, 1))
    And shape â† glass_sphere() with:
      | transform | translation(0, 0, 1) |
    And i â† intersection(5, shape)
    And xs â† intersections(i)
  When comps â† prepare_computations(i, r, xs)
  Then comps.under_point.z > EPSILON/2
    And comps.point.z < comps.under_point.z

Scenario Outline: Finding n1 and n2 at various intersections
  Given A â† glass_sphere() with:
      | transform                 | scaling(2, 2, 2) |
      | material.refractive_index | 1.5              |
    And B â† glass_sphere() with:
      | transform                 | translation(0, 0, -0.25) |
      | material.refractive_index | 2.0                      |
    And C â† glass_sphere() with:
      | transform                 | translation(0, 0, 0.25) |
      | material.refractive_index | 2.5                     |
    And r â† ray(point(0, 0, -4), vector(0, 0, 1))
    And xs â† intersections(2:A, 2.75:B, 3.25:C, 4.75:B, 5.25:C, 6:A)
  When comps â† prepare_computations(xs[<index>], r, xs)  
  Then comps.n1 = <n1>
    And comps.n2 = <n2>             

  Examples:
    | index | n1  | n2  |
    | 0     | 1.0 | 1.5 |                 
    | 1     | 1.5 | 2.0 |
    | 2     | 2.0 | 2.5 |
    | 3     | 2.5 | 2.5 |
    | 4     | 2.5 | 1.5 |
    | 5     | 1.5 | 1.0 |

Scenario: The Schlick approximation under total internal reflection
  Given shape â† glass_sphere()
    And r â† ray(point(0, 0, âˆš2/2), vector(0, 1, 0))
    And xs â† intersections(-âˆš2/2:shape, âˆš2/2:shape)
  When comps â† prepare_computations(xs[1], r, xs)
    And reflectance â† schlick(comps)
  Then reflectance = 1.0

Scenario: The Schlick approximation with a perpendicular viewing angle
  Given shape â† glass_sphere()
    And r â† ray(point(0, 0, 0), vector(0, 1, 0))
    And xs â† intersections(-1:shape, 1:shape)
  When comps â† prepare_computations(xs[1], r, xs)
    And reflectance â† schlick(comps)
  Then reflectance = 0.04

Scenario: The Schlick approximation with small angle and n2 > n1
  Given shape â† glass_sphere()
    And r â† ray(point(0, 0.99, -2), vector(0, 0, 1))
    And xs â† intersections(1.8589:shape)
  When comps â† prepare_computations(xs[0], r, xs)
    And reflectance â† schlick(comps)
  Then reflectance = 0.48873

Scenario: An intersection can encapsulate `u` and `v`
  Given s â† triangle(point(0, 1, 0), point(-1, 0, 0), point(1, 0, 0))
  When i â† intersection_with_uv(3.5, s, 0.2, 0.4)
  Then i.u = 0.2
    And i.v = 0.4
*/
