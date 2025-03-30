mod app;
mod config;
mod error;
mod module;
mod routes;

pub type Result<T> = std::result::Result<T, Error>;

pub use app::App;
pub use error::Error;
pub use module::*;
