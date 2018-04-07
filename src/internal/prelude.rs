//! These prelude re-exports are a set of exports that are commonly used from
//! within the library.
//!
//! These are not publicly re-exported to the end user, and must stay as a
//! private module.

pub(crate) type JsonMap = Map<String, Value>;

pub(crate) use failure::{Error, ResultExt};
pub(crate) use std::result::Result as StdResult;
pub(crate) use serde_json::{Map, Number, Value};

#[cfg(feature = "client")]
pub(crate) use client::ClientError;
