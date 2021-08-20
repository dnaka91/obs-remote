use std::io::{self, Write};

use log::Level;
use tracing_subscriber::fmt::MakeWriter;

pub struct ObsWriter;

impl MakeWriter for ObsWriter {
    type Writer = Self;

    fn make_writer(&self) -> Self::Writer {
        Self
    }
}

impl Write for ObsWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let message = String::from_utf8_lossy(buf);
        super::logger::blog(Level::Info, &message);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
