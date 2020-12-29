use crate::prelude::*;

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Light {
    pub pos: T4,
    pub intensity: Color,
}

impl Light {
    pub fn new(pos: T4, intensity: Color) -> Self {
        Self { pos, intensity }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn light_new() {
        let pos = point(0.0, 0.0, 0.0);
        let intensity = color_rgb!(1.0, 1.0, 1.0);
        let light = Light::new(pos, intensity);
        assert_eq!(light.pos, pos);
        assert_eq!(light.intensity, intensity);
    }
}
