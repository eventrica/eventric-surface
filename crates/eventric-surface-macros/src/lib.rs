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
    macros::event_derive(&parse_macro_input!(input)).into()
}

#[proc_macro_derive(Identifier, attributes(identifier))]
pub fn identifier(input: TokenStream) -> TokenStream {
    macros::identifier_derive(&parse_macro_input!(input)).into()
}

#[proc_macro_derive(Tags, attributes(tags))]
pub fn tags(input: TokenStream) -> TokenStream {
    macros::tags_derive(&parse_macro_input!(input)).into()
}

// -------------------------------------------------------------------------------------------------

// Projection

#[proc_macro_derive(Projection, attributes(projection))]
pub fn projection(input: TokenStream) -> TokenStream {
    macros::projection_derive(&parse_macro_input!(input)).into()
}

#[proc_macro_derive(Dispatch, attributes(dispatch))]
pub fn dispatch(input: TokenStream) -> TokenStream {
    macros::dispatch_derive(&parse_macro_input!(input)).into()
}

#[proc_macro_derive(Query, attributes(query))]
pub fn query(input: TokenStream) -> TokenStream {
    macros::query_derive(&parse_macro_input!(input)).into()
}

#[proc_macro_derive(Recognize, attributes(recognize))]
pub fn recognize(input: TokenStream) -> TokenStream {
    macros::recognize_derive(&parse_macro_input!(input)).into()
}
