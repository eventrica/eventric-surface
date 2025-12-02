//! See the `eventric-surface` crate for full documentation, including
//! module-level documentation.

use std::ops::{
    Deref,
    DerefMut,
};

use eventric_stream::{
    error::Error,
    stream::select::{
        EventMasked,
        Selections,
    },
};

use crate::event::Events;

// =================================================================================================
// Action
// =================================================================================================

// Action

pub trait Action: Act + Context + Select + Update {}

// Act

pub trait Act: Context
where
    Self::Err: From<Error>,
{
    type Err;
    type Ok = ();

    fn action(&mut self, context: &mut Self::Context) -> Result<Self::Ok, Self::Err>;
}

// Context

pub trait Context
where
    Self::Context: Deref<Target = Events> + DerefMut + Into<Events>,
{
    type Context;

    fn context(&self) -> Self::Context;
}

// Select

pub trait Select: Context {
    fn select(&self, context: &Self::Context) -> Result<Selections, Error>;
}

// Update

pub trait Update: Context {
    fn update(&self, context: &mut Self::Context, event: &EventMasked) -> Result<(), Error>;
}
