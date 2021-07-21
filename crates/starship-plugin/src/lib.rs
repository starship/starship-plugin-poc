use std::{borrow::BorrowMut, convert::{TryFrom, TryInto}, error::Error, io::{self, Read, Stdin, Stdout, Write}};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    CurrentDir,
    Result(String)
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
        self.receive()
    }

    pub fn result(&mut self, result: String) -> Result<(), Box<dyn Error>> {
        self.send(Message::Result(result))
    }

    fn send(&mut self, message: Message) -> Result<(), Box<dyn Error>> {
        let message = bincode::serialize(&message)?;
        let message_size: u32 = message.len().try_into()?;

        let handle = self.stdout.borrow_mut();
        handle.write_all(&u32::to_le_bytes(message_size))?;
        handle.write_all(&message)?;
        handle.flush()?;
        Ok(())
    }

    fn receive<T: DeserializeOwned>(&mut self) -> Result<T, Box<dyn Error>> {
        let handle = self.stdin.borrow_mut();

        let mut size_buffer = [0; 4];
        handle.read_exact(&mut size_buffer)?;
        let size = u32::from_le_bytes(size_buffer);

        let mut content_buffer = vec![0; size.try_into()?];
        handle.read_exact(&mut content_buffer)?;

        bincode::deserialize(&content_buffer).map_err(Into::into)
    }
}
