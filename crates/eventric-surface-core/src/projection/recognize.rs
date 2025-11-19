use darling::FromDeriveInput;
use eventric_stream::{
    error::Error,
    event::PersistentEvent,
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
    event::codec::Codec,
    macros::List,
    projection::dispatch::DispatchEvent,
};

// =================================================================================================
// Recognise
// =================================================================================================

pub trait Recognize {
    fn recognize<C>(codec: &C, event: &PersistentEvent) -> Result<Option<DispatchEvent>, Error>
    where
        C: Codec;
}

// =================================================================================================
// Recognise Macros
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(recognize), supports(struct_named))]
pub(crate) struct RecognizeDerive {
    ident: Ident,
    events: List<Path>,
}

impl RecognizeDerive {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl RecognizeDerive {
    #[must_use]
    pub fn recognize(ident: &Ident, event: &Vec<Path>) -> TokenStream {
        let codec_trait = quote! {eventric_surface::event::Codec };
        let identifier_trait = quote! { eventric_surface::event::Identifier };

        let dispatch_event_type = quote! { eventric_surface::projection::DispatchEvent };
        let error_type = quote! { eventric_stream::error::Error };
        let persistent_event_type = quote! { eventric_stream::event::PersistentEvent };

        quote! {
            impl eventric_surface::projection::Recognize for #ident {
                fn recognize<C>(codec: &C, event: &#persistent_event_type) -> Result<Option<#dispatch_event_type>, #error_type>
                where
                    C: #codec_trait,
                {
                    let event = match event {
                      #(_ if event.identifier() == <#event as #identifier_trait>::identifier()? => {
                            Some(#dispatch_event_type::from_persistent_event::<C, #event>(codec, event)?)
                        }),*
                        _ => None,
                    };

                    Ok(event)
                }
            }
        }
    }
}

impl ToTokens for RecognizeDerive {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(Self::recognize(&self.ident, self.events.as_ref()));
    }
}
