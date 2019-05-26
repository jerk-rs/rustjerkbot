use failure::Fail;

mod arrow;
mod cw;
mod huify;
mod reverse;
mod square;
mod star;
mod utils;

pub use self::arrow::transform as to_arrow;
pub use self::cw::transform as to_cw;
pub use self::huify::transform as to_huified;
pub use self::reverse::transform as to_reversed;
pub use self::square::transform as to_square;
pub use self::star::transform as to_star;

#[derive(Fail, Debug)]
pub enum TransformError {
    #[fail(display = "Text must contain from {} up to {} characters", min, max)]
    InvalidLength { min: usize, max: usize },
}

pub type TransformResult<T> = Result<T, TransformError>;
