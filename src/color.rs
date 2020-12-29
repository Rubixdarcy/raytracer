use float_cmp::approx_eq;

#[derive(Copy, Clone, Debug, Default)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        approx_eq!(f32, self.red, other.red, epsilon = 0.00001)
        && approx_eq!(f32, self.green, other.green, epsilon = 0.00001)
        && approx_eq!(f32, self.blue, other.blue, epsilon = 0.00001)
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

impl std::ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, k: f32) -> Self {
        Self {
            red: self.red * k,
            green: self.green * k,
            blue: self.blue * k,
        }
    }
}

impl std::ops::Mul<Color> for f32 {
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

impl std::ops::Div<f32> for Color {
    type Output = Self;

    fn div(self, k: f32) -> Self {
        Self {
            red: self.red / k,
            green: self.green / k,
            blue: self.blue / k,
        }
    }
}

pub fn color(red: f32, green: f32, blue: f32) -> Color {
    Color { red, green, blue }
}

#[macro_export]
macro_rules! color_rgb {
    ($r:expr, $g:expr, $b:expr) => {
        $crate::color::Color { red: $r, green: $g, blue: $b }
    }
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
