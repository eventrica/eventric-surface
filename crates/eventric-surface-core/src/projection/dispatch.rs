use std::any::Any;

use darling::FromDeriveInput;
use eventric_stream::{
    error::Error,
    event::{
        PersistentEvent,
        Position,
        Timestamp,
    },
};
use fancy_constructor::new;
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
    Path,
};

use crate::{
    event::{
        Codec,
        Event,
    },
    macros::List,
    projection::update::UpdateEvent,
};

// =================================================================================================
// Dispatch
// =================================================================================================

pub trait Dispatch {
    fn dispatch(&mut self, event: &DispatchEvent);
}

// -------------------------------------------------------------------------------------------------

// Event

#[derive(new, Debug)]
#[new(const_fn, vis(pub(crate)))]
pub struct DispatchEvent {
    event: Box<dyn Any>,
    position: Position,
    timestamp: Timestamp,
}

impl DispatchEvent {
    #[must_use]
    pub fn as_update_event<E>(&self) -> Option<UpdateEvent<'_, E>>
    where
        E: Event + 'static,
    {
        self.event
            .downcast_ref()
            .map(|inner_event| UpdateEvent::new(inner_event, self.position, self.timestamp))
    }

    pub fn from_persistent_event<C, E>(codec: &C, event: &PersistentEvent) -> Result<Self, Error>
    where
        C: Codec,
        E: Event + 'static,
    {
        codec
            .decode::<E>(event)
            .map(|inner_event| Box::new(inner_event) as Box<dyn Any>)
            .map(|inner_event| Self::new(inner_event, *event.position(), *event.timestamp()))
    }
}

// =================================================================================================
// Dispatch Macros
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(dispatch), supports(struct_named))]
pub(crate) struct DispatchDerive {
    ident: Ident,
    events: List<Path>,
}

impl DispatchDerive {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl DispatchDerive {
    #[must_use]
    pub fn dispatch(ident: &Ident, event: &Vec<Path>) -> TokenStream {
        let dispatch_trait = format_ident!("Dispatch{ident}");

        let dispatch_event_type = quote! { eventric_surface::projection::DispatchEvent };
        let update_trait = quote! { eventric_surface::projection::Update };

        quote! {
            trait #dispatch_trait: #(#update_trait<#event>)+* {}

            impl #dispatch_trait for #ident {}

            impl eventric_surface::projection::Dispatch for #ident {
                fn dispatch(&mut self, event: &#dispatch_event_type) {
                    match event {
                      #(_ if let Some(event) = event.as_update_event::<#event>() => self.update(event),)*
                        _ => {}
                    }
                }
            }
        }
    }
}

impl ToTokens for DispatchDerive {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(Self::dispatch(&self.ident, self.events.as_ref()));
    }
}
