use std::result;
use crate::BishopError;

/// Local result type
pub type Result<T> = result::Result<T, BishopError>;
