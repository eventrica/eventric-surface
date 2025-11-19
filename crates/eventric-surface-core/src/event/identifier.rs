#![allow(clippy::needless_continue)]

use darling::FromDeriveInput;
use eventric_stream::{
    error::Error,
    event::Identifier,
};
use proc_macro2::{
    TokenStream,
    TokenTree,
};
use quote::{
    ToTokens,
    TokenStreamExt as _,
    quote,
};
use syn::{
    DeriveInput,
    Ident,
    Meta,
};

// =================================================================================================
// Identifier
// =================================================================================================

pub trait Identified {
    fn identifier() -> Result<&'static Identifier, Error>;
}

// =================================================================================================
// Identifier Macros
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(identified), supports(struct_named))]
pub(crate) struct IdentifiedDerive {
    ident: Ident,
    #[darling(with = "parse")]
    identifier: String,
}

impl IdentifiedDerive {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input).and_then(|identifier| {
            IdentifiedDerive::validate(&identifier.identifier.clone(), identifier)
        })
    }
}

impl IdentifiedDerive {
    pub fn identifier(ident: &Ident, identifier: &str) -> TokenStream {
        let cell_type = quote! {std::sync::OnceLock };
        let identifier_type = quote! { eventric_stream::event::Identifier };
        let error_type = quote! { eventric_stream::error::Error };

        quote! {
            impl eventric_surface::event::Identified for #ident {
                fn identifier() -> Result<&'static #identifier_type, #error_type> {
                    static IDENTIFIER: #cell_type<#identifier_type> = #cell_type::new();

                    IDENTIFIER.get_or_try_init(|| #identifier_type::new(#identifier))
                }
            }
        }
    }
}

impl IdentifiedDerive {
    pub fn validate<T>(ident: &str, ok: T) -> darling::Result<T> {
        Self::validate_identifier(ident)?;

        Ok(ok)
    }

    fn validate_identifier(ident: &str) -> darling::Result<()> {
        Identifier::new(ident)
            .map(|_| ())
            .map_err(darling::Error::custom)
    }
}

impl ToTokens for IdentifiedDerive {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(IdentifiedDerive::identifier(&self.ident, &self.identifier));
    }
}

// -------------------------------------------------------------------------------------------------

// Identifier Functions

pub fn parse(meta: &Meta) -> darling::Result<String> {
    let identifier = meta.require_list()?;
    let identifier = identifier.tokens.clone().into_iter().collect::<Vec<_>>();

    match &identifier[..] {
        [TokenTree::Ident(ident)] => Ok(ident.to_string()),
        _ => Err(darling::Error::unsupported_shape("identifier")),
    }
}
