use anyhow::Result;
use swayipc::Connection;

use crate::utils::{find_workspace_of_number, get_current_workspace, msg_change_index};

pub fn swap_with(index: i32) -> Result<()> {
    let mut connection = Connection::new()?;
    let current_workspace = get_current_workspace(&mut connection)?;
    if let Some(swaped_workspace) = find_workspace_of_number(&mut connection, index)? {
        msg_change_index(&mut connection, &swaped_workspace, current_workspace.num)?;
    }
    msg_change_index(&mut connection, &current_workspace, index)?;
    Ok(())
}
