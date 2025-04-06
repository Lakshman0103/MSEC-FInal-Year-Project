// Defines the source for embedding data

use std::path::Path;
use anyhow::{Result, anyhow};

// EmbedSource represents different types of files to embed
pub enum EmbedSource {
    File(String),
    Text(String),
    Binary(Vec<u8>),
}

impl EmbedSource {
    // Create a new embed source from a file path
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        
        // Check if the file exists
        if !Path::new(&path_str).exists() {
            return Err(anyhow!("File does not exist: {}", path_str));
        }
        
        Ok(EmbedSource::File(path_str))
    }
    
    // Create a new embed source from a text string
    pub fn from_text(text: &str) -> Self {
        EmbedSource::Text(text.to_string())
    }
    
    // Create a new embed source from raw binary data
    pub fn from_binary(data: Vec<u8>) -> Self {
        EmbedSource::Binary(data)
    }
    
    // Get the binary data from this source
    pub fn get_binary_data(&self) -> Result<Vec<u8>> {
        match self {
            EmbedSource::File(path) => {
                std::fs::read(path).map_err(|e| anyhow!("Failed to read file: {}", e))
            },
            EmbedSource::Text(text) => {
                Ok(text.as_bytes().to_vec())
            },
            EmbedSource::Binary(data) => {
                Ok(data.clone())
            }
        }
    }
}