//! See the `eventric-surface` crate for full documentation, including
//! module-level documentation.

pub(crate) mod projections;
pub(crate) mod query;
pub(crate) mod update;

// =================================================================================================
// Decision
// =================================================================================================

pub trait Decision: Projections + Query + Update {}

// -------------------------------------------------------------------------------------------------

// Re-Exports

pub use self::{
    projections::Projections,
    query::Query,
    update::Update,
};
