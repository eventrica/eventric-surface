#![allow(clippy::needless_continue)]

use std::collections::{
    HashMap,
    HashSet,
};

use darling::{
    FromDeriveInput,
    FromMeta,
};
use eventric_stream::{
    error::Error,
    stream::query,
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
    macros::List,
};

// =================================================================================================
// Query
// =================================================================================================

pub trait Query {
    fn query(&self) -> Result<query::Query, Error>;
}

// =================================================================================================
// Query Macros
// =================================================================================================

// Queried

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
        let query = IdentAndSelectorDefinitions(ident, selectors);

        let query_type = quote! { eventric_stream::stream::query::Query };
        let error_type = quote! { eventric_stream::error::Error };

        quote! {
            impl eventric_surface::projection::query::Query for #ident {
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

// Query Definition

#[derive(Debug, FromMeta)]
pub struct QueryDefinition {
    #[darling(multiple)]
    pub select: Vec<SelectorDefinition>,
}

impl QueryDefinition {
    #[must_use]
    pub fn events(&self) -> Vec<Path> {
        self.select
            .iter()
            .flat_map(|selector| selector.events.as_ref())
            .cloned()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }
}

// -------------------------------------------------------------------------------------------------

// Selector Definition

#[derive(Debug, FromMeta)]
pub struct SelectorDefinition {
    events: List<Path>,
    #[darling(map = "tag::map")]
    filter: Option<HashMap<Ident, List<TagDefinition>>>,
}

// Selector Definition Composites

pub struct IdentAndSelectorDefinitions<'a>(pub &'a Ident, pub &'a Vec<SelectorDefinition>);

impl ToTokens for IdentAndSelectorDefinitions<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let IdentAndSelectorDefinitions(ident, selectors) = *self;

        let selector = selectors
            .iter()
            .map(|selector| IdentAndSelectorDefinition(ident, selector));

        let query_type = quote! { eventric_stream::stream::query::Query };

        tokens.append_all(quote! {
            #query_type::new([#(#selector?),*])
        });
    }
}

struct IdentAndSelectorDefinition<'a>(pub &'a Ident, pub &'a SelectorDefinition);

impl ToTokens for IdentAndSelectorDefinition<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let IdentAndSelectorDefinition(ident, selector) = *self;

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
