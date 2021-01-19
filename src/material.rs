use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: color_rgb!(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl Material {
    pub fn lighting(self, light: Light, pos: T4, eyev: T4, normalv: T4, in_shadow: bool) -> Color {
        let effective_color = self.color * light.intensity;

        // Ambient depends on nothing
        let ambient = effective_color * self.ambient;

        // Shadow means diffuse and specular are 0
        if in_shadow { return ambient; }

        let lightv = (light.pos - pos).normalize();
        let light_normal_cos = lightv * normalv;

        let (diffuse, specular) = if light_normal_cos < 0.0 {
            (Color::BLACK, Color::BLACK)            
        } else {
            // Diffuse depends on lightv and normalv
            let diffuse = effective_color * (self.diffuse * light_normal_cos);
            let reflectv = (-lightv).reflect(normalv);
            let reflect_eye_cos = reflectv * eyev;

            let specular = if reflect_eye_cos <= 0.0 {
                Color::BLACK
            } else {
                let factor = reflect_eye_cos.powf(self.shininess);
                // Specular depends on reflectv and eyev
                light.intensity * (self.specular * factor)
            };
            (diffuse, specular)
        };
        return ambient + diffuse + specular;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f64::consts::FRAC_1_SQRT_2 as S2O2;

    #[test]
    fn material_default() {
        let material = Material::default();
        assert_eq!(material.color, color_rgb!(1.0, 1.0, 1.0));
        assert_eq!(material.ambient, 0.1);
        assert_eq!(material.diffuse, 0.9);
        assert_eq!(material.specular, 0.9);
        assert_eq!(material.shininess, 200.0);
    }

    #[test]
    fn material_lighting_eye_between_light_and_surface() {
        let (m, pos) = lighting_defaults();
        let light = Light::new(point(0.0, 0.0, -10.0), color_rgb!(1.0, 1.0, 1.0));
        let eye = vector(0.0, 0.0, -1.0);
        let normal = vector(0.0, 0.0, -1.0);
        // Result is ambient + diffuse + specular
        assert_eq!(m.lighting(light, pos, eye, normal, false), color_rgb!(1.9, 1.9, 1.9));
    }

    #[test]
    fn material_lighting_eye_between_light_and_surface_offset_45() {
        let (m, pos) = lighting_defaults();
        let light = Light::new(point(0.0, 0.0, -10.0), color_rgb!(1.0, 1.0, 1.0));
        let eye = vector(0.0, S2O2, -S2O2);
        let normal = vector(0.0, 0.0, -1.0);
        // Result is ambient + diffuse. Specular has dropped to basically zero.
        assert_eq!(m.lighting(light, pos, eye, normal, false), color_rgb!(1.0, 1.0, 1.0));
    }

    #[test]
    fn material_lighting_eye_opposite_surface_light_offset_45() {
        let (m, pos) = lighting_defaults();
        let light = Light::new(point(0.0, 10.0, -10.0), color_rgb!(1.0, 1.0, 1.0));
        let eye = vector(0.0, 0.0, -1.0);
        let normal = vector(0.0, 0.0, -1.0);
        // Specular is basically zero. Diffuse is reduced to proportion S2O2 because
        // of the angle between the light and the normal
        // result = ambient + diffuse * S2O2
        assert_eq!(m.lighting(light, pos, eye, normal, false), color_rgb!(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn material_lighting_light_offset_45_eye_in_reflection_vector() {
        let (m, pos) = lighting_defaults();
        let light = Light::new(point(0.0, 10.0, -10.0), color_rgb!(1.0, 1.0, 1.0));
        let eye = vector(0.0, -S2O2, -S2O2);
        let normal = vector(0.0, 0.0, -1.0);
        // Specular is full strenth. Diffuse is reduced to proportion S2O2 because
        // of the angle between the light and the normal
        // result = ambient + diffuse * S2O2 + specular
        //assert_eq!(m.lighting(light, pos, eye, normal), color_rgb!(1.6364, 1.6364, 1.6364));
        assert_eq!(m.lighting(light, pos, eye, normal, false), color_rgb!(1.63639, 1.63639, 1.63639));
    }

    #[test]
    fn material_lighting_light_behind_surface() {
        let (m, pos) = lighting_defaults();
        let light = Light::new(point(0.0, 0.0, 10.0), color_rgb!(1.0, 1.0, 1.0));
        let eye = vector(0.0, 0.0, -1.0);
        let normal = vector(0.0, 0.0, -1.0);
        // Result is ambient
        assert_eq!(m.lighting(light, pos, eye, normal, false), color_rgb!(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let (m, pos) = lighting_defaults();
        let light = Light::new(point(0.0, 0.0, -10.0), color_rgb!(1.0, 1.0, 1.0));
        let eye = vector(0.0, 0.0, -1.0);
        let normal = vector(0.0, 0.0, -1.0);
        // Result is ambient + diffuse + specular
        assert_eq!(m.lighting(light, pos, eye, normal, true), color_rgb!(0.1, 0.1, 0.1));
    }

    fn lighting_defaults() -> (Material, T4) {
        (Material::default(), point(0.0, 0.0, 0.0))
    }
}

/*
Feature: Materials

Background:
  Given m â† material()
    And position â† point(0, 0, 0)

Scenario: Reflectivity for the default material
  Given m â† material()
  Then m.reflective = 0.0

Scenario: Transparency and Refractive Index for the default material
  Given m â† material()
  Then m.transparency = 0.0
    And m.refractive_index = 1.0

Scenario: Lighting with a pattern applied
  Given m.pattern â† stripe_pattern(color(1, 1, 1), color(0, 0, 0))
    And m.ambient â† 1
    And m.diffuse â† 0
    And m.specular â† 0
    And eyev â† vector(0, 0, -1)
    And normalv â† vector(0, 0, -1)
    And light â† point_light(point(0, 0, -10), color(1, 1, 1))
  When c1 â† lighting(m, light, point(0.9, 0, 0), eyev, normalv, false)
    And c2 â† lighting(m, light, point(1.1, 0, 0), eyev, normalv, false)
  Then c1 = color(1, 1, 1)
    And c2 = color(0, 0, 0)
*/
