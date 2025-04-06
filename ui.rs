use std::io::{self, Write};
use anyhow::Result;
use crate::args::{Commands, EmbedParams, DislodgeParams, DownloadParams, EmbedPreset, EmbedOutputMode};

// Simple interactive UI for command-line interface
pub async fn enrich_arguments(command: Option<Commands>) -> Result<Commands> {
    match command {
        Some(cmd) => Ok(cmd),
        None => {
            // No command was provided, ask the user
            println!("Select an operation:");
            println!("1. Embed (encode data into a video)");
            println!("2. Dislodge (decode data from a video)");
            println!("3. Download (download a video for decoding)");
            print!("Enter your choice (1-3): ");
            io::stdout().flush()?;
            
            let mut choice = String::new();
            io::stdin().read_line(&mut choice)?;
            let choice = choice.trim();
            
            match choice {
                "1" => {
                    let params = configure_embed().await?;
                    Ok(Commands::Embed(params))
                },
                "2" => {
                    let params = configure_dislodge().await?;
                    Ok(Commands::Dislodge(params))
                },
                "3" => {
                    let params = configure_download().await?;
                    Ok(Commands::Download(params))
                },
                _ => {
                    println!("Invalid choice. Defaulting to Embed operation.");
                    let params = configure_embed().await?;
                    Ok(Commands::Embed(params))
                }
            }
        }
    }
}

async fn configure_embed() -> Result<EmbedParams> {
    println!("\n--- Embed Configuration ---");
    
    // Get input file path
    print!("Enter input file path (or press Enter for default 'test_file.txt'): ");
    io::stdout().flush()?;
    let mut in_path = String::new();
    io::stdin().read_line(&mut in_path)?;
    let in_path = in_path.trim();
    let in_path = if in_path.is_empty() { None } else { Some(in_path.to_string()) };
    
    // Select preset
    println!("\nSelect encoding preset:");
    println!("1. Optimal (balanced compression resistance)");
    println!("2. Paranoid (maximum compression resistance)");
    println!("3. Maximum Efficiency (highest data density)");
    println!("4. Custom (manual configuration)");
    print!("Enter your choice (1-4): ");
    io::stdout().flush()?;
    
    let mut preset_choice = String::new();
    io::stdin().read_line(&mut preset_choice)?;
    let preset_choice = preset_choice.trim();
    
    let mut params = EmbedParams::default();
    params.in_path = in_path;
    
    match preset_choice {
        "1" => {
            params.preset = Some(EmbedPreset::Optimal);
        },
        "2" => {
            params.preset = Some(EmbedPreset::Paranoid);
        },
        "3" => {
            params.preset = Some(EmbedPreset::MaxEfficiency);
        },
        "4" => {
            // Custom configuration
            
            // Output mode
            println!("\nSelect output mode:");
            println!("1. Binary (black and white, 1 bit per pixel)");
            println!("2. Color (RGB values, 24 bits per pixel)");
            print!("Enter your choice (1-2): ");
            io::stdout().flush()?;
            
            let mut mode_choice = String::new();
            io::stdin().read_line(&mut mode_choice)?;
            let mode_choice = mode_choice.trim();
            
            params.mode = match mode_choice {
                "1" => Some(EmbedOutputMode::Binary),
                "2" => Some(EmbedOutputMode::Colored),
                _ => Some(EmbedOutputMode::Binary), // Default to binary
            };
            
            // Block size
            print!("\nEnter block size (1-8, default: 2): ");
            io::stdout().flush()?;
            
            let mut block_size = String::new();
            io::stdin().read_line(&mut block_size)?;
            let block_size = block_size.trim();
            
            if !block_size.is_empty() {
                if let Ok(size) = block_size.parse::<i32>() {
                    params.block_size = Some(size);
                }
            }
            
            // Resolution
            println!("\nSelect resolution:");
            println!("1. 144p (256x144)");
            println!("2. 240p (426x240)");
            println!("3. 360p (640x360)");
            println!("4. 480p (854x480)");
            println!("5. 720p (1280x720)");
            print!("Enter your choice (1-5): ");
            io::stdout().flush()?;
            
            let mut res_choice = String::new();
            io::stdin().read_line(&mut res_choice)?;
            let res_choice = res_choice.trim();
            
            params.resolution = match res_choice {
                "1" => Some("144p".to_string()),
                "2" => Some("240p".to_string()),
                "3" => Some("360p".to_string()),
                "4" => Some("480p".to_string()),
                "5" => Some("720p".to_string()),
                _ => Some("360p".to_string()), // Default to 360p
            };
            
            // FPS
            print!("\nEnter FPS (1-60, default: 10): ");
            io::stdout().flush()?;
            
            let mut fps = String::new();
            io::stdin().read_line(&mut fps)?;
            let fps = fps.trim();
            
            if !fps.is_empty() {
                if let Ok(fps_val) = fps.parse::<i32>() {
                    params.fps = Some(fps_val);
                }
            }
            
            // Threads
            print!("\nEnter number of threads (1-16, default: 8): ");
            io::stdout().flush()?;
            
            let mut threads = String::new();
            io::stdin().read_line(&mut threads)?;
            let threads = threads.trim();
            
            if !threads.is_empty() {
                if let Ok(threads_val) = threads.parse::<usize>() {
                    params.threads = Some(threads_val);
                }
            }
        },
        _ => {
            // Default to Optimal
            params.preset = Some(EmbedPreset::Optimal);
        }
    }
    
    Ok(params)
}

async fn configure_dislodge() -> Result<DislodgeParams> {
    println!("\n--- Dislodge Configuration ---");
    
    // Get input file path
    print!("Enter input video/binvid path (or press Enter for default 'output.binvid'): ");
    io::stdout().flush()?;
    let mut in_path = String::new();
    io::stdin().read_line(&mut in_path)?;
    let in_path = in_path.trim();
    let in_path = if in_path.is_empty() { None } else { Some(in_path.to_string()) };
    
    // Get output file path
    print!("Enter output file path (or press Enter for default 'extracted_file.bin'): ");
    io::stdout().flush()?;
    let mut out_path = String::new();
    io::stdin().read_line(&mut out_path)?;
    let out_path = out_path.trim();
    let out_path = if out_path.is_empty() { None } else { Some(out_path.to_string()) };
    
    let params = DislodgeParams {
        in_path,
        out_path,
    };
    
    Ok(params)
}

async fn configure_download() -> Result<DownloadParams> {
    println!("\n--- Download Configuration ---");
    
    // Get video URL
    print!("Enter video URL: ");
    io::stdout().flush()?;
    let mut url = String::new();
    io::stdin().read_line(&mut url)?;
    let url = url.trim();
    let url = if url.is_empty() { None } else { Some(url.to_string()) };
    
    let params = DownloadParams {
        url,
    };
    
    Ok(params)
}