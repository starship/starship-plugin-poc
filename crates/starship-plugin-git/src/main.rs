use std::{error::Error, io};

use starship_plugin::Sender;

fn main() -> Result<(), Box<dyn Error>> {
    let mut sender = Sender::new();

    let dir = sender.current_dir()?;
    sender.result(dir)?;

    Ok(())
}
