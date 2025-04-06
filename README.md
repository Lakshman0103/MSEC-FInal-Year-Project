# Steganographic Data Handling

A Rust-based tool for encoding any file type into binary videos and decoding them back to retrieve the original data using steganography techniques.

## Overview

This tool allows you to hide data within video files using steganography techniques. The system encodes files into binary patterns visually represented as video frames, which can later be decoded back to their original form.

## Features

- **Multiple Encoding Modes**:
  - **Binary Mode**: Uses black and white pixels (1 bit per pixel)
  - **Color Mode**: Uses RGB values (24 bits per pixel)

- **Encoding Presets**:
  - **MaxEfficiency**: Optimized for maximum data density
  - **Optimal**: Balanced for good compression resistance
  - **Paranoid**: Maximum resistance to compression artifacts

- **Customizable Settings**:
  - Resolution: 144p to 720p
  - Block size: Adjustable pixel blocks for data encoding
  - FPS: Configurable frame rate
  - Threads: Multi-threaded processing

- **File Format Support**:
  - Custom .binvid format for simple storage
  - Future support for standard video formats

- **YouTube Integration**:
  - Download videos to extract hidden data

## Usage

### Building from Source

```bash
# Install required dependencies (Kali Linux)
sudo apt update
sudo apt install build-essential pkg-config libssl-dev

# Clone the repository
git clone https://github.com/yourusername/steganographic_data_handling.git
cd steganographic_data_handling

# Build the project
cargo build --release
```

### Encoding a File

```bash
# Using command line arguments
./steganographic_data_handling embed -i your_file.txt

# Using a preset
./steganographic_data_handling embed -i your_file.txt --preset Optimal

# Using custom settings
./steganographic_data_handling embed -i your_file.txt --mode Binary --block_size 2 --resolution 720p
```

### Decoding a File

```bash
# Basic decode
./steganographic_data_handling dislodge -i encoded_video.binvid -o extracted_file.bin
```

### Downloading a YouTube Video

```bash
# Download a video to decode
./steganographic_data_handling download -u "https://youtu.be/example"
```

### Interactive Mode

Simply run the application without any arguments for an interactive command-line interface:

```bash
./steganographic_data_handling
```

## How It Works

1. **Encoding Process**:
   - Input file is read as binary data
   - Data is converted to a binary stream of 1s and 0s
   - Binary data is visually encoded as pixels (black/white or RGB values)
   - Pixels are arranged into frames according to selected settings
   - Frames are combined into a video or stored in our custom .binvid format

2. **Decoding Process**:
   - Video frames are read and converted back to binary data
   - Binary stream is reconstructed into the original file format
   - Output file is identical to the original input

## Requirements

- Rust 1.60 or higher
- OpenSSL development libraries
- Basic image and video processing tools (automatically downloaded if needed)

## License

This project is licensed under the MIT License - see the LICENSE file for details.