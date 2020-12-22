pub mod tuple;
pub mod color;
pub mod canvas;
pub mod matrix;

pub mod prelude {
    pub use crate::tuple::{tuple, point, vector, T4};
    pub use crate::color::{color, Color};
    pub use crate::canvas::{Canvas};
    pub use crate::matrix::{M4, matrix4};
}
