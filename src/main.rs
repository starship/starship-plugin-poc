use std::{
    convert::{TryFrom},
    error::Error,
    io::{Write},
    process::{Command, Stdio},
};

use starship_plugin::{Message, Sender};

fn main() -> Result<(), Box<dyn Error>> {
    let process = Command::new("target/debug/starship-plugin-git")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = process.stdout.expect("Could not capture stdout");
    let stdin = process.stdin.expect("Could not capture stdin");
    let mut sender = Sender::from(stdout, stdin);

    loop {
        let message = &sender.receive()?;

        match message {
            Message::CurrentDir => {
                let current_dir = std::env::current_dir()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                let message = bincode::serialize(&current_dir).unwrap();
                let message_size = u32::try_from(message.len())?;
    
                let writer = &mut sender.write;
                writer.write_all(&u32::to_le_bytes(message_size))?;
                writer.write_all(&message)?;
                writer.flush()?;
            }
            Message::Result(value) => {
                println!("{}", value);
                return Ok(())
            }
        }
    }
}
