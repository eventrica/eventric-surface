use darling::{
    Error,
    FromDeriveInput,
};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

// =================================================================================================
// Projection
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
pub(crate) struct Derive {}

impl Derive {
    pub fn new(input: &DeriveInput) -> Result<Self, Error> {
        Self::from_derive_input(input)
    }
}

impl ToTokens for Derive {
    fn to_tokens(&self, _tokens: &mut TokenStream) {}
}
