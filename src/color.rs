use crate::prelude::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

#[macro_export]
macro_rules! color_rgb {
    ($r:expr, $g:expr, $b:expr) => {
        $crate::color::Color { red: $r, green: $g, blue: $b }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        float_eq!(self.red, other.red)
        && float_eq!(self.green, other.green)
        && float_eq!(self.blue, other.blue)
    }
}

impl Eq for Color {}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl std::ops::Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, k: f64) -> Self {
        Self {
            red: self.red * k,
            green: self.green * k,
            blue: self.blue * k,
        }
    }
}

impl std::ops::Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, t: Color) -> Self::Output { t * self }
}

impl std::ops::Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl std::ops::Div<f64> for Color {
    type Output = Self;

    fn div(self, k: f64) -> Self {
        Self {
            red: self.red / k,
            green: self.green / k,
            blue: self.blue / k,
        }
    }
}

impl Color {
    pub const BLACK: Self = color_rgb!(0.0, 0.0, 0.0);
    pub const RED: Self = color_rgb!(1.0, 0.0, 0.0);
    pub const GREEN: Self = color_rgb!(0.0, 1.0, 0.0);
    pub const BLUE: Self = color_rgb!(0.0, 0.0, 1.0);
    pub const WHITE: Self = color_rgb!(1.0, 1.0, 1.0);
}

pub fn color(red: f64, green: f64, blue: f64) -> Color {
    Color { red, green, blue }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn color_func() {
        let c = color(-0.5, 0.4, 1.7);
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn color_add() {
        assert_eq!(color(0.9, 0.6, 0.75) + color(0.7, 0.1, 0.25), color(1.6, 0.7, 1.0));
    }

    #[test]
    fn color_sub() {
        assert_eq!(color(0.9, 0.6, 0.75) - color(0.7, 0.1, 0.25), color(0.2, 0.5, 0.5));
    }

    #[test]
    fn color_mul() {
        assert_eq!(color(1.0, 0.2, 0.4) * color(0.9, 1.0, 0.1), color(0.9, 0.2, 0.04));
    }

    #[test]
    fn color_mul_scalar() {
        assert_eq!(2.0 * color(0.2, 0.3, 0.4), color(0.4, 0.6, 0.8));
    }
}
