use obs::{
    signal::{Handle, SourceSignal},
    source::Source,
};
use tokio::sync::broadcast;

use super::{event_reply::Event, *};
use crate::get_data;

#[must_use]
pub fn connect_signals(source: &Source, tx: broadcast::Sender<Event>) -> Vec<Handle> {
    let handler = source.signal_handler();
    let mut handles = Vec::new();

    handles.push({
        let tx = tx.clone();
        handler.connect(SourceSignal::Rename, move |data| {
            let source = get_data!(data.get_source(), SourceSignal::Rename, "source");
            let new_name = get_data!(data.string("new_name"), SourceSignal::Rename, "new_name");
            let previous_name =
                get_data!(data.string("prev_name"), SourceSignal::Rename, "prev_name");

            tx.send(Event::SourceRenamed({
                let mut event = SourceRenamed {
                    previous_name,
                    new_name,
                    source_type: Default::default(),
                };
                event.set_source_type(source.ty().into());
                event
            }))
            .ok();
        })
    });

    handles.push({
        handler.connect(SourceSignal::Mute, move |data| {
            let source = get_data!(data.get_source(), SourceSignal::Mute, "source");
            let muted = get_data!(data.bool("muted"), SourceSignal::Mute, "muted");

            tx.send(Event::SourceMuteStateChanged(SourceMuteStateChanged {
                name: source.name(),
                muted,
            }))
            .ok();
        })
    });

    handles
}
