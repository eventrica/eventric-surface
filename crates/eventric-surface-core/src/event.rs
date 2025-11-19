#![allow(clippy::needless_continue)]

pub(crate) mod codec;
pub(crate) mod identifier;
pub(crate) mod tag;

use std::collections::HashMap;

use darling::FromDeriveInput;
use eventric_stream::{
    error::Error,
    event::Specifier,
};
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    TokenStreamExt as _,
    quote,
};
use serde::{
    Serialize,
    de::DeserializeOwned,
};
use syn::{
    DeriveInput,
    Ident,
};

use crate::{
    event::{
        identifier::IdentifiedDerive,
        tag::TaggedDerive,
    },
    macros::List,
};

// =================================================================================================
// Event
// =================================================================================================

// Event

pub trait Event: DeserializeOwned + Identified + Tagged + Serialize {}

// Specified

pub trait Specified {
    fn specifier() -> Result<Specifier, Error>;
}

impl<T> Specified for T
where
    T: Identified,
{
    fn specifier() -> Result<Specifier, Error> {
        T::identifier().cloned().map(Specifier::new)
    }
}

// =================================================================================================
// Event Macros
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(event), supports(struct_named))]
pub(crate) struct EventDerive {
    ident: Ident,
    #[darling(with = "identifier::parse")]
    identifier: String,
    #[darling(map = "tag::map")]
    tags: Option<HashMap<Ident, List<tag::TagValueSource>>>,
}

impl EventDerive {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
            .and_then(|event| IdentifiedDerive::validate(&event.identifier.clone(), event))
    }
}

impl EventDerive {
    fn event(ident: &Ident) -> TokenStream {
        quote! {
            impl eventric_surface::event::Event for #ident {}
        }
    }
}

impl ToTokens for EventDerive {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(EventDerive::event(&self.ident));
        tokens.append_all(IdentifiedDerive::identifier(&self.ident, &self.identifier));
        tokens.append_all(TaggedDerive::tags(&self.ident, self.tags.as_ref()));
    }
}

// -------------------------------------------------------------------------------------------------

// Re-Exports

pub use self::{
    codec::{
        Codec,
        JsonCodec,
    },
    identifier::Identified,
    tag::Tagged,
};
