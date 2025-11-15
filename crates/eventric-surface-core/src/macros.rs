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

pub mod derive;

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

impl<T> Parse for List<T>
where
    T: Parse,
{
    fn parse(stream: ParseStream<'_>) -> syn::Result<Self> {
        let items = Punctuated::<T, Comma>::parse_terminated(stream)?;
        let items = items.into_iter().collect();
        let items = Self(items);

        Ok(items)
    }
}

impl<T> FromMeta for List<T>
where
    T: Parse,
{
    fn from_meta(meta: &Meta) -> darling::Result<Self> {
        syn::parse2::<List<T>>(meta.require_list()?.tokens.clone()).map_err(darling::Error::custom)
    }
}
