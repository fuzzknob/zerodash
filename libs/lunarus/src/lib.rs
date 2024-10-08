use errors::Error;

pub mod app;
pub mod context;
pub mod database;
pub mod errors;
pub mod prelude;
pub mod res;
pub mod utils;

pub type Result<T, E = Error> = std::result::Result<T, E>;
