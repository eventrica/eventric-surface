use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

use crate::{
    event,
    projection,
};

// =================================================================================================
// Derive
// =================================================================================================

macro_rules! emit_impl_or_error {
    ($e:expr) => {
        match $e {
            Ok(val) => val.into_token_stream(),
            Err(err) => err.write_errors(),
        }
    };
}

// -------------------------------------------------------------------------------------------------

// Event

#[must_use]
pub fn event(input: &DeriveInput) -> TokenStream {
    emit_impl_or_error!(event::macros::Event::new(input))
}

#[must_use]
pub fn identifier(input: &DeriveInput) -> TokenStream {
    emit_impl_or_error!(event::macros::Identified::new(input))
}

#[must_use]
pub fn tagged(input: &DeriveInput) -> TokenStream {
    emit_impl_or_error!(event::macros::Tagged::new(input))
}

// -------------------------------------------------------------------------------------------------

// Projection

#[must_use]
pub fn projection(input: &DeriveInput) -> TokenStream {
    emit_impl_or_error!(projection::Derive::new(input))
}
