use eventric_stream::stream::{
    Stream,
    append::AppendSelect as _,
    iterate::IterateSelect as _,
};

use crate::decision::Decision;

// =================================================================================================
// Stream
// =================================================================================================

// Executor

pub trait Executor {
    fn execute<D>(&mut self, decision: D) -> Result<D::Ok, D::Err>
    where
        D: Decision;
}

// Stream

impl Executor for Stream {
    fn execute<D>(&mut self, mut decision: D) -> Result<D::Ok, D::Err>
    where
        D: Decision,
    {
        let mut after = None;
        let mut context = decision.context();

        let selections = decision.select(&context)?;

        let (events, select) = self.iter_select(selections, None);

        for event in events {
            let event = event?;
            let position = *event.position();

            after = Some(position);

            decision.update(&mut context, &event)?;
        }

        let ok = decision.execute(&mut context)?;
        let events = context.into().take();

        if !events.is_empty() {
            self.append_select(events, select, after)?;
        }

        Ok(ok)
    }
}
