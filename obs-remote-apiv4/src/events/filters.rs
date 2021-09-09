use obs::{
    signal::{Handle, SourceSignal},
    source::Source,
};
use tokio::sync::broadcast;

use super::event_reply::Event;

#[must_use]
pub fn connect_signals(source: &Source<'_>, tx: broadcast::Sender<Event>) -> Vec<Handle> {
    let handler = source.signal_handler();
    let mut handles = Vec::new();

    handles.push({
        handler.connect(SourceSignal::Enable, move |data| {
            tx.send(Event::SourceFilterVisibilityChanged(todo!())).ok();
        })
    });

    handles
}
