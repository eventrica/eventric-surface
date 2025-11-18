use std::collections::HashMap;

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    TokenStreamExt as _,
    quote,
};
use syn::{
    Ident,
    Path,
};

use crate::{
    event::{
        tag,
        tag::macros::Tag,
    },
    macros::List,
};

// =================================================================================================
// Query
// =================================================================================================

// Query

#[derive(Debug, FromMeta)]
pub struct Query {
    #[darling(multiple)]
    pub select: Vec<Selector>,
}

// Query Composites

pub struct IdentAndQuery<'a>(pub &'a Ident, pub &'a Query);

impl ToTokens for IdentAndQuery<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let IdentAndQuery(ident, query) = *self;

        let selector = query
            .select
            .iter()
            .map(|selector| IdentAndSelector(ident, selector));

        let query_type = quote! { eventric_stream::stream::query::Query };

        tokens.append_all(quote! {
            #query_type::new([#(#selector?),*])
        });
    }
}

// -------------------------------------------------------------------------------------------------

// Selector

#[derive(Debug, FromMeta)]
pub struct Selector {
    pub events: List<Path>,
    #[darling(map = "tag::macros::tags_map")]
    filter: Option<HashMap<Ident, List<Tag>>>,
}

// Selector Composites

pub struct IdentAndSelector<'a>(pub &'a Ident, pub &'a Selector);

impl ToTokens for IdentAndSelector<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let IdentAndSelector(ident, selector) = *self;

        let event = selector.events.as_ref();
        let tag = tag::macros::tags_fold(ident, selector.filter.as_ref());

        let selector_type = quote! { eventric_stream::stream::query::Selector };

        if tag.is_empty() {
            tokens.append_all(quote! {
                #selector_type::specifiers(
                    [#(#event::specifier()?),*]
                )
            });
        } else {
            tokens.append_all(quote! {
                #selector_type::specifiers_and_tags(
                    [#(#event::specifier()?),*],
                    [#(#tag?),*]
                )
            });
        }
    }
}
