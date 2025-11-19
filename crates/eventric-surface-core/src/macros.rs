use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    DeriveInput,
    Meta,
    parse::{
        Parse,
        ParseStream,
    },
    punctuated::Punctuated,
    token::Comma,
};

use crate::{
    event,
    projection,
};

// =================================================================================================
// Macros
// =================================================================================================

// List

#[derive(Debug, Clone)]
pub(crate) struct List<T>(Vec<T>)
where
    T: Parse;

impl<T> AsRef<Vec<T>> for List<T>
where
    T: Parse,
{
    fn as_ref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> FromMeta for List<T>
where
    T: Parse,
{
    fn from_meta(meta: &Meta) -> darling::Result<Self> {
        let list = meta.require_list()?;
        let list = list.tokens.clone();

        syn::parse2::<List<T>>(list).map_err(darling::Error::custom)
    }
}

impl<T> Parse for List<T>
where
    T: Parse,
{
    fn parse(stream: ParseStream<'_>) -> syn::Result<Self> {
        let list = Punctuated::<T, Comma>::parse_terminated(stream)?;
        let list = list.into_iter().collect();
        let list = Self(list);

        Ok(list)
    }
}

// -------------------------------------------------------------------------------------------------

// Macros

macro_rules! emit_impl_or_error {
    ($e:expr) => {
        match $e {
            Ok(val) => val.into_token_stream(),
            Err(err) => err.write_errors(),
        }
    };
}

// Event

#[doc(hidden)]
#[must_use]
pub fn event_derive(input: &DeriveInput) -> TokenStream {
    emit_impl_or_error!(event::EventDerive::new(input))
}

#[doc(hidden)]
#[must_use]
pub fn identifier_derive(input: &DeriveInput) -> TokenStream {
    emit_impl_or_error!(event::identifier::IdentifierDerive::new(input))
}

#[doc(hidden)]
#[must_use]
pub fn tags_derive(input: &DeriveInput) -> TokenStream {
    emit_impl_or_error!(event::tag::TagsDerive::new(input))
}

// Projection

#[doc(hidden)]
#[must_use]
pub fn projection_derive(input: &DeriveInput) -> TokenStream {
    emit_impl_or_error!(projection::ProjectionDerive::new(input))
}

#[doc(hidden)]
#[must_use]
pub fn dispatch_derive(input: &DeriveInput) -> TokenStream {
    emit_impl_or_error!(projection::dispatch::DispatchDerive::new(input))
}

#[doc(hidden)]
#[must_use]
pub fn query_derive(input: &DeriveInput) -> TokenStream {
    emit_impl_or_error!(projection::query::QueryDerive::new(input))
}

#[doc(hidden)]
#[must_use]
pub fn recognize_derive(input: &DeriveInput) -> TokenStream {
    emit_impl_or_error!(projection::recognize::RecognizeDerive::new(input))
}
