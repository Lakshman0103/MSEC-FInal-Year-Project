use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{Write, copy};
use std::process::Command;
use anyhow::{Result, Context, anyhow};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};

// URLs for yt-dlp binaries
const YT_DLP_LINUX_URL: &str = "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp";

// Download the yt-dlp binary to the specified directory
pub async fn download_yt_dlp<P: AsRef<Path>>(dir_path: P) -> Result<PathBuf> {
    let dir_path = dir_path.as_ref();
    let yt_dlp_path = dir_path.join("yt-dlp");
    
    // Check if yt-dlp already exists
    if yt_dlp_path.exists() {
        // Check if yt-dlp is executable
        if is_executable(&yt_dlp_path) {
            return Ok(yt_dlp_path);
        } else {
            // Make it executable
            make_executable(&yt_dlp_path)?;
            return Ok(yt_dlp_path);
        }
    }
    
    // On Kali Linux, we can check if yt-dlp is installed system-wide
    let system_check = Command::new("which")
        .arg("yt-dlp")
        .output();
    
    if let Ok(output) = system_check {
        if output.status.success() {
            let system_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !system_path.is_empty() {
                println!("Using system-installed yt-dlp at: {}", system_path);
                return Ok(PathBuf::from(system_path));
            }
        }
    }
    
    // If not found in system or in the specified directory, download it
    println!("yt-dlp not found, downloading...");
    
    // Create a client
    let client = reqwest::Client::new();
    
    // Create the progress bar
    let pb = ProgressBar::new(0);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );
    
    // Send the request
    let res = client.get(YT_DLP_LINUX_URL)
        .send()
        .await
        .context("Failed to download yt-dlp")?;
    
    // Get the content length
    let total_size = res.content_length().unwrap_or(0);
    pb.set_length(total_size);
    
    // Create the file
    let mut file = File::create(&yt_dlp_path).context("Failed to create yt-dlp file")?;
    
    // Download the file
    let mut stream = res.bytes_stream();
    let mut downloaded: u64 = 0;
    
    while let Some(item) = stream.next().await {
        let chunk = item.context("Error while downloading file")?;
        file.write_all(&chunk).context("Error while writing to file")?;
        
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }
    
    pb.finish_with_message("Download complete");
    
    // Make the file executable
    make_executable(&yt_dlp_path)?;
    
    Ok(yt_dlp_path)
}

// Check if a file is executable
fn is_executable<P: AsRef<Path>>(path: P) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = fs::metadata(path) {
            return metadata.permissions().mode() & 0o111 != 0;
        }
    }
    false
}

// Make a file executable
fn make_executable<P: AsRef<Path>>(path: P) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(perms.mode() | 0o111);
        fs::set_permissions(&path, perms)?;
        return Ok(());
    }
    
    #[cfg(not(unix))]
    {
        // Not needed on non-Unix systems
        let _ = path;
        return Ok(());
    }
}

// Download a YouTube video
pub async fn download_video(url: &str, output_path: &str) -> Result<()> {
    // Create temp directory if it doesn't exist
    let temp_dir = "./temp";
    std::fs::create_dir_all(temp_dir).context("Failed to create temp directory")?;
    
    // Download yt-dlp if needed
    let yt_dlp_path = download_yt_dlp(temp_dir).await?;
    
    // Download the video
    let status = Command::new(&yt_dlp_path)
        .args(&[
            "--format", "mp4",
            "--output", output_path,
            "--no-playlist",
            url
        ])
        .status()
        .context("Failed to execute yt-dlp")?;
    
    if !status.success() {
        return Err(anyhow!("Failed to download video"));
    }
    
    Ok(())
}