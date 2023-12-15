use anyhow::Result;
use serde_yaml::Value as YamlValue;

use std::{fs::File, path::PathBuf};

pub fn load_yaml(file_path: PathBuf) -> Result<YamlValue> {
    let file_reader = File::open(file_path).expect("Unable to open file");
    let data: YamlValue = serde_yaml::from_reader(file_reader)?;

    Ok(data)
}
