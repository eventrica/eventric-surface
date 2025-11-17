//! See the `eventric-surface` crate for full documentation, including
//! crate-level documentation.

#![allow(clippy::multiple_crate_versions)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(clippy::missing_safety_doc)]
#![allow(missing_docs)]

use eventric_surface_core::macros;
use proc_macro::TokenStream;
use syn::parse_macro_input;

// =================================================================================================
// Eventric Surface Macro
// =================================================================================================

// Event

#[proc_macro_derive(Event, attributes(event))]
pub fn event(input: TokenStream) -> TokenStream {
    macros::derive::event(&parse_macro_input!(input)).into()
}

#[proc_macro_derive(Identified, attributes(identified))]
pub fn identified(input: TokenStream) -> TokenStream {
    macros::derive::identifier(&parse_macro_input!(input)).into()
}

#[proc_macro_derive(Tagged, attributes(tagged))]
pub fn tagged(input: TokenStream) -> TokenStream {
    macros::derive::tagged(&parse_macro_input!(input)).into()
}

// -------------------------------------------------------------------------------------------------

// Projection

#[proc_macro_derive(Projection, attributes(projection))]
pub fn projection(input: TokenStream) -> TokenStream {
    macros::derive::projection(&parse_macro_input!(input)).into()
}
