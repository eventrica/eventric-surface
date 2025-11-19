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
    stream::query::Query,
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
        tag::TagValueSource,
    },
    macros::List,
};

// =================================================================================================
// Query
// =================================================================================================

pub trait QuerySource {
    fn query(&self) -> Result<Query, Error>;
}

// =================================================================================================
// Query Macros
// =================================================================================================

// Query Source

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(query_source), supports(struct_named))]
pub struct QuerySourceDerive {
    ident: Ident,
    query: QueryDefinition,
}

impl QuerySourceDerive {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl QuerySourceDerive {
    pub fn query_source(ident: &Ident, query: &QueryDefinition) -> TokenStream {
        let query = IdentAndQueryDefinition(ident, query);

        let query_type = quote! { eventric_stream::stream::query::Query };
        let error_type = quote! { eventric_stream::error::Error };

        quote! {
            impl eventric_surface::projection::QuerySource for #ident {
                fn query(&self) -> Result<#query_type, #error_type> {
                    #query
                }
            }
        }
    }
}

impl ToTokens for QuerySourceDerive {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(QuerySourceDerive::query_source(&self.ident, &self.query));
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
    pub fn events(&self) -> HashSet<&Path> {
        self.select.iter().flat_map(|s| s.events.as_ref()).collect()
    }
}

// Query Composites

pub struct IdentAndQueryDefinition<'a>(pub &'a Ident, pub &'a QueryDefinition);

impl ToTokens for IdentAndQueryDefinition<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let IdentAndQueryDefinition(ident, query) = *self;

        let selector = query
            .select
            .iter()
            .map(|selector| IdentAndSelectorDefinition(ident, selector));

        let query_type = quote! { eventric_stream::stream::query::Query };

        tokens.append_all(quote! {
            #query_type::new([#(#selector?),*])
        });
    }
}

// -------------------------------------------------------------------------------------------------

// Selector Definition

#[derive(Debug, FromMeta)]
pub struct SelectorDefinition {
    pub events: List<Path>,
    #[darling(map = "tag::map")]
    filter: Option<HashMap<Ident, List<TagValueSource>>>,
}

// Selector Definition Composites

pub struct IdentAndSelectorDefinition<'a>(pub &'a Ident, pub &'a SelectorDefinition);

impl ToTokens for IdentAndSelectorDefinition<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let IdentAndSelectorDefinition(ident, selector) = *self;

        let event = selector.events.as_ref();
        let tag = tag::fold(ident, selector.filter.as_ref());

        let selector_type = quote! { eventric_stream::stream::query::Selector };
        let specified_trait = quote! { eventric_surface::event::Specified };

        if tag.is_empty() {
            tokens.append_all(quote! {
                #selector_type::specifiers(
                    [#(<#event as #specified_trait>::specifier()?),*]
                )
            });
        } else {
            tokens.append_all(quote! {
                #selector_type::specifiers_and_tags(
                    [#(<#event as #specified_trait>::specifier()?),*],
                    [#(#tag?),*]
                )
            });
        }
    }
}
