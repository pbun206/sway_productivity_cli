use std::process::Command;
use std::process::Stdio;

use crate::utils::get_current_workspace;
use crate::utils::msg_rename_name;
use anyhow::Result;
use anyhow::anyhow;
use swayipc::Connection;

pub fn rename() -> Result<()> {
    let mut connection = Connection::new()?;
    let new_name = get_new_name()?;
    let current_workspace = get_current_workspace(&mut connection)?;
    msg_rename_name(&mut connection, &current_workspace, &new_name)
}

fn get_new_name() -> Result<String> {
    let qtai = Command::new("qtai")
        .arg("run")
        .arg("echo")
        .arg("--runner")
        .arg("echo $1")
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|_| anyhow!("qtai command failed"))?;
    String::from_utf8(
        qtai.wait_with_output()
            .map_err(|err| anyhow!("Failed to wait for output: {}", err))?
            .stdout,
    )
    .map_err(|err| anyhow!("Output is not utf8: {}", err))
}
