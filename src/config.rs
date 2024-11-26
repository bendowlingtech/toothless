use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub php: PhpConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {

}

#[derive(Debug, Deserialize)]
pub struct PhpConfig {
    pub ini_path: String,
    pub extensions: Vec<String>,
}

impl Config {
    pub fn load()
}
