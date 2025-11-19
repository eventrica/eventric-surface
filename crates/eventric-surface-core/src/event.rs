//! See the `eventric-surface` crate for full documentation, including
//! module-level documentation.

#![allow(clippy::needless_continue)]

pub(crate) mod codec;
pub(crate) mod identifier;
pub(crate) mod specifier;
pub(crate) mod tag;

use std::collections::HashMap;

use darling::FromDeriveInput;
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
        identifier::IdentifierDerive,
        tag::TagsDerive,
    },
    macros::List,
};

// =================================================================================================
// Event
// =================================================================================================

pub trait Event: DeserializeOwned + Identifier + Tags + Serialize {}

// -------------------------------------------------------------------------------------------------

// Re-Exports

pub use self::{
    codec::{
        Codec,
        JsonCodec,
    },
    identifier::Identifier,
    specifier::Specifier,
    tag::Tags,
};

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
    tags: Option<HashMap<Ident, List<tag::TagDefinition>>>,
}

impl EventDerive {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
            .and_then(|event| IdentifierDerive::validate(&event.identifier.clone(), event))
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
        tokens.append_all(IdentifierDerive::identifier(&self.ident, &self.identifier));
        tokens.append_all(TagsDerive::tags(&self.ident, self.tags.as_ref()));
    }
}
