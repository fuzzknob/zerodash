use errors::LunarError;

pub mod context;
pub mod database;
pub mod errors;
pub mod lunar_app;
pub mod prelude;
pub mod res;
pub mod utils;

pub type Result<T, E = LunarError> = std::result::Result<T, E>;
