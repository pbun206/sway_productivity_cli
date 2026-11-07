use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Splashes {
    pub content: Vec<String>,
}

fn get_splashes_path() -> Result<PathBuf> {
    let config_dir =
        dirs::config_dir().ok_or_else(|| anyhow!("could not determine config directory"))?;
    Ok(config_dir.join("spc/splashes.ron"))
}

pub fn get_splashes() -> Result<Splashes> {
    let path = get_splashes_path()?;
    let data = fs::read_to_string(&path)
        .with_context(|| format!("failed to read splashes file at {:?}", path))?;
    let splashes: Splashes =
        ron::from_str(&data).with_context(|| format!("failed to parse ron file at {:?}", path))?;
    Ok(splashes)
}
