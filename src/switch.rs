use anyhow::Result;
use swayipc::Connection;

use crate::{utils::msg_switch, window::window};

pub fn switch_to(index: u32) -> Result<()> {
    let mut sway_connection = Connection::new()?;
    msg_switch(&mut sway_connection, index)?;

    window()?;

    Ok(())
}
