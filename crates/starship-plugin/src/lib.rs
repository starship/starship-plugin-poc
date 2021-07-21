use std::{
    convert::{TryFrom, TryInto},
    error::Error,
    io::{self, Read, Stdin, Stdout, Write},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    CurrentDir,
}

impl TryFrom<Vec<u8>> for Message {
    type Error = Box<dyn Error>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        bincode::deserialize(&value).map_err(Into::into)
    }
}

pub struct Sender {
    stdout: Stdout,
    stdin: Stdin,
}

impl Sender {
    pub fn new() -> Self {
        Self {
            stdout: io::stdout(),
            stdin: io::stdin(),
        }
    }

    pub fn current_dir(&mut self) -> Result<String, Box<dyn Error>> {
        self.send(Message::CurrentDir)?;

        let mut handle = self.stdin.lock();

        let mut size_buffer = [0; 4];
        handle.read_exact(&mut size_buffer)?;
        let size = u32::from_le_bytes(size_buffer);

        let mut content_buffer = vec![0; size.try_into()?];
        handle.read_exact(&mut content_buffer)?;

        bincode::deserialize(&content_buffer).map_err(Into::into)
    }

    fn send(&mut self, message: Message) -> Result<(), Box<dyn Error>> {
        let message = bincode::serialize(&message)?;
        let message_size: u32 = message.len().try_into()?;

        let mut handle = self.stdout.lock();
        handle.write_all(&u32::to_le_bytes(message_size))?;
        handle.write_all(&message)?;
        handle.flush()?;
        Ok(())
    }
}
