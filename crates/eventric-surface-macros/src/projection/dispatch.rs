#![allow(clippy::needless_continue)]

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
    Ident,
    Path,
};

use crate::util::List;

// =================================================================================================
// Dispatch
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(dispatch), supports(struct_named))]
pub struct DispatchDerive {
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
