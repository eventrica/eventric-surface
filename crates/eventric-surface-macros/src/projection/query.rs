#![allow(clippy::needless_continue)]

use std::collections::HashMap;

use darling::{
    FromDeriveInput,
    FromMeta,
};
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    TokenStreamExt as _,
    quote,
};
use syn::{
    DeriveInput,
    Ident,
    Path,
};

use crate::{
    event::{
        tag,
        tag::TagDefinition,
    },
    util::List,
};

// =================================================================================================
// Query
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(query), supports(struct_named))]
pub struct QueryDerive {
    ident: Ident,
    #[darling(multiple)]
    select: Vec<SelectorDefinition>,
}

impl QueryDerive {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl QueryDerive {
    #[must_use]
    pub fn query(ident: &Ident, selectors: &Vec<SelectorDefinition>) -> TokenStream {
        let query = IntoQueryTokens(ident, selectors);

        let query_type = quote! { eventric_stream::stream::query::Query };
        let error_type = quote! { eventric_stream::error::Error };

        quote! {
            impl eventric_surface::projection::Query for #ident {
                fn query(&self) -> Result<#query_type, #error_type> {
                    #query
                }
            }
        }
    }
}

impl ToTokens for QueryDerive {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(QueryDerive::query(&self.ident, &self.select));
    }
}

// -------------------------------------------------------------------------------------------------

// Selector Definition

#[derive(Debug, FromMeta)]
pub struct SelectorDefinition {
    pub events: List<Path>,
    #[darling(map = "tag::map")]
    pub filter: Option<HashMap<Ident, List<TagDefinition>>>,
}

// Selector Definition Composites

struct IntoQueryTokens<'a>(pub &'a Ident, pub &'a Vec<SelectorDefinition>);

impl ToTokens for IntoQueryTokens<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let IntoQueryTokens(ident, selectors) = *self;

        let selector = selectors
            .iter()
            .map(|selector| IntoSelectorTokens(ident, selector));

        let query_type = quote! { eventric_stream::stream::query::Query };

        tokens.append_all(quote! {
            #query_type::new([#(#selector?),*])
        });
    }
}

struct IntoSelectorTokens<'a>(pub &'a Ident, pub &'a SelectorDefinition);

impl ToTokens for IntoSelectorTokens<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let IntoSelectorTokens(ident, selector) = *self;

        let event = selector.events.as_ref();
        let tag = tag::fold(ident, selector.filter.as_ref());

        let selector_type = quote! { eventric_stream::stream::query::Selector };
        let specifier_trait = quote! { eventric_surface::event::Specifier };

        if tag.is_empty() {
            tokens.append_all(quote! {
                #selector_type::specifiers(
                    [#(<#event as #specifier_trait>::specifier()?),*]
                )
            });
        } else {
            tokens.append_all(quote! {
                #selector_type::specifiers_and_tags(
                    [#(<#event as #specifier_trait>::specifier()?),*],
                    [#(#tag?),*]
                )
            });
        }
    }
}
