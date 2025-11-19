#![allow(clippy::needless_continue)]

pub(crate) mod dispatch;
pub(crate) mod query;
pub(crate) mod recognize;

use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    TokenStreamExt as _,
    quote,
};
use syn::{
    DeriveInput,
    Ident,
};

use crate::projection::{
    dispatch::DispatchDerive,
    query::{
        QueryDefinition,
        QueryDerive,
    },
    recognize::RecognizeDerive,
};

// =================================================================================================
// Projection
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(projection), supports(struct_named))]
pub struct ProjectionDerive {
    ident: Ident,
    query: QueryDefinition,
}

impl ProjectionDerive {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl ProjectionDerive {
    fn projection(ident: &Ident) -> TokenStream {
        quote! {
            impl eventric_surface::projection::Projection for #ident {}
        }
    }
}

impl ToTokens for ProjectionDerive {
    #[rustfmt::skip]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(ProjectionDerive::projection(&self.ident));
        tokens.append_all(DispatchDerive::dispatch(&self.ident, &self.query.events()));
        tokens.append_all(QueryDerive::query(&self.ident, self.query.selectors()));
        tokens.append_all(RecognizeDerive::recognize(&self.ident, &self.query.events()));
    }
}
