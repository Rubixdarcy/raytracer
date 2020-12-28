pub mod tuple;
pub mod color;
pub mod canvas;
pub mod matrix;
pub mod transformations;

pub mod prelude {
    pub use crate::tuple::{tuple, point, vector, T4};
    pub use crate::color::{color, Color};
    pub use crate::canvas::{Canvas};
    pub use crate::matrix::{M2, M3, M4, matrix2, matrix3, matrix4};
    pub use crate::transformations::{translation, scaling, rotation_x, rotation_y, rotation_z, shearing};
}
