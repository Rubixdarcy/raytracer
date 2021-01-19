use crate::prelude::*;

#[derive(Clone, Default, Debug)]
pub struct World {
    pub objects: Vec<Sphere>,
    pub lights: Vec<Light>,
}

impl World {
    pub fn new(objects: Vec<Sphere>, lights: Vec<Light>) -> Self {
        Self { objects, lights }
    }

    // A world with 1 light source and 2 concentric spheres centered at the
    // origin.
    pub fn simple() -> Self {
        let objects = vec![
            Sphere {
                material: Material {
                    color: color_rgb!(0.8, 1.0, 0.6),
                    diffuse: 0.7,
                    specular: 0.2,
                    ..Material::default()
                },
                ..Sphere::default()
            },
            Sphere {
                transform: scaling(0.5, 0.5, 0.5),
                ..Sphere::default()
            },
        ];
        let lights = vec![Light::new(point(-10.0, 10.0, -10.0), Color::WHITE)];
        Self { objects, lights }
    }

    fn intersect<'a>(&'a self, ray: Ray, xs: &mut Intersections<'a>) {
        for obj in self.objects.iter() {
            obj.intersect(ray, xs);
        }
    }

    // Given world and intersection computations calculate colour
    fn shade_hit<'a>(&'a self, comps: Computations<'a>, xs: &mut Intersections<'a>) -> Color {
        let shadow = self.is_shadowed(comps.over_point, xs);
        comps.object.material.lighting(
            self.lights[0],
            comps.over_point,
            comps.eyev,
            comps.normalv,
            shadow,
        )
    }

    pub fn color_at<'a>(&'a self, ray: Ray, xs1: &mut Intersections<'a>, xs2: &mut Intersections<'a>) -> Color {
        for object in self.objects.iter() {
            object.intersect(ray, xs1);
        }

        let hit = match xs1.hit() {
            None => return Color::BLACK,
            Some(h) => h,
        };

        let comps = hit.prepare_computations(ray);

        return self.shade_hit(comps, xs2);
    }

    pub fn is_shadowed<'a>(&'a self, point: T4, xs: &mut Intersections<'a>) -> bool {
      let v = self.lights[0].pos - point;
      let distance = v.mag();
      let direction = v.normalize();

      let r = Ray::new(point, direction);
      self.intersect(r, xs);

      let h = xs.hit();

      if let Some(h) = h {
          return h.t < distance;
      }
      return false;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn world_new() {
        let w = World::default();

        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);
    }

    #[test]
    fn world_simple() {
        let w = World::simple();
        assert_eq!(w.objects.len(), 2);
        assert_eq!(w.lights.len(), 1);
        let s1 = w.objects[0];
        let s2 = w.objects[1];
        let l = w.lights[0];

        assert_eq!(s1.material.color, color_rgb!(0.8, 1.0, 0.6));
        assert_eq!(s1.material.diffuse, 0.7);
        assert_eq!(s1.material.specular, 0.2);
        assert_eq!(s2.transform, scaling(0.5, 0.5, 0.5));
        assert_eq!(l, Light::new(point(-10.0, 10.0, -10.0), color_rgb!(1.0, 1.0, 1.0)));
    }

    #[test]
    fn world_intersect_ray() {
        let w = World::simple();
        let mut xs = Intersections::empty();
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        w.intersect(ray, &mut xs);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shade_intersection() {
        let w = World::simple();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = w.objects[0];
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(r);
        assert_eq!(w.shade_hit(comps, &mut Intersections::empty()), color_rgb!(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shade_intersection_from_inside() {
        let mut w = World::simple();
        w.lights[0] = Light::new(point(0.0, 0.25, 0.0), color_rgb!(1.0, 1.0, 1.0));
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = w.objects[1];
        let i = Intersection::new(0.5, &shape);
        let comps = i.prepare_computations(r);
        assert_eq!(w.shade_hit(comps, &mut Intersections::empty()), color_rgb!(0.90498, 0.90498, 0.90498));
    }
    
    #[test]
    fn color_when_a_ray_misses() {
        let w = World::simple();
        let mut xs = Intersections::empty();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        assert_eq!(w.color_at(r, &mut xs, &mut Intersections::empty()), Color::BLACK);
    }

    #[test]
    fn color_when_a_ray_hits() {
        let w = World::simple();
        let mut xs = Intersections::empty();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        assert_eq!(w.color_at(r, &mut xs, &mut Intersections::empty()), color_rgb!(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut w = World::simple();
        for obj in w.objects.iter_mut() {
            obj.material.ambient = 1.0;
        }
        let mut xs = Intersections::empty();
        let r = Ray::new(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
        assert_eq!(w.color_at(r, &mut xs, &mut Intersections::empty()), w.objects[1].material.color);
    }

    #[test]
    fn no_shadow_when_nothing_collinear_with_point_and_light() {
        assert!(!World::simple().is_shadowed(point(0.0, 10.0, 0.0), &mut Intersections::empty()));
    }

    #[test]
    fn shadow_when_object_between_point_and_light() {
        assert!(!World::simple().is_shadowed(point(10.0, -10.0, 0.0), &mut Intersections::empty()));
    }

    #[test]
    fn no_shadow_when_object_behind_light() {
        assert!(!World::simple().is_shadowed(point(-20.0, 20.0, -20.0), &mut Intersections::empty()));
    }

    #[test]
    fn no_shadow_when_object_behind_point() {
        assert!(!World::simple().is_shadowed(point(-2.0, 2.0, -2.0), &mut Intersections::empty()));
    }

    #[test]
    fn shade_hit_is_given_intersection_in_shadow() {
        let w = World::new(
            vec![
                Sphere::default(),
                Sphere {
                    transform: translation(0.0, 0.0, 10.0),
                    ..Sphere::default()
                }
            ],
            vec![Light::new(point(0.0, 0.0, -10.0), color_rgb!(1.0, 1.0, 1.0))]
        );
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, &w.objects[1]);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps, &mut Intersections::empty());
        assert_eq!(c, color_rgb!(0.1, 0.1, 0.1));
    }
}

/* Feature: World

Scenario: shade_hit() is given an intersection in shadow
  Given w â† world()
    And w.light â† point_light(point(0, 0, -10), color(1, 1, 1))
    And s1 â† sphere()
    And s1 is added to w
    And s2 â† sphere() with:
      | transform | translation(0, 0, 10) |
    And s2 is added to w
    And r â† ray(point(0, 0, 5), vector(0, 0, 1))
    And i â† intersection(4, s2)
  When comps â† prepare_computations(i, r)
    And c â† shade_hit(w, comps)
  Then c = color(0.1, 0.1, 0.1)

Scenario: The reflected color for a nonreflective material
  Given w â† default_world()
    And r â† ray(point(0, 0, 0), vector(0, 0, 1))
    And shape â† the second object in w
    And shape.material.ambient â† 1
    And i â† intersection(1, shape)
  When comps â† prepare_computations(i, r)
    And color â† reflected_color(w, comps)
  Then color = color(0, 0, 0)

Scenario: The reflected color for a reflective material
  Given w â† default_world()
    And shape â† plane() with:                 
      | material.reflective | 0.5                   |
      | transform           | translation(0, -1, 0) |   
    And shape is added to w
    And r â† ray(point(0, 0, -3), vector(0, -âˆš2/2, âˆš2/2))
    And i â† intersection(âˆš2, shape)
  When comps â† prepare_computations(i, r)
    And color â† reflected_color(w, comps)
  Then color = color(0.19032, 0.2379, 0.14274)

Scenario: shade_hit() with a reflective material
  Given w â† default_world()
    And shape â† plane() with:
      | material.reflective | 0.5                   |
      | transform           | translation(0, -1, 0) |
    And shape is added to w
    And r â† ray(point(0, 0, -3), vector(0, -âˆš2/2, âˆš2/2))
    And i â† intersection(âˆš2, shape)
  When comps â† prepare_computations(i, r)
    And color â† shade_hit(w, comps)
  Then color = color(0.87677, 0.92436, 0.82918)

Scenario: color_at() with mutually reflective surfaces
  Given w â† world()
    And w.light â† point_light(point(0, 0, 0), color(1, 1, 1))
    And lower â† plane() with:
      | material.reflective | 1                     |
      | transform           | translation(0, -1, 0) |
    And lower is added to w
    And upper â† plane() with:
      | material.reflective | 1                    |
      | transform           | translation(0, 1, 0) |
    And upper is added to w
    And r â† ray(point(0, 0, 0), vector(0, 1, 0))
  Then color_at(w, r) should terminate successfully

Scenario: The reflected color at the maximum recursive depth
  Given w â† default_world()
    And shape â† plane() with:
      | material.reflective | 0.5                   |
      | transform           | translation(0, -1, 0) |
    And shape is added to w
    And r â† ray(point(0, 0, -3), vector(0, -âˆš2/2, âˆš2/2))
    And i â† intersection(âˆš2, shape)
  When comps â† prepare_computations(i, r)
    And color â† reflected_color(w, comps, 0)    
  Then color = color(0, 0, 0)

Scenario: The refracted color with an opaque surface
  Given w â† default_world()
    And shape â† the first object in w
    And r â† ray(point(0, 0, -5), vector(0, 0, 1))
    And xs â† intersections(4:shape, 6:shape)
  When comps â† prepare_computations(xs[0], r, xs)
    And c â† refracted_color(w, comps, 5)
  Then c = color(0, 0, 0)

Scenario: The refracted color at the maximum recursive depth
  Given w â† default_world()
    And shape â† the first object in w
    And shape has:
      | material.transparency     | 1.0 |
      | material.refractive_index | 1.5 |
    And r â† ray(point(0, 0, -5), vector(0, 0, 1))
    And xs â† intersections(4:shape, 6:shape)
  When comps â† prepare_computations(xs[0], r, xs)
    And c â† refracted_color(w, comps, 0)
  Then c = color(0, 0, 0)

Scenario: The refracted color under total internal reflection
  Given w â† default_world()
    And shape â† the first object in w
    And shape has:
      | material.transparency     | 1.0 |
      | material.refractive_index | 1.5 |
    And r â† ray(point(0, 0, âˆš2/2), vector(0, 1, 0))
    And xs â† intersections(-âˆš2/2:shape, âˆš2/2:shape)
  # NOTE: this time you're inside the sphere, so you need
  # to look at the second intersection, xs[1], not xs[0]
  When comps â† prepare_computations(xs[1], r, xs)
    And c â† refracted_color(w, comps, 5)
  Then c = color(0, 0, 0)

Scenario: The refracted color with a refracted ray
  Given w â† default_world()
    And A â† the first object in w
    And A has:
      | material.ambient | 1.0            |
      | material.pattern | test_pattern() |
    And B â† the second object in w
    And B has:
      | material.transparency     | 1.0 |
      | material.refractive_index | 1.5 |
    And r â† ray(point(0, 0, 0.1), vector(0, 1, 0))
    And xs â† intersections(-0.9899:A, -0.4899:B, 0.4899:B, 0.9899:A)
  When comps â† prepare_computations(xs[2], r, xs)
    And c â† refracted_color(w, comps, 5)
  Then c = color(0, 0.99888, 0.04725)

Scenario: shade_hit() with a transparent material
  Given w â† default_world()
    And floor â† plane() with:
      | transform                 | translation(0, -1, 0) |
      | material.transparency     | 0.5                   |
      | material.refractive_index | 1.5                   |
    And floor is added to w
    And ball â† sphere() with:
      | material.color     | (1, 0, 0)                  |
      | material.ambient   | 0.5                        |
      | transform          | translation(0, -3.5, -0.5) |
    And ball is added to w
    And r â† ray(point(0, 0, -3), vector(0, -âˆš2/2, âˆš2/2))
    And xs â† intersections(âˆš2:floor)
  When comps â† prepare_computations(xs[0], r, xs)
    And color â† shade_hit(w, comps, 5)
  Then color = color(0.93642, 0.68642, 0.68642)

Scenario: shade_hit() with a reflective, transparent material
  Given w â† default_world()
    And r â† ray(point(0, 0, -3), vector(0, -âˆš2/2, âˆš2/2))
    And floor â† plane() with:
      | transform                 | translation(0, -1, 0) |
      | material.reflective       | 0.5                   |
      | material.transparency     | 0.5                   |
      | material.refractive_index | 1.5                   |
    And floor is added to w
    And ball â† sphere() with:
      | material.color     | (1, 0, 0)                  |
      | material.ambient   | 0.5                        |
      | transform          | translation(0, -3.5, -0.5) |
    And ball is added to w
    And xs â† intersections(âˆš2:floor)
  When comps â† prepare_computations(xs[0], r, xs)
    And color â† shade_hit(w, comps, 5)
  Then color = color(0.93391, 0.69643, 0.69243)
*/
