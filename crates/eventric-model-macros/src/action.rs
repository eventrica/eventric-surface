#![allow(clippy::needless_continue)]

use darling::{
    FromDeriveInput,
    FromMeta,
};
use heck::AsSnakeCase;
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
    Meta,
    Path,
    parse::{
        Parse,
        ParseStream,
    },
    token::{
        At,
        Colon,
    },
};

// =================================================================================================
// Action
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(action), supports(struct_named))]
pub struct Action {
    ident: Ident,
    #[darling(multiple, rename = "projection")]
    projections: Vec<Projection>,
}

impl Action {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl Action {
    fn action(&self) -> TokenStream {
        let ident = &self.ident;

        quote! {
            #[automatically_derived]
            impl ::eventric_model::action::Action for #ident {}
        }
    }

    fn context(&self) -> TokenStream {
        let ident = &self.ident;
        let projections = &self.projections;

        let context_type = format_ident!("{ident}Context");

        let context_field_name = projections.iter().map(|p| &p.field_name);
        let context_field_type = projections.iter().map(|p| &p.field_type);
        let context_field_init = projections
            .iter()
            .map(|proj| ProjectionInitializer(ident, proj));

        quote! {
            #[automatically_derived]
            impl ::eventric_model::action::Context for #ident {
                type Context = #context_type;

                fn context(&self) -> Self::Context {
                    Self::Context::new(self)
                }
            }

            #[derive(Debug)]
            pub struct #context_type {
                pub events: eventric_model::action::Events,
                #(pub #context_field_name: #context_field_type),*
            }

            #[automatically_derived]
            impl ::std::ops::Deref for #context_type {
                type Target = eventric_model::action::Events;

                fn deref(&self) -> &Self::Target {
                    &self.events
                }
            }

            #[automatically_derived]
            impl ::std::ops::DerefMut for #context_type {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.events
                }
            }

            #[automatically_derived]
            impl ::core::convert::Into<::eventric_model::action::Events> for #context_type {
                fn into(self) -> ::eventric_model::action::Events {
                    self.events
                }
            }

            impl #context_type {
                pub fn new(action: &#ident) -> Self {
                    Self {
                        events: eventric_model::action::Events::new(),
                        #(#context_field_init),*
                    }
                }
            }
        }
    }

    fn select(&self) -> TokenStream {
        let ident = &self.ident;
        let projections = &self.projections;

        let context_field_name = projections.iter().map(|p| &p.field_name);

        quote! {
            #[automatically_derived]
            impl ::eventric_model::action::Select for #ident {
                fn select(
                    &self,
                    context: &Self::Context
                ) -> ::std::result::Result<
                    ::eventric_stream::stream::select::Selections,
                    ::eventric_stream::error::Error
                > {
                    ::eventric_stream::stream::select::Selections::new([
                        #(::eventric_model::projection::Select::select(&context.#context_field_name)?),*
                    ])
                }
            }
        }
    }

    fn update(&self) -> TokenStream {
        let ident = &self.ident;
        let projections = &self.projections;

        let context_field_name = projections.iter().map(|p| &p.field_name);
        let context_field_index = 0..projections.len();

        quote! {
            #[automatically_derived]
            impl ::eventric_model::action::Update for #ident {
                fn update(
                    &self,
                    context: &mut Self::Context,
                    event: &::eventric_stream::stream::select::EventMasked
                ) -> ::std::result::Result<(), ::eventric_stream::error::Error> {
                    let mut dispatch_event = None;

                    #({
                        if event.mask()[#context_field_index] && dispatch_event.is_none() {
                            dispatch_event = ::eventric_model::projection::Recognize::recognize(
                                &context.#context_field_name,
                                event,
                            )?;
                        }

                        if event.mask()[#context_field_index] && let Some(dispatch_event) = dispatch_event.as_ref() {
                            ::eventric_model::projection::Dispatch::dispatch(
                                &mut context.#context_field_name,
                                dispatch_event,
                            );
                        }
                    })*

                    Ok(())
                }
            }
        }
    }
}

impl ToTokens for Action {
    #[rustfmt::skip]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.action());
        tokens.append_all(self.context());
        tokens.append_all(self.select());
        tokens.append_all(self.update());
    }
}

// -------------------------------------------------------------------------------------------------

// Projection

#[derive(Debug)]
pub struct Projection {
    pub field_name: Ident,
    pub field_type: Path,
    pub initializer: ExprClosure,
}

impl FromMeta for Projection {
    fn from_meta(meta: &Meta) -> darling::Result<Self> {
        let list = meta.require_list()?;
        let input = list.tokens.clone();

        syn::parse2(input).map_err(darling::Error::custom)
    }
}

impl Parse for Projection {
    #[allow(clippy::match_bool, clippy::single_match_else)]
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let field_type = Path::parse(input)?;

        let field_name = match input.peek(At) {
            true => At::parse(input).and_then(|_| Ident::parse(input))?,
            _ => {
                let segment = field_type.segments.last().expect("ident");
                let ident = segment.ident.to_string();

                format_ident!("{}", AsSnakeCase(ident).to_string())
            }
        };

        let _ = Colon::parse(input)?;

        let initializer = match ExprClosure::parse(input) {
            Ok(expr) => expr,
            _ => Expr::parse(input).and_then(|expr| syn::parse2(quote! { |this| #expr }))?,
        };

        Ok(Self {
            field_name,
            field_type,
            initializer,
        })
    }
}

// Projection Composites

pub struct ProjectionInitializer<'a>(&'a Ident, &'a Projection);

impl ToTokens for ProjectionInitializer<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ProjectionInitializer(
            action_type,
            Projection {
                field_name,
                field_type,
                initializer,
            },
        ) = *self;

        tokens.append_all(quote! {
            #field_name: ::std::convert::identity::<fn(&#action_type) -> #field_type>(#initializer)(action)
        });
    }
}
