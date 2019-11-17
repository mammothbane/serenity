//! These prelude re-exports are a set of exports that are commonly used from
//! within the library.
//!
//! These are not publicly re-exported to the end user, and must stay as a
//! private module.

pub(crate) type JsonMap = Map<String, Value>;

pub use serde_json::{Map, Number, Value};
pub(crate) use failure::Error;
pub(crate) use crate::error::SerenityError;

pub(crate) type Result<T> = ::std::result::Result<T, Error>;
pub(crate) type StdResult<T, E> = ::std::result::Result<T, E>;

#[cfg(feature = "client")]
pub use crate::client::ClientError;
