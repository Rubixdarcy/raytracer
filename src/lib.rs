pub mod tuple;
pub mod color;
pub mod canvas;

pub mod prelude {
    pub use crate::tuple::{point, vector, T4};
    pub use crate::color::{color, Color};
    pub use crate::canvas::{Canvas};
}
