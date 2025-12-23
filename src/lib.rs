//! Token system based on Paseto v4.public
//! - Auth service: generates tokens
//! - Other services: validates tokens

mod claims;
mod token;
mod errors;

pub use claims::Claims;
pub use token::{create_paseto_v4_public, verify_paseto_v4_public};
pub use errors::PasetoError;