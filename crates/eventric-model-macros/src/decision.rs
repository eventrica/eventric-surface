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
// Decision
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(decision), supports(struct_named))]
pub struct Decision {
    ident: Ident,
    #[darling(multiple, rename = "projection")]
    projections: Vec<Projection>,
}

impl Decision {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl Decision {
    pub fn decision(&self) -> TokenStream {
        let ident = &self.ident;

        quote! {
            impl eventric_model::decision::Decision for #ident {}
        }
    }

    fn projections(&self) -> TokenStream {
        let ident = &self.ident;
        let projections = &self.projections;

        let proj_type = format_ident!("{ident}Projections");

        let proj_field_name = projections.iter().map(|p| &p.field_name);
        let proj_field_type = projections.iter().map(|p| &p.field_type);
        let proj_init = projections
            .iter()
            .map(|proj| ProjectionInitializer(ident, proj));

        quote! {
            impl ::eventric_model::decision::Projections for #ident {
                type Projections = #proj_type;

                fn projections(&self) -> Self::Projections {
                    Self::Projections::new(self)
                }
            }

            #[derive(Debug)]
            pub struct #proj_type {
                #(pub #proj_field_name: #proj_field_type),*
            }

            impl #proj_type {
                fn new(decision: &#ident) -> Self {
                    Self {
                        #(#proj_init),*
                    }
                }
            }
        }
    }

    fn select(&self) -> TokenStream {
        let ident = &self.ident;
        let projections = &self.projections;

        let proj_field_name = projections.iter().map(|p| &p.field_name);

        quote! {
            impl ::eventric_model::decision::Select for #ident {
                fn select(
                    &self,
                    projections: &Self::Projections
                ) -> ::std::result::Result<
                    ::eventric_stream::stream::select::Selections,
                    ::eventric_stream::error::Error
                > {
                    ::eventric_stream::stream::select::Selections::new([
                        #(::eventric_model::projection::Select::select(&projections.#proj_field_name)?),*
                    ])
                }
            }
        }
    }

    fn update(&self) -> TokenStream {
        let ident = &self.ident;
        let projections = &self.projections;

        let proj_field_name = projections.iter().map(|p| &p.field_name);
        let proj_index = 0..projections.len();

        quote! {
            impl ::eventric_model::decision::Update for #ident {
                fn update<C>(
                    &self,
                    codec: &C,
                    event: &::eventric_stream::stream::select::EventMasked,
                    projections: &mut Self::Projections
                ) -> ::std::result::Result<(), ::eventric_stream::error::Error>
                where
                    C: ::eventric_model::event::Codec,
                {
                    let mut dispatch_event = None;

                    #({
                        if event.mask()[#proj_index] && dispatch_event.is_none() {
                            dispatch_event = ::eventric_model::projection::Recognize::recognize(
                                &projections.#proj_field_name,
                                codec,
                                event,
                            )?;
                        }

                        if event.mask()[#proj_index] && let Some(dispatch_event) = dispatch_event {
                            ::eventric_model::projection::Dispatch::dispatch(
                                &mut projections.#proj_field_name,
                                &dispatch_event,
                            );
                        }
                    })*

                    Ok(())
                }
            }
        }
    }
}

impl ToTokens for Decision {
    #[rustfmt::skip]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(self.decision());
        tokens.append_all(self.projections());
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
            decision_type,
            Projection {
                field_name,
                field_type,
                initializer,
            },
        ) = *self;

        tokens.append_all(quote! {
            #field_name: ::std::convert::identity::<fn(&#decision_type) -> #field_type>(#initializer)(decision)
        });
    }
}
