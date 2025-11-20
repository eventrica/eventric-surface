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
pub struct DecisionDerive {
    ident: Ident,
    #[darling(multiple)]
    projection: Vec<ProjectionDefinition>,
}

impl DecisionDerive {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl DecisionDerive {
    pub fn projections(ident: &Ident, projections: &[ProjectionDefinition]) -> TokenStream {
        let projections_type = format_ident!("{ident}Projections");

        let projection_expr = projections.iter().map(|p| &p.expr);
        let projection_ident = projections.iter().map(|p| &p.ident).collect::<Vec<_>>();
        let projection_path = projections.iter().map(|p| &p.path).collect::<Vec<_>>();

        let identity_fn = quote! { std::convert::identity };

        quote! {
            #[derive(Debug)]
            pub struct #projections_type {
                #(pub #projection_ident: #projection_path),*
            }

            impl #projections_type {
                fn new(decision: &#ident) -> Self {
                    Self {
                        #(#projection_ident: #identity_fn::<fn(&#ident) -> #projection_path>(#projection_expr)(decision)),*
                    }
                }
            }
        }
    }
}

impl ToTokens for DecisionDerive {
    #[rustfmt::skip]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(DecisionDerive::projections(&self.ident, &self.projection));
    }
}

// -------------------------------------------------------------------------------------------------

// Projection Definition

#[derive(Debug)]
pub struct ProjectionDefinition {
    expr: ExprClosure,
    ident: Ident,
    path: Path,
}

impl FromMeta for ProjectionDefinition {
    fn from_meta(meta: &Meta) -> darling::Result<Self> {
        let list = meta.require_list()?;
        let list = list.tokens.clone();

        syn::parse2(list).map_err(darling::Error::custom)
    }
}

impl Parse for ProjectionDefinition {
    #[rustfmt::skip]
    #[allow(clippy::match_bool)]
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let path = Path::parse(input)?;

        let ident = match input.peek(At) {
            true => At::parse(input).and_then(|_| Ident::parse(input))?,
            _ => format_ident!("{}", AsSnakeCase(path.segments.last().expect("ident").ident.to_string()).to_string()),
        };

        let _ = Colon::parse(input)?;

        let expr = match ExprClosure::parse(input) {
            Ok(expr) => expr,
            _ => Expr::parse(input).and_then(|expr| syn::parse2(quote! { |this| #expr }))?,
        };

        Ok(Self { expr, ident, path })
    }
}
