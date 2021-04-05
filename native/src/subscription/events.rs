use crate::event::{self, Event};
use crate::subscription::{EventStream, Recipe};
use crate::Hasher;
use iced_futures::futures::future;
use iced_futures::futures::StreamExt;
use iced_futures::BoxStream;

pub struct Events<Message> {
    pub(super) f: Box<dyn Fn(Event, event::Status) -> Option<Message> + Send>,
}

impl<Message> Recipe<Hasher, (Event, event::Status)> for Events<Message>
where
    Message: 'static + Send,
{
    type Output = Message;

    fn hash(&self, state: &mut Hasher) {
        use std::hash::{ Hash, Hasher };

        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);
        state.write_usize(&self.f as *const _ as usize)
    }

    fn stream(
        self: Box<Self>,
        event_stream: EventStream,
    ) -> BoxStream<Self::Output> {
        event_stream
            .filter_map(move |(event, status)| {
                future::ready((self.f)(event, status))
            })
            .boxed()
    }
}
