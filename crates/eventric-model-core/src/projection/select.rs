//! See the `eventric-surface` crate for full documentation, including
//! module-level documentation.

use eventric_stream::{
    error::Error,
    stream::select::Selection,
};

// =================================================================================================
// Query
// =================================================================================================

pub trait Select {
    fn select(&self) -> Result<Selection, Error>;
}
