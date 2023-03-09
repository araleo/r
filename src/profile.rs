use anyhow::{anyhow, Ok, Result};
use r::constants;
use std::{fs::create_dir_all, path::PathBuf};

pub fn _get_config_dir() -> Result<PathBuf> {
    let home_dir = dirs_next::home_dir();
    match home_dir {
        Some(h) => {
            let config_dir = h.join(".r");
            if !config_dir.exists() {
                create_dir_all(&config_dir)?;
            }
            Ok(config_dir)
        }
        None => Err(anyhow!("Could not find user's home dir")),
    }
}

pub fn _get_profile_dir(config_dir: PathBuf, profile: &str) -> Result<PathBuf> {
    let profile_dir = config_dir.join(profile);
    if profile_dir.exists() {
        return Ok(profile_dir);
    }
    Err(anyhow!("Could not find profile"))
}

pub fn _create_default_profile(config_dir: PathBuf) -> Result<()> {
    let default_profile_dir = config_dir.join(constants::DEFAULT_PROFILE);
    create_dir_all(default_profile_dir)?;
    Ok(())
}
