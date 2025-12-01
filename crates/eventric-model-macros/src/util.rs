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
// Utilities
// =================================================================================================

#[derive(Debug, Clone)]
pub struct List<T>(Vec<T>)
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
