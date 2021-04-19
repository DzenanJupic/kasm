use std::sync::Arc;

use parking_lot::Mutex;
use seed::{*, prelude::*};

use crate::Msg;

const MAX_BUFFER_SIZE: usize = 100_000;

#[derive(Clone, Debug, Default)]
pub struct ConsoleOut {
    buffer: Arc<Mutex<String>>
}

#[derive(Clone)]
pub enum ConsoleMsg {
    Clear
}

impl ConsoleOut {
    pub fn read(&self) -> String {
        self.buffer.lock().clone()
    }

    pub fn clear(&self) {
        self.buffer.lock().clear()
    }
    
    pub fn write_str(&self, s: &str) {
        let mut lines = s.lines();
        let mut buffer = self.buffer.lock();

        if buffer.ends_with('\n') || buffer.is_empty() {
            buffer.push_str("> ")
        }

        if let Some(line) = lines.next() {
            buffer.push_str(line);
            buffer.push('\n');
        }

        for line in lines {
            buffer.push_str("  ");
            buffer.push_str(line);
            buffer.push('\n');
        }

        if !s.ends_with('\n') {
            buffer.pop();
        }

        if buffer.len() > MAX_BUFFER_SIZE {
            buffer.drain(0..buffer.len() / 2);
            let first_new_line = buffer
                .find('\n')
                .unwrap_or(0);
            buffer.drain(..first_new_line);
        }
    }
    
    pub fn view(&self) -> Node<Msg> {
        div![
            id!("console"),
            pre![
                style! {
                    St::Background => "#000",
                    St::WhiteSpace => "pre-wrap",
                    St::Height => "10em",
                },
                C![
                    "m-0", "px-3", "py-2", "text-white", "overflow-auto",
                    "d-flex", "flex-column-reverse"
                ],
                
                self.read()
            ]
        ]
    }

    pub fn update(msg: ConsoleMsg, console: &mut ConsoleOut) {
        match msg {
            ConsoleMsg::Clear => {
                console.clear();
            }
        }
    }
}

impl std::io::Write for ConsoleOut {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = std::str::from_utf8(buf)
            .map_err(|_| std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "The buffer to write contains invalid utf-8 bytes",
            ))?;
        
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
