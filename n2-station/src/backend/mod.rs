mod model;
mod routes;

pub use model::{form, handler, response::Action};
pub use routes::{init, RBATIS};
