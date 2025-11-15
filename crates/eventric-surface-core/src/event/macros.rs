#![allow(clippy::needless_continue)]

use std::collections::HashMap;

use darling::{
    Error,
    FromDeriveInput,
};
use eventric_stream::event;
use proc_macro2::{
    TokenStream,
    TokenTree,
};
use quote::{
    ToTokens,
    TokenStreamExt,
    quote,
};
use syn::{
    DeriveInput,
    Ident,
    Meta,
    MetaList,
    parse::{
        Parse,
        ParseStream,
    },
};

use crate::macros::List;

// =================================================================================================
// Event
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(event), forward_attrs(allow, doc), supports(struct_named))]
pub(crate) struct Event {
    ident: Ident,
    #[darling(with = "identifier_parser")]
    identifier: String,
    tags: Option<HashMap<String, List<Tag>>>,
}

impl Event {
    pub fn new(input: &DeriveInput) -> Result<Self, Error> {
        Self::from_derive_input(input)
            .and_then(|event| Identified::validate(&event.identifier.clone(), event))
    }
}

impl ToTokens for Event {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(Identified::identifier(&self.ident, &self.identifier));
        tokens.append_all(Tagged::tags(&self.ident, self.tags.as_ref()));
    }
}

// -------------------------------------------------------------------------------------------------

// Identified

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(identified),
    forward_attrs(allow, doc),
    supports(struct_named)
)]
pub(crate) struct Identified {
    ident: Ident,
    #[darling(with = "identifier_parser")]
    identifier: String,
}

impl Identified {
    pub fn new(input: &DeriveInput) -> Result<Self, Error> {
        Self::from_derive_input(input)
            .and_then(|identifier| Identified::validate(&identifier.identifier.clone(), identifier))
    }
}

impl Identified {
    fn identifier(ident: &Ident, identifier: &str) -> TokenStream {
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

impl Identified {
    fn validate<T>(ident: &str, ok: T) -> Result<T, Error> {
        Self::validate_identifier(ident)?;

        Ok(ok)
    }

    fn validate_identifier(ident: &str) -> Result<(), Error> {
        event::Identifier::new(ident)
            .map(|_| ())
            .map_err(Error::custom)
    }
}

impl ToTokens for Identified {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(Self::identifier(&self.ident, &self.identifier));
    }
}

// Parser

fn identifier_parser(meta: &syn::Meta) -> darling::Result<String> {
    match meta {
        Meta::List(MetaList { tokens, .. }) => {
            let tokens = &tokens.clone().into_iter().collect::<Vec<_>>()[..];

            match tokens {
                [TokenTree::Ident(ident)] => Ok(ident.to_string()),
                _ => Err(darling::Error::unsupported_shape("identifier")),
            }
        }
        _ => Err(darling::Error::unexpected_type("name-value")),
    }
}

// -------------------------------------------------------------------------------------------------

// Tagged

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(tagged), forward_attrs(allow, doc), supports(struct_named))]
pub(crate) struct Tagged {
    ident: Ident,
    tags: Option<HashMap<String, List<Tag>>>,
}

impl Tagged {
    pub fn new(input: &DeriveInput) -> Result<Self, Error> {
        Self::from_derive_input(input)
    }
}

impl Tagged {
    fn tags(ident: &Ident, tags: Option<&HashMap<String, List<Tag>>>) -> TokenStream {
        let tag_type = quote! { eventric_stream::event::Tag };
        let error_type = quote! { eventric_stream::error::Error };

        let mut capacity = 0usize;

        let body = tags
            .as_ref()
            .map(|tags| {
                tags.iter()
                    .fold(TokenStream::new(), |mut tokens, (prefix, tags)| {
                        for tag in tags.as_ref() {
                            match tag {
                                Tag::Ident(ident) => tokens.append_all(quote! {
                                    tags.push(#tag_type::new(format!("{}:{}", #prefix, &self.#ident))?);
                                }),
                            }

                            capacity += 1;
                        }

                        tokens
                    })
            })
            .unwrap_or_default();

        quote! {
            impl eventric_surface::event::Tagged for #ident {
                fn tags(&self) -> Result<Vec<#tag_type>, #error_type> {
                    let mut tags = Vec::with_capacity(#capacity);

                    #body

                    Ok(tags)
                }
            }
        }
    }
}

impl ToTokens for Tagged {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(Self::tags(&self.ident, self.tags.as_ref()));
    }
}

// Tag

#[derive(Debug)]
pub enum Tag {
    Ident(Ident),
}

impl Parse for Tag {
    fn parse(stream: ParseStream<'_>) -> syn::Result<Self> {
        Ident::parse(stream).map(Self::Ident)
    }
}
