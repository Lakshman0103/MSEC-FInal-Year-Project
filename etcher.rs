use std::fs;
use std::io::{Write, Read};
use std::path::Path;
use anyhow::{Result, anyhow};
use crate::settings::{Data, Settings, OutputMode};
use image::{RgbImage, Rgb};
use serde::{Serialize, Deserialize};

/// Read a file and convert it to bytes
pub fn rip_bytes<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let bytes = fs::read(path)?;
    Ok(bytes)
}

/// Convert bytes to binary (boolean vector)
pub fn rip_binary(bytes: Vec<u8>) -> Result<Vec<bool>> {
    let mut binary = Vec::with_capacity(bytes.len() * 8);
    
    for byte in bytes {
        for i in 0..8 {
            let bit = (byte >> i) & 1 == 1;
            binary.push(bit);
        }
    }
    
    Ok(binary)
}

/// Write bytes to a file
pub fn write_bytes<P: AsRef<Path>>(path: P, bytes: Vec<u8>) -> Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct VideoMetadata {
    mode: String,
    frames: u32,
    width: u32,
    height: u32,
    block_size: u32,
    data_size: usize,
}

/// Read binary data from the encoded format
pub fn read<P: AsRef<Path>>(path: P, _mode: i32) -> Result<Vec<u8>> {
    println!("Reading encoded data from: {}", path.as_ref().display());
    let mut file = fs::File::open(&path)?;
    
    // Read the first 10 bytes to determine the format
    let mut header = [0u8; 10];
    file.read_exact(&mut header)?;
    
    let header_str = std::str::from_utf8(&header[0..6])
        .map_err(|_| anyhow!("Invalid header format"))?;
    
    match header_str {
        "BINVID" => {
            // Binary mode
            let mut length_bytes = [0u8; 4];
            length_bytes.copy_from_slice(&header[6..10]);
            let data_length = u32::from_le_bytes(length_bytes) as usize;
            
            // Read the packed binary data
            let mut packed_data = Vec::new();
            file.read_to_end(&mut packed_data)?;
            
            // Unpack the bits
            let mut result = Vec::with_capacity(data_length);
            let mut bits_read = 0;
            
            for byte in packed_data {
                for i in 0..8 {
                    if bits_read >= data_length {
                        break;
                    }
                    
                    let bit = (byte >> i) & 1 == 1;
                    if bit {
                        result.push(1);
                    } else {
                        result.push(0);
                    }
                    
                    bits_read += 1;
                }
                
                if bits_read >= data_length {
                    break;
                }
            }
            
            // Convert bits back to bytes
            let mut bytes = Vec::with_capacity(data_length / 8 + 1);
            let mut current_byte = 0u8;
            let mut bit_pos = 0;
            
            for bit in result {
                if bit == 1 {
                    current_byte |= 1 << bit_pos;
                }
                
                bit_pos += 1;
                
                if bit_pos == 8 {
                    bytes.push(current_byte);
                    current_byte = 0;
                    bit_pos = 0;
                }
            }
            
            // Add the last byte if there are remaining bits
            if bit_pos > 0 {
                bytes.push(current_byte);
            }
            
            Ok(bytes)
        },
        "COLVID" => {
            // Color mode
            let mut length_bytes = [0u8; 4];
            length_bytes.copy_from_slice(&header[6..10]);
            let data_length = u32::from_le_bytes(length_bytes) as usize;
            
            // Read the actual data
            let mut data = Vec::with_capacity(data_length);
            file.read_to_end(&mut data)?;
            
            Ok(data)
        },
        _ => Err(anyhow!("Unknown file format")),
    }
}

/// Encode data to our custom format which simulates how we would encode it to video frames
pub fn etch<P: AsRef<Path> + Clone>(path: P, data: Data, settings: Settings) -> Result<()> {
    println!("Encoding data with the following settings:");
    println!("  Size: {}", settings.size);
    println!("  Resolution: {}x{}", settings.width, settings.height);
    println!("  FPS: {}", settings.fps);
    
    // Calculate how many pixels we can fit per frame
    let pixels_per_frame = (settings.width / settings.size) * (settings.height / settings.size);
    println!("  Pixels per frame: {}", pixels_per_frame);
    
    // Store path as a string for sample file
    let path_str = path.as_ref().to_string_lossy().to_string();
    
    match data.out_mode {
        OutputMode::Binary => {
            println!("Using binary (black/white) mode");
            let total_bits = data.binary.len();
            println!("  Total bits to encode: {}", total_bits);
            
            // Calculate how many frames we need
            let bits_per_frame = pixels_per_frame;
            let frames_needed = (total_bits as f64 / bits_per_frame as f64).ceil() as u32;
            println!("  Frames needed: {}", frames_needed);
            
            // Generate a sample image to demonstrate the encoding
            let img_width = settings.width as u32;
            let img_height = settings.height as u32;
            let mut img = RgbImage::new(img_width, img_height);
            
            // Fill the image with the binary data pattern
            let block_size = settings.size as u32;
            let mut bit_index = 0;
            
            for y in (0..img_height).step_by(block_size as usize) {
                for x in (0..img_width).step_by(block_size as usize) {
                    if bit_index < data.binary.len() {
                        let color = if data.binary[bit_index] {
                            Rgb([255, 255, 255]) // White for 1
                        } else {
                            Rgb([0, 0, 0]) // Black for 0
                        };
                        
                        // Fill the block with the color
                        for by in 0..block_size {
                            for bx in 0..block_size {
                                if y + by < img_height && x + bx < img_width {
                                    img.put_pixel(x + bx, y + by, color);
                                }
                            }
                        }
                        
                        bit_index += 1;
                    }
                }
            }
            
            // Save a sample frame as PNG to show what it looks like
            let sample_path = format!("{}_sample.png", path_str);
            img.save(&sample_path)?;
            println!("  Saved sample frame to: {}", sample_path);
            
            // For the actual data storage, we create our custom file format
            let mut output_bytes = Vec::new();
            
            // Add header to indicate binary mode
            output_bytes.extend_from_slice(b"BINVID");
            
            // Add data size as a u32 (4 bytes)
            let data_len = (data.binary.len() as u32).to_le_bytes();
            output_bytes.extend_from_slice(&data_len);
            
            // Pack boolean values into bytes (8 booleans per byte)
            let mut current_byte = 0u8;
            let mut bit_count = 0;
            
            for bit in data.binary {
                if bit {
                    current_byte |= 1 << bit_count;
                }
                
                bit_count += 1;
                
                if bit_count == 8 {
                    output_bytes.push(current_byte);
                    current_byte = 0;
                    bit_count = 0;
                }
            }
            
            // Push the final byte if there are remaining bits
            if bit_count > 0 {
                output_bytes.push(current_byte);
            }
            
            // Write the encoded data to the file
            write_bytes(path.clone(), output_bytes)?;
        }
        OutputMode::Color => {
            println!("Using color mode");
            let total_bytes = data.bytes.len();
            println!("  Total bytes to encode: {}", total_bytes);
            
            // In color mode, each pixel can store 3 bytes (RGB)
            let bytes_per_pixel = 3;
            let bytes_per_frame = pixels_per_frame * bytes_per_pixel;
            let frames_needed = (total_bytes as f64 / bytes_per_frame as f64).ceil() as u32;
            println!("  Frames needed: {}", frames_needed);
            
            // Generate a sample image to demonstrate the encoding
            let img_width = settings.width as u32;
            let img_height = settings.height as u32;
            let mut img = RgbImage::new(img_width, img_height);
            
            // Fill the image with the color data pattern
            let block_size = settings.size as u32;
            let mut byte_index = 0;
            
            for y in (0..img_height).step_by(block_size as usize) {
                for x in (0..img_width).step_by(block_size as usize) {
                    if byte_index + 2 < data.bytes.len() {
                        // Use 3 bytes for RGB
                        let r = data.bytes[byte_index];
                        let g = data.bytes[byte_index + 1];
                        let b = data.bytes[byte_index + 2];
                        let color = Rgb([r, g, b]);
                        
                        // Fill the block with the color
                        for by in 0..block_size {
                            for bx in 0..block_size {
                                if y + by < img_height && x + bx < img_width {
                                    img.put_pixel(x + bx, y + by, color);
                                }
                            }
                        }
                        
                        byte_index += 3;
                    }
                }
            }
            
            // Save a sample frame as PNG to show what it looks like
            let sample_path = format!("{}_sample.png", path_str);
            img.save(&sample_path)?;
            println!("  Saved sample frame to: {}", sample_path);
            
            // For the actual data storage, we create our custom file format
            let mut output_bytes = Vec::new();
            
            // Add header to indicate color mode
            output_bytes.extend_from_slice(b"COLVID");
            
            // Add data size as a u32 (4 bytes)
            let data_len = (data.bytes.len() as u32).to_le_bytes();
            output_bytes.extend_from_slice(&data_len);
            
            // Add the actual data
            output_bytes.extend_from_slice(&data.bytes);
            
            // Write the encoded data to the file
            write_bytes(path.clone(), output_bytes)?;
        }
    }
    
    println!("Data encoded successfully to: {}", path.as_ref().display());
    Ok(())
}