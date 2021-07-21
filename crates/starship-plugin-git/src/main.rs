use std::{error::Error};

use starship_plugin::Sender;

fn main() -> Result<(), Box<dyn Error>> {
    let mut sender = Sender::new_child();
    let dir = sender.current_dir()?;
    sender.result(dir)?;

    Ok(())
}
