use anyhow::Result;
use swayipc::Connection;

use crate::utils::msg_switch;

pub fn switch_to(index: i32) -> Result<()> {
    let mut connection = Connection::new()?;
    msg_switch(&mut connection, index)
}
