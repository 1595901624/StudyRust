use std::io;
use std::io::{Read, Write};
use reqwest::blocking::Response;

pub trait Output {
    fn output(&mut self) -> io::Result<()>;
}

impl Output for Response {
    fn output(&mut self) -> io::Result<()> {
        let mut bytes = Vec::new();
        self.read_to_end(&mut bytes)?;
        io::stdout().write_all(&bytes)
    }
}
