#![allow(clippy::needless_continue)]

pub(crate) mod query;

use std::any::Any;

use darling::FromDeriveInput;
use eventric_stream::{
    error::Error,
    event::PersistentEvent,
};
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    TokenStreamExt as _,
    format_ident,
    quote,
};
use syn::{
    DeriveInput,
    Ident,
};

use crate::{
    event::{
        Codec,
        Event,
    },
    projection::query::{
        QueryDefinition,
        QuerySourceDerive,
    },
};

// =================================================================================================
// Projection
// =================================================================================================

// Projection

pub trait Projection: QuerySource {}

// Dispatch

pub trait Dispatch {
    fn dispatch(&mut self, event: &Box<dyn Any>);
}

// Recognise

pub trait Recognize {
    fn recognize<C>(codec: &C, event: &PersistentEvent) -> Result<Option<Box<dyn Any>>, Error>
    where
        C: Codec;
}

// Update

pub trait Update<E>
where
    E: Event,
{
    fn update(&mut self, event: &E);
}

// =================================================================================================
// Projection Macros
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(projection), supports(struct_named))]
pub struct ProjectionDerive {
    ident: Ident,
    query: QueryDefinition,
}

impl ProjectionDerive {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl ProjectionDerive {
    fn projection(ident: &Ident) -> TokenStream {
        quote! {
            impl eventric_surface::projection::Projection for #ident {}
        }
    }

    fn update(ident: &Ident, query: &QueryDefinition) -> TokenStream {
        let event = query.events().into_iter();
        let ident_update_trait = format_ident!("{ident}Update");

        let update_trait = quote! { eventric_surface::projection::Update };

        quote! {
            trait #ident_update_trait: #(#update_trait<#event>)+* {}

            impl #ident_update_trait for #ident {}
        }
    }
}

impl ToTokens for ProjectionDerive {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(ProjectionDerive::projection(&self.ident));
        tokens.append_all(ProjectionDerive::update(&self.ident, &self.query));
        tokens.append_all(QuerySourceDerive::query_source(&self.ident, &self.query));
    }
}

// -------------------------------------------------------------------------------------------------

// Re-Exports

pub use self::query::QuerySource;
