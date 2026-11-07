use std::{
    fs::OpenOptions,
    time::{Duration, SystemTime},
};

use anyhow::Result;
use fs2::FileExt;
use swayipc::Connection;

use crate::{utils::msg_switch, window::window};

pub fn switch_to(index: u32) -> Result<()> {
    let mut sway_connection = Connection::new()?;
    msg_switch(&mut sway_connection, index)?;

    let runtime_dir = dirs::runtime_dir().unwrap_or_else(std::env::temp_dir);
    let lock_path = runtime_dir.join("spc_splash.lock");

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)?;

    if file.try_lock_exclusive().is_err() {
        return Ok(());
    }

    if let Ok(metadata) = file.metadata()
        && let Ok(modified) = metadata.modified()
        && let Ok(elapsed) = modified.elapsed()
        && elapsed < Duration::from_secs(60)
    {
        return Ok(());
    }

    window()?;

    file.set_modified(SystemTime::now())?;

    Ok(())
}
