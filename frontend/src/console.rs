use std::sync::Arc;

use parking_lot::Mutex;

#[derive(Clone, Debug, Default)]
pub struct ConsoleOut {
    buffer: Arc<Mutex<String>>
}

impl ConsoleOut {
    pub fn new(s: String) -> Self {
        Self {
            buffer: Arc::new(Mutex::new(s))
        }
    }

    pub fn read(&self) -> String {
        self.buffer.lock().clone()
    }

    pub fn clear(&self) {
        self.buffer.lock().clear()
    }
}

impl std::io::Write for ConsoleOut {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = std::str::from_utf8(buf)
            .map_err(|_| std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "The buffer to write contains invalid utf-8 bytes",
            ))?;

        self.buffer
            .lock()
            .push_str(s);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
