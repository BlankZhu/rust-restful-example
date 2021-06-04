use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, io::BufReader, path::Path};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub log_level: Option<String>,
    pub mongo_uri: String,
    pub port: Option<u16>,
    pub threads: Option<usize>,
}

impl AppConfig {
    pub fn load_from_yaml_file<P: AsRef<Path>>(path: P) -> Result<AppConfig, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let ret = serde_yaml::from_reader(reader)?;
        return Ok(ret);
    }
}
