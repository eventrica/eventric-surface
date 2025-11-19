#![allow(clippy::needless_continue)]

use std::collections::HashMap;

use darling::FromDeriveInput;
use proc_macro2::{
    Span,
    TokenStream,
};
use quote::{
    ToTokens,
    TokenStreamExt as _,
    format_ident,
    quote,
};
use syn::{
    DeriveInput,
    ExprClosure,
    Ident,
    parse::{
        Parse,
        ParseStream,
    },
};

use crate::util::List;

// =================================================================================================
// Tag
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(tags), supports(struct_named))]
pub struct TagsDerive {
    ident: Ident,
    #[darling(map = "map")]
    tags: Option<HashMap<Ident, List<TagDefinition>>>,
}

impl TagsDerive {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl TagsDerive {
    #[must_use]
    pub fn tags(ident: &Ident, tags: Option<&HashMap<Ident, List<TagDefinition>>>) -> TokenStream {
        let tag = fold(ident, tags);
        let tag_count = tag.len();

        let tag_type = quote! { eventric_stream::event::Tag };
        let error_type = quote! { eventric_stream::error::Error };

        quote! {
            impl eventric_surface::event::Tags for #ident {
                fn tags(&self) -> Result<Vec<#tag_type>, #error_type> {
                    let mut tags = Vec::with_capacity(#tag_count);

                  #(tags.push(#tag?);)*

                    Ok(tags)
                }
            }
        }
    }
}

impl ToTokens for TagsDerive {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(TagsDerive::tags(&self.ident, self.tags.as_ref()));
    }
}

// -------------------------------------------------------------------------------------------------

// Tag

#[derive(Debug)]
pub enum TagDefinition {
    Expr(ExprClosure),
    Ident(Ident),
}

impl Parse for TagDefinition {
    fn parse(stream: ParseStream<'_>) -> syn::Result<Self> {
        if let Ok(mut expr) = ExprClosure::parse(stream) {
            let body = &expr.body;
            let body = syn::parse(quote! { { #body }.into() }.into())?;

            *expr.body = body;

            return Ok(Self::Expr(expr));
        }

        if let Ok(ident) = Ident::parse(stream) {
            return Ok(Self::Ident(ident));
        }

        Err(syn::Error::new(Span::call_site(), "Unexpected Tag Format"))
    }
}

// Tag Composites

pub struct IdentPrefixAndTagDefinition<'a>(pub &'a Ident, pub &'a Ident, pub &'a TagDefinition);

impl ToTokens for IdentPrefixAndTagDefinition<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let IdentPrefixAndTagDefinition(ident, prefix, tag) = *self;

        let tag_macro = quote! { eventric_stream::event::tag };
        let identity_fn = quote! { std::convert::identity };
        let cow_type = quote! { std::borrow::Cow };

        match tag {
            TagDefinition::Expr(expr) => tokens.append_all(quote! {
                #tag_macro!(#prefix, #identity_fn::<for<'a> fn(&'a #ident) -> #cow_type<'a, _>>(#expr)(&self))
            }),
            TagDefinition::Ident(ident) => tokens.append_all(quote! {
                #tag_macro!(#prefix, &self.#ident)
            }),
        }
    }
}

// -------------------------------------------------------------------------------------------------

// Tag Functions

pub fn map(
    tags: Option<HashMap<String, List<TagDefinition>>>,
) -> Option<HashMap<Ident, List<TagDefinition>>> {
    tags.map(|tags| {
        tags.into_iter()
            .map(|(prefix, tags)| (format_ident!("{prefix}"), tags))
            .collect()
    })
}

pub fn fold<'a>(
    ident: &'a Ident,
    tags: Option<&'a HashMap<Ident, List<TagDefinition>>>,
) -> Vec<IdentPrefixAndTagDefinition<'a>> {
    tags.as_ref()
        .map(|tags| {
            tags.iter().fold(Vec::new(), |mut acc, (prefix, tags)| {
                for tag in tags.as_ref() {
                    acc.push(IdentPrefixAndTagDefinition(ident, prefix, tag));
                }

                acc
            })
        })
        .unwrap_or_default()
}
