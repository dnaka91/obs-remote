use obs::{
    signal::{Handle, SourceSignal},
    source::Source,
};
use tokio::sync::broadcast;

use super::event_reply::Event;
use crate::get_data;

#[allow(dead_code, unreachable_code, clippy::diverging_sub_expression)]
#[must_use]
pub fn connect_signals(source: &Source<'_>, tx: broadcast::Sender<Event>) -> Vec<Handle> {
    let handler = source.signal_handler();
    let mut handles = Vec::new();

    handles.push({
        let tx = tx.clone();
        handler.connect(SourceSignal::TransitionStart, move |data| {
            let source = get_data!(data.get_source(), SourceSignal::TransitionStart, "source");

            tx.send(Event::TransitionBegin(todo!())).ok();
        })
    });

    handles.push({
        let tx = tx.clone();
        handler.connect(SourceSignal::TransitionStop, move |data| {
            let source = get_data!(data.get_source(), SourceSignal::TransitionStop, "source");

            tx.send(Event::TransitionEnd(todo!())).ok();
        })
    });

    handles.push({
        handler.connect(SourceSignal::TransitionVideoStop, move |data| {
            let source = get_data!(
                data.get_source(),
                SourceSignal::TransitionVideoStop,
                "source"
            );

            tx.send(Event::TransitionVideoEnd(todo!())).ok();
        })
    });

    handles
}
