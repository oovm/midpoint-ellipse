#![feature(total_cmp)]

mod errors;
mod dim2;
mod dim3;
pub use errors::{Error, Result};
pub use self::dim2::Points;

mod utils;
