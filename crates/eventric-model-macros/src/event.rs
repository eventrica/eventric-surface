#![allow(clippy::needless_continue)]

use std::collections::HashMap;

use darling::FromDeriveInput;
use eventric_stream::event::Identifier;
use proc_macro2::{
    TokenStream,
    TokenTree,
};
use quote::{
    ToTokens,
    TokenStreamExt as _,
    format_ident,
    quote,
};
use syn::{
    DeriveInput,
    Expr,
    ExprClosure,
    Ident,
    Meta,
    parse::{
        Parse,
        ParseStream,
    },
};

use crate::util::List;

// =================================================================================================
// Event
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(event), supports(struct_named))]
pub struct Event {
    ident: Ident,
    #[darling(with = "parse_identifier")]
    identifier: String,
    #[darling(map = "tags_map")]
    tags: Option<HashMap<Ident, List<Tag>>>,
}

impl Event {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input).and_then(Event::validate)
    }
}

impl Event {
    fn event(&self) -> TokenStream {
        let ident = &self.ident;

        quote! {
            impl ::eventric_model::event::Event for #ident {}
        }
    }

    fn identifier(&self) -> TokenStream {
        let event_type = &self.ident;
        let identifier = &self.identifier;

        quote! {
            impl ::eventric_model::event::Identifier for #event_type {
                fn identifier() -> ::std::result::Result<
                    &'static ::eventric_stream::event::Identifier,
                    ::eventric_stream::error::Error
                > {
                    static IDENTIFIER: ::std::sync::OnceLock<eventric_stream::event::Identifier> = ::std::sync::OnceLock::new();

                    IDENTIFIER.get_or_try_init(|| ::eventric_stream::event::Identifier::new(#identifier))
                }
            }
        }
    }

    fn tags(&self) -> TokenStream {
        let ident = &self.ident;
        let tags = self.tags.as_ref();

        let tag = tags_fold(ident, tags);
        let tag_count = tag.len();

        quote! {
            impl ::eventric_model::event::Tags for #ident {
                fn tags(&self) -> ::std::result::Result<
                    ::std::vec::Vec<::eventric_stream::event::Tag>,
                    ::eventric_stream::error::Error
                > {
                    let mut tags = ::std::vec::Vec::with_capacity(#tag_count);

                  #(tags.push(#tag?);)*

                    Ok(tags)
                }
            }
        }
    }
}

impl Event {
    fn validate(self) -> darling::Result<Self> {
        self.validate_identifier().map(|()| self)
    }

    fn validate_identifier(&self) -> darling::Result<()> {
        Identifier::new(&self.identifier)
            .map(|_| ())
            .map_err(darling::Error::custom)
    }
}

impl ToTokens for Event {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.event());
        tokens.append_all(self.identifier());
        tokens.append_all(self.tags());
    }
}

// -------------------------------------------------------------------------------------------------

// Identifier

pub fn parse_identifier(meta: &Meta) -> darling::Result<String> {
    let identifier = meta.require_list()?;
    let identifier = identifier.tokens.clone().into_iter().collect::<Vec<_>>();

    match &identifier[..] {
        [TokenTree::Ident(ident)] => Ok(ident.to_string()),
        _ => Err(darling::Error::unsupported_shape("identifier")),
    }
}

// -------------------------------------------------------------------------------------------------

// Tag

#[derive(Debug)]
pub enum Tag {
    ExprClosure(ExprClosure),
    Ident(Ident),
}

impl Parse for Tag {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if let Ok(mut expr) = ExprClosure::parse(input) {
            let body = &expr.body;
            let body = syn::parse2(quote! { { #body }.into() })?;

            *expr.body = body;

            return Ok(Self::ExprClosure(expr));
        }

        if let Ok(ident) = Ident::parse(input) {
            return Ok(Self::Ident(ident));
        }

        Expr::parse(input).and_then(|expr| {
            Ok(Self::ExprClosure(syn::parse2(
                quote! { |this| { #expr }.into() },
            )?))
        })
    }
}

// Composites

pub struct TagInitialize<'a>(pub &'a Ident, pub &'a Ident, pub &'a Tag);

impl ToTokens for TagInitialize<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let TagInitialize(ident, prefix, tag) = *self;

        match tag {
            Tag::ExprClosure(expr) => tokens.append_all(quote! {
                ::eventric_stream::event::tag!(
                    #prefix,
                    ::std::convert::identity::<for<'a> fn(&'a #ident) -> ::std::borrow::Cow<'a, _>>(#expr)(&self)
                )
            }),
            Tag::Ident(ident) => tokens.append_all(quote! {
                ::eventric_stream::event::tag!(
                    #prefix,
                    &self.#ident
                )
            }),
        }
    }
}

// Functions

pub fn tags_map(tags: Option<HashMap<String, List<Tag>>>) -> Option<HashMap<Ident, List<Tag>>> {
    tags.map(|tags| {
        tags.into_iter()
            .map(|(prefix, tags)| (format_ident!("{prefix}"), tags))
            .collect()
    })
}

pub fn tags_fold<'a>(
    ident: &'a Ident,
    tags: Option<&'a HashMap<Ident, List<Tag>>>,
) -> Vec<TagInitialize<'a>> {
    tags.as_ref()
        .map(|tags| {
            tags.iter().fold(Vec::new(), |mut acc, (prefix, tags)| {
                for tag in tags.as_ref() {
                    acc.push(TagInitialize(ident, prefix, tag));
                }

                acc
            })
        })
        .unwrap_or_default()
}
