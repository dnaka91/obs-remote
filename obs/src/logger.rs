use std::{ffi::CString, os::raw::c_int, usize};

use log::{Level, Log, SetLoggerError};

pub struct ObsLogger {
    name: &'static str,
    level: Level,
    filters: Vec<(&'static str, Level)>,
}

impl ObsLogger {
    pub fn init(
        name: &'static str,
        level: Level,
        filters: Vec<(&'static str, Level)>,
    ) -> Result<(), SetLoggerError> {
        let max_level = level.to_level_filter().max(
            filters
                .iter()
                .map(|f| f.1.to_level_filter())
                .max()
                .unwrap_or(log::LevelFilter::Off),
        );

        log::set_boxed_logger(Box::new(ObsLogger {
            name,
            level,
            filters,
        }))
        .map(|()| log::set_max_level(max_level))
    }
}

impl Log for ObsLogger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        self.level >= metadata.level()
            || self.filters.iter().cloned().any(|(target, level)| {
                metadata.target().starts_with(target) && level >= metadata.level()
            })
    }

    fn log(&self, record: &log::Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let message = format!(
            "[{}] [{}] {}: {}",
            self.name,
            record.level(),
            record.target(),
            record.args()
        );
        let level = match record.level() {
            Level::Error => libobs_sys::LOG_ERROR,
            Level::Warn => libobs_sys::LOG_WARNING,
            Level::Info | Level::Debug | Level::Trace => libobs_sys::LOG_INFO,
        };

        for chunk in to_chunks(message.replace("%", "%%")) {
            if let Ok(chunk) = CString::new(chunk) {
                unsafe { libobs_sys::blog(level as c_int, chunk.as_ptr()) }
            }
        }
    }

    fn flush(&self) {}
}

fn to_chunks(value: String) -> impl Iterator<Item = String> {
    std::iter::once(value)
}

struct StringChunker<'a> {
    value: &'a str,
    size: usize,
}

impl<'a> Iterator for StringChunker<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let last = self
            .value
            .char_indices()
            .take_while(|(i, _)| *i < self.size)
            .last();

        if let Some((index, _)) = last {
            let chunk = &self.value[..index];
            self.value = &self.value[index..];

            Some(chunk)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunker() {
        let mut chunker = StringChunker {
            value: "hello world!",
            size: 5,
        };

        assert_eq!(Some("hello"), chunker.next());
    }
}
