use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    pub transform: M4,

    // Cached calculations
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64, transform: M4) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = half_width * 2.0 / hsize as f64;

        Self { hsize, vsize, field_of_view, transform, pixel_size, half_width, half_height }
    }

    pub fn simple(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        Self::new(hsize, vsize, field_of_view, M4::IDENTITY)
    }

    pub fn pixel_size(self) -> f64 {
        self.pixel_size
    }

    pub fn ray_for_pixel(self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let cam2world = self.transform.inverse();
        let origin = cam2world * point(0.0, 0.0, 0.0);
        // We use z = -1 because the camera is at the origin and points at -Z
        let pixel = cam2world * point(world_x, world_y, -1.0);

        let direction = (pixel - origin).normalize();

        return Ray::new(origin, direction);
    }

    pub fn render(self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.hsize, self.vsize);
        let mut xs1 = Intersections::empty();
        let mut xs2 = Intersections::empty();

        for row in 0..self.vsize {
            for col in 0..self.hsize {
                xs1.clear();
                xs2.clear();
                let ray = self.ray_for_pixel(col, row);
                let c = world.color_at(ray, &mut xs1, &mut xs2);

                canvas.write_pixel(col as i32, row as i32, c);
            }
        }

        return canvas;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f64::consts::FRAC_PI_2;
    use std::f64::consts::FRAC_PI_4;
    use std::f64::consts::FRAC_1_SQRT_2 as S2O2;

    #[test]
    fn constructing_a_camera() {
        let c = Camera::simple(160, 120, FRAC_PI_2);
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, FRAC_PI_2);
        assert_eq!(c.transform, M4::IDENTITY);
    }

    #[test]
    fn pixel_size_horizontal_canvas() {
        assert!(float_eq!(Camera::simple(200, 125, FRAC_PI_2).pixel_size(), 0.01));
    }

    #[test]
    fn pixel_size_vertical_canvas() {
        assert!(float_eq!(Camera::simple(125, 200, FRAC_PI_2).pixel_size(), 0.01));
    }
    
    #[test]
    fn constructing_ray_center_canvas() {
        let c = Camera::simple(201, 101, FRAC_PI_2);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_ray_corner_canvas() {
        let c = Camera::simple(201, 101, FRAC_PI_2);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_ray_camera_transformed() {
        let c = Camera::new(201, 101, FRAC_PI_2, rotation_y(FRAC_PI_4) * translation(0.0, -2.0, 5.0));
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, point(0.0, 2.0, -5.0));
        assert_eq!(r.direction, vector(S2O2, 0.0, -S2O2));
    }

    #[test]
    fn rendering_world_with_camera() {
        let w = World::simple();
        let mut c = Camera::simple(11, 11, FRAC_PI_2);

        let from = point(0.0, 0.0, -5.0);
        let to = T4::ZERO;
        let up = T4::EY;

        c.transform = view_transform(from, to, up);

        let col = c.render(&w).pixel_at(5, 5);
        assert_eq!(col, color_rgb!(0.38066, 0.47583, 0.2855))
    }
}