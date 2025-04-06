use crate::args::DownloadParams;
use crate::youtube_dl;
use anyhow::{Context, Result};
use std::process::Command;
use std::path::Path;
use std::io::{self, Write};
use regex::Regex;

pub async fn run_download(args: DownloadParams) -> Result<()> {
    let url = match args.url {
        Some(url) => url,
        None => {
            println!("Please enter the URL of the video to download:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).context("Failed to read URL input")?;
            input.trim().to_string()
        }
    };

    println!("Downloading video from URL: {}", url);
    
    // Create temp directory if it doesn't exist
    let temp_dir = "./temp";
    std::fs::create_dir_all(temp_dir).context("Failed to create temp directory")?;
    
    // Download yt-dlp if needed
    let yt_dlp_path = youtube_dl::download_yt_dlp(temp_dir).await?;
    
    // Use yt-dlp to get video info
    println!("Getting video information...");
    let output = Command::new(&yt_dlp_path)
        .args(&["--dump-json", "--no-playlist", &url])
        .output()
        .context("Failed to execute yt-dlp for video info")?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("yt-dlp failed: {}", error));
    }
    
    let info_json = String::from_utf8_lossy(&output.stdout);
    
    // Parse video ID and title from the json
    let id_regex = Regex::new(r#""id":\s*"([^"]+)"#).unwrap();
    let title_regex = Regex::new(r#""title":\s*"([^"]+)"#).unwrap();
    
    let video_id = id_regex.captures(&info_json)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
        .unwrap_or("unknown");
    
    let video_title = title_regex.captures(&info_json)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
        .unwrap_or("Unknown Title");
    
    let output_path = Path::new(temp_dir).join(format!("{}.mp4", video_id));
    
    println!("Downloading: {}", video_title);
    print!("Progress: ");
    io::stdout().flush().ok();
    
    // Download the video
    let status = Command::new(&yt_dlp_path)
        .args(&[
            "--format", "mp4",
            "--output", output_path.to_str().unwrap(),
            "--no-playlist",
            &url
        ])
        .status()
        .context("Failed to execute yt-dlp for download")?;
    
    if !status.success() {
        return Err(anyhow::anyhow!("yt-dlp download failed"));
    }
    
    println!("\nDownload complete! Video saved to: {}", output_path.display());
    println!("You can now use the 'dislodge' command with this video to extract any embedded data.");
    
    Ok(())
}