use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Embed(EmbedParams),
    Dislodge(DislodgeParams),
    Download(DownloadParams),
}

#[derive(Args, Default, Debug)]
pub struct EmbedParams {
    #[arg(short, long)]
    /// Path to the file with the data to encode
    pub in_path: Option<String>,
    
    #[arg(short, long)]
    /// Preset to use when encoding data
    pub preset: Option<EmbedPreset>,
    
    #[arg(long)]
    /// Etching mode
    pub mode: Option<EmbedOutputMode>,
    
    #[arg(long)]
    /// Block size, in pixels per side
    pub block_size: Option<i32>,
    
    #[arg(long)]
    /// Number of threads to use for processing
    pub threads: Option<usize>,
    
    #[arg(long)]
    /// Output video frames per second (FPS)
    pub fps: Option<i32>,
    
    #[arg(long)]
    /// Output video resolution
    pub resolution: Option<String>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum EmbedPreset {
    /// Optimal compression resistance
    Optimal,
    /// Paranoid compression resistance
    Paranoid,
    /// Maximum efficiency
    MaxEfficiency,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum EmbedOutputMode {
    /// Uses RGB values
    Colored,
    /// Uses black and white pixels
    Binary,
}

impl From<EmbedOutputMode> for crate::settings::OutputMode {
    fn from(value: EmbedOutputMode) -> Self {
        match value {
            EmbedOutputMode::Colored => Self::Color,
            EmbedOutputMode::Binary => Self::Binary,
        }
    }
}

#[derive(Args, Default)]
pub struct DislodgeParams {
    /// Path to input video
    #[arg(short, long)]
    pub in_path: Option<String>,
    
    /// Path to file output (including extension)
    #[arg(short, long)]
    pub out_path: Option<String>,
}

#[derive(Args, Default)]
pub struct DownloadParams {
    /// Video URL
    #[arg(short, long)]
    pub url: Option<String>,
}