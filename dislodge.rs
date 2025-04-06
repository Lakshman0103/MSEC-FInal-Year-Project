use crate::args::DislodgeParams;
use crate::etcher;

pub async fn run_dislodge(args: DislodgeParams) -> anyhow::Result<()> {
    println!("Starting dislodge process...");
    
    // Get input and output paths
    let in_path = args.in_path.unwrap_or_else(|| {
        println!("No input path specified, using output.binvid");
        "output.binvid".to_string()
    });
    
    let out_path = args.out_path.unwrap_or_else(|| {
        println!("No output path specified, using extracted_file.bin");
        "extracted_file.bin".to_string()
    });
    
    println!("Reading from: {}", in_path);
    println!("Writing to: {}", out_path);
    
    // Read the encoded data and extract it
    let out_data = etcher::read(&in_path, 1)?;
    etcher::write_bytes(&out_path, out_data)?;
    
    println!("Dislodge process completed successfully!");
    println!("Extracted data written to: {}", out_path);
    
    Ok(())
}