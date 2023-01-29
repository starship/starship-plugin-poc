use crate::DATA_DIR;
use std::{fs, process};

use anyhow::{bail, Context, Result};

pub(crate) fn create_pid_file() -> Result<()> {
    let pid_path = DATA_DIR.join("starship.pid");

    // If the pid file already exists, we should check if the process is still running.
    if pid_path.exists() {
        let pid = fs::read_to_string(pid_path.as_path())
            .with_context(|| format!("Failed to read pid file: {pid_path:?}"))?;
        let pid = pid
            .trim()
            .parse::<u32>()
            .with_context(|| format!("Failed to parse pid file: {pid_path:?}"))?;

        // If the process is still running, we should return an error.
        let process = psutil::process::Process::new(pid);
        if process.is_ok() && process?.is_running() {
            bail!(
                "Pid file already exists and another daemon seems to be running.\n\
                Please stop the daemon beforehand or delete the file manually: {:?}",
                pid_path
            );
        }
    }

    // Write the pid to the file.
    fs::write(pid_path.as_path(), process::id().to_string())
        .with_context(|| format!("Failed to write pid file: {pid_path:?}"))?;

    Ok(())
}
