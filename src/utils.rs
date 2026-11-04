use anyhow::{Result, anyhow};
use swayipc::{Connection, Workspace};

pub fn get_current_workspace(connection: &mut Connection) -> Result<Workspace> {
    let workspaces = connection
        .get_workspaces()
        .map_err(|_| anyhow!("get workspaces ipc failed"))?;

    workspaces
        .iter()
        .find(|w| w.focused)
        .ok_or(anyhow!("finding focused workspace failed"))
        .cloned()
}

pub fn find_workspace_of_number(
    connection: &mut Connection,
    index: i32,
) -> Result<Option<Workspace>> {
    let workspaces = connection
        .get_workspaces()
        .map_err(|_| anyhow!("get workspaces ipc failed"))?;

    Ok(workspaces.iter().find(|w| w.num == index).cloned())
}

// Rename doesn't not contian index
pub fn msg_rename_name(
    connection: &mut Connection,
    workspace: &Workspace,
    new_name: &str,
) -> Result<()> {
    msg_rename_full(
        connection,
        workspace,
        &(workspace.num.to_string() + ":" + new_name),
    )
}

// Rename doesn't not contain index
pub fn msg_change_index(
    connection: &mut Connection,
    workspace: &Workspace,
    new_index: i32,
) -> Result<()> {
    match workspace.name.find(':') {
        Some(i) => msg_rename_full(
            connection,
            workspace,
            &(new_index.to_string() + ":" + &workspace.name[i + 1..]),
        ),
        None => msg_rename_full(connection, workspace, &(new_index.to_string())),
    }?;
    Ok(())
}

// fully rename something :p
pub fn msg_rename_full(
    connection: &mut Connection,
    workspace: &Workspace,
    new_name: &str,
) -> Result<()> {
    match connection.run_command(format!(
        "rename workspace {} to {}",
        workspace.name, new_name
    )) {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow!("renaming run comand failed")),
    }
}

pub fn msg_switch(connection: &mut Connection, index: i32) -> Result<()> {
    match connection.run_command(format!("workspace number {}", index)) {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow!("switch command failed")),
    }
}
