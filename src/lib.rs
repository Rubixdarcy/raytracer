pub mod tuple;
pub mod color;
pub mod canvas;
pub mod matrix;
pub mod transformations;
pub mod ray;
pub mod sphere;
pub mod intersection;
pub mod light;
pub mod material;
pub mod world;
pub mod camera;

pub mod consts {
    pub const EPSILON: f32 = 0.00001;
    pub const SHADOW_SHIFT_LENGTH: f32 = EPSILON * 500.0;
}

pub mod prelude {
    pub use crate::{color_rgb};
    pub use crate::tuple::{tuple, point, vector, T4};
    pub use crate::color::{color, Color};
    pub use crate::canvas::{Canvas};
    pub use crate::matrix::{M2, M3, M4, matrix2, matrix3, matrix4};
    pub use crate::transformations::{translation, scaling, rotation_x, rotation_y, rotation_z, shearing, view_transform};
    pub use crate::ray::{Ray};
    pub use crate::sphere::{Sphere};
    pub use crate::intersection::{Intersection, Intersections, Computations};
    pub use crate::light::{Light};
    pub use crate::material::{Material};
    pub use crate::world::{World};
    pub use crate::camera::{Camera};
}

#[cfg(test)]
pub mod test_prelude {
    use crate::prelude::*;
    use nom::{
        IResult,
        combinator::{map_res, map},
        sequence::{preceded},
        multi::{many1},
        bytes::complete::{take_while1},
    };

    pub fn parse_matrix4(i: &str) -> M4 {
        map(many1(preceded(take_while1(|b| !num_char(b)), num)),
            |v| matrix4(v[0], v[1], v[2], v[3],
                        v[4], v[5], v[6], v[7],
                        v[8], v[9], v[10], v[11],
                        v[12], v[13], v[14], v[15]))(i).unwrap().1
    }

    fn num(i: &str) -> IResult<&str, f32> {
        map_res(take_while1(num_char), |s: &str| s.parse::<f32>())(i)
    }

    fn num_char(c: char) -> bool {
        ('0' <= c && c <= '9') || c == '.' || c == '-'
    }
}
