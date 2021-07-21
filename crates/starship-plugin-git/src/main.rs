use std::error::Error;

use starship_plugin::Sender;

fn main() -> Result<(), Box<dyn Error>> {
    let mut sender = Sender::new();
    let dir = sender.current_dir()?;
    dbg!(&dir);

    Ok(())
}
