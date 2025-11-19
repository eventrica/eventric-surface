use darling::FromMeta;
use syn::{
    Meta,
    parse::{
        Parse,
        ParseStream,
    },
    punctuated::Punctuated,
    token::Comma,
};

// =================================================================================================
// Macros
// =================================================================================================

// List

#[derive(Debug, Clone)]
pub(crate) struct List<T>(Vec<T>)
where
    T: Parse;

impl<T> AsRef<Vec<T>> for List<T>
where
    T: Parse,
{
    fn as_ref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> FromMeta for List<T>
where
    T: Parse,
{
    fn from_meta(meta: &Meta) -> darling::Result<Self> {
        let list = meta.require_list()?;
        let list = list.tokens.clone();

        syn::parse2::<List<T>>(list).map_err(darling::Error::custom)
    }
}

impl<T> Parse for List<T>
where
    T: Parse,
{
    fn parse(stream: ParseStream<'_>) -> syn::Result<Self> {
        let list = Punctuated::<T, Comma>::parse_terminated(stream)?;
        let list = list.into_iter().collect();
        let list = Self(list);

        Ok(list)
    }
}

// =================================================================================================
// Macros Derive
// =================================================================================================

pub mod derive {
    use proc_macro2::TokenStream;
    use quote::ToTokens;
    use syn::DeriveInput;

    use crate::{
        event,
        projection,
    };

    macro_rules! emit_impl_or_error {
        ($e:expr) => {
            match $e {
                Ok(val) => val.into_token_stream(),
                Err(err) => err.write_errors(),
            }
        };
    }

    // Event

    #[doc(hidden)]
    #[must_use]
    pub fn event(input: &DeriveInput) -> TokenStream {
        emit_impl_or_error!(event::EventDerive::new(input))
    }

    #[doc(hidden)]
    #[must_use]
    pub fn identified(input: &DeriveInput) -> TokenStream {
        emit_impl_or_error!(event::identifier::IdentifiedDerive::new(input))
    }

    #[doc(hidden)]
    #[must_use]
    pub fn tagged(input: &DeriveInput) -> TokenStream {
        emit_impl_or_error!(event::tag::TaggedDerive::new(input))
    }

    // Projection

    #[doc(hidden)]
    #[must_use]
    pub fn projection(input: &DeriveInput) -> TokenStream {
        emit_impl_or_error!(projection::ProjectionDerive::new(input))
    }

    #[doc(hidden)]
    #[must_use]
    pub fn query_source(input: &DeriveInput) -> TokenStream {
        emit_impl_or_error!(projection::query::QuerySourceDerive::new(input))
    }
}
