use std::{io, str};

/// A simple struct that implements Write that can be written to instead of, say, stdout.
/// Captures the output for inspection.
#[derive(Default, Debug, Clone)]
pub struct CapturedWrite {
    pub buffer: String,
    pub flushed: bool,
}

impl CapturedWrite {
    pub fn new() -> Self {
        Self {
            buffer: "".to_owned(),
            flushed: false,
        }
    }

    pub fn reset(&mut self) {
        "".clone_into(&mut self.buffer);
        self.flushed = false;
    }
}

impl io::Write for CapturedWrite {
    fn write(&mut self, content: &[u8]) -> io::Result<usize> {
        let content =
            str::from_utf8(content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        self.buffer.push_str(content);
        self.flushed = false;
        Ok(content.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flushed = true;
        Ok(())
    }
}
