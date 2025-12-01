//! See the `eventric-surface` crate for full documentation, including
//! crate-level documentation.

#![allow(clippy::multiple_crate_versions)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(clippy::missing_safety_doc)]
#![allow(missing_docs)]

pub(crate) mod decision;
pub(crate) mod event;
pub(crate) mod projection;
pub(crate) mod util;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

use crate::{
    decision::Decision,
    event::Event,
    projection::Projection,
};

// =================================================================================================
// Eventric Surface Macro
// =================================================================================================

// Helpers

macro_rules! emit_impl_or_error {
    ($e:expr) => {
        match $e {
            Ok(val) => val.into_token_stream(),
            Err(err) => err.write_errors(),
        }
    };
}

// -------------------------------------------------------------------------------------------------

// Macros

// Event

#[proc_macro_derive(Event, attributes(event))]
pub fn event(input: TokenStream) -> TokenStream {
    emit_impl_or_error!(Event::new(&parse_macro_input!(input))).into()
}

// Decision

#[proc_macro_derive(Decision, attributes(decision))]
pub fn decision(input: TokenStream) -> TokenStream {
    emit_impl_or_error!(Decision::new(&parse_macro_input!(input))).into()
}

// Projection

#[proc_macro_derive(Projection, attributes(projection))]
pub fn projection(input: TokenStream) -> TokenStream {
    emit_impl_or_error!(Projection::new(&parse_macro_input!(input))).into()
}
