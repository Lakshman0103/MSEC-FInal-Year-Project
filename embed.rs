use crate::{
    args::{EmbedParams, EmbedPreset},
    etcher,
    settings::{Data, OutputMode, Settings},
};

pub async fn run_embed(args: EmbedParams) -> anyhow::Result<()> {
    println!("Starting embed process...");
    
    //Should use enums
    let mut settings = Settings::default();
    let mut output_mode = OutputMode::Binary;
    
    match args.preset {
        Some(EmbedPreset::MaxEfficiency) => {
            output_mode = OutputMode::Color;
            settings.size = 1;
            settings.threads = 8;
            settings.fps = 10.0;
            settings.width = 256;
            settings.height = 144;
        }
        Some(EmbedPreset::Optimal) => {
            output_mode = OutputMode::Binary;
            settings.size = 2;
            settings.threads = 8;
            settings.fps = 10.0;
            settings.width = 1280;
            settings.height = 720;
        }
        Some(EmbedPreset::Paranoid) => {
            output_mode = OutputMode::Binary;
            settings.size = 4;
            settings.threads = 8;
            settings.fps = 10.0;
            settings.width = 1280;
            settings.height = 720;
        }
        _ => (),
    }

    // If none of the presets were picked,
    // then all the parameters are included in the args,
    // so it is safe to gather them from the args now
    if settings.width == 0 || settings.height == 0 {
        if args.resolution.is_none() {
            settings.width = 640;
            settings.height = 360;
        } else {
            let (width, height) = match args.resolution.as_ref().unwrap().as_str() {
                "144p" => (256, 144),
                "240p" => (426, 240),
                "360p" => (640, 360),
                "480p" => (854, 480),
                "720p" => (1280, 720),
                _ => (640, 360),
            };
            settings.width = width;
            settings.height = height;
        }
    }
    
    if let Some(mode) = args.mode {
        output_mode = mode.into();
    }
    
    if let Some(bs) = args.block_size {
        settings.size = bs;
    }
    
    if let Some(threads) = args.threads {
        settings.threads = threads;
    }
    
    if let Some(fps) = args.fps {
        settings.fps = fps as f64;
    }

    // Get the input path or use a default
    let input_path = args.in_path.unwrap_or_else(|| {
        println!("No input path specified, using test_file.txt");
        "test_file.txt".to_string()
    });

    match output_mode {
        OutputMode::Color => {
            println!("Using COLOR mode with block size: {}", settings.size);
            let bytes = etcher::rip_bytes(&input_path)?;
            let data = Data::from_color(bytes);
            etcher::etch("output.binvid", data, settings)?;
        }
        OutputMode::Binary => {
            println!("Using BINARY mode with block size: {}", settings.size);
            let bytes = etcher::rip_bytes(&input_path)?;
            let binary = etcher::rip_binary(bytes)?;
            let data = Data::from_binary(binary);
            etcher::etch("output.binvid", data, settings)?;
        }
    }
    
    println!("Embed process completed successfully!");
    println!("Output file: output.binvid");
    
    Ok(())
}