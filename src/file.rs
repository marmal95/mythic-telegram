use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Ok, Result};

pub fn read_bytes(path: &Path) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

pub fn write_bytes(path: &PathBuf, data: &[u8]) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

pub fn extract_file_name(path: &Path) -> Result<String> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|str| str.to_owned())
        .ok_or(anyhow!(
            "Could not extract filename from path: {}",
            path.to_string_lossy()
        ))
}
