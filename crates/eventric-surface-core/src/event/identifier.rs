#![allow(clippy::needless_continue)]

use darling::FromDeriveInput;
use eventric_stream::{
    error::Error,
    event,
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

pub trait Identifier {
    fn identifier() -> Result<&'static event::Identifier, Error>;
}

// =================================================================================================
// Identifier Macros
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(identifier), supports(struct_named))]
pub struct IdentifierDerive {
    ident: Ident,
    #[darling(with = "parse")]
    identifier: String,
}

impl IdentifierDerive {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input).and_then(|identifier| {
            IdentifierDerive::validate(&identifier.identifier.clone(), identifier)
        })
    }
}

impl IdentifierDerive {
    #[must_use]
    pub fn identifier(ident: &Ident, identifier: &str) -> TokenStream {
        let cell_type = quote! {std::sync::OnceLock };
        let identifier_type = quote! { eventric_stream::event::Identifier };
        let error_type = quote! { eventric_stream::error::Error };

        quote! {
            impl eventric_surface::event::Identifier for #ident {
                fn identifier() -> Result<&'static #identifier_type, #error_type> {
                    static IDENTIFIER: #cell_type<#identifier_type> = #cell_type::new();

                    IDENTIFIER.get_or_try_init(|| #identifier_type::new(#identifier))
                }
            }
        }
    }
}

impl IdentifierDerive {
    pub fn validate<T>(ident: &str, ok: T) -> darling::Result<T> {
        Self::validate_identifier(ident)?;

        Ok(ok)
    }

    fn validate_identifier(ident: &str) -> darling::Result<()> {
        event::Identifier::new(ident)
            .map(|_| ())
            .map_err(darling::Error::custom)
    }
}

impl ToTokens for IdentifierDerive {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(IdentifierDerive::identifier(&self.ident, &self.identifier));
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
