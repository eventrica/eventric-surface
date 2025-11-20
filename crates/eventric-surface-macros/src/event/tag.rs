#![allow(clippy::needless_continue)]

use std::collections::HashMap;

use darling::FromDeriveInput;
use proc_macro2::TokenStream;
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
    ExprClosure(ExprClosure),
    Ident(Ident),
}

impl Parse for TagDefinition {
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

// Tag Composites

pub struct IntoTagTokens<'a>(pub &'a Ident, pub &'a Ident, pub &'a TagDefinition);

impl ToTokens for IntoTagTokens<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let IntoTagTokens(ident, prefix, tag) = *self;

        let tag_macro = quote! { eventric_stream::event::tag };
        let identity_fn = quote! { std::convert::identity };
        let cow_type = quote! { std::borrow::Cow };

        match tag {
            TagDefinition::ExprClosure(expr) => tokens.append_all(quote! {
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
) -> Vec<IntoTagTokens<'a>> {
    tags.as_ref()
        .map(|tags| {
            tags.iter().fold(Vec::new(), |mut acc, (prefix, tags)| {
                for tag in tags.as_ref() {
                    acc.push(IntoTagTokens(ident, prefix, tag));
                }

                acc
            })
        })
        .unwrap_or_default()
}
