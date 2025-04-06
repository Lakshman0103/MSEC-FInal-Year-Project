#!/bin/bash

# Build for Kali Linux script
echo "Building Steganographic Data Handling for Kali Linux..."

# Check if cargo is installed
if ! command -v cargo &> /dev/null
then
    echo "Cargo is not installed. Installing Rust and Cargo..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Install dependencies if needed
echo "Checking for required dependencies..."
pkgs=("pkg-config" "libssl-dev" "build-essential")
for pkg in "${pkgs[@]}"; do
    dpkg -s "$pkg" >/dev/null 2>&1 || {
        echo "Installing $pkg..."
        apt-get update && apt-get install -y "$pkg"
    }
done

# Build in release mode
echo "Building in release mode..."
cargo build --release

# Create output directory
mkdir -p ./release

# Copy binary and necessary files
echo "Copying files to release directory..."
cp ./target/release/steganographic_data_handling ./release/
cp ./README.md ./release/ 2>/dev/null || echo "No README.md found"
cp ./LICENSE ./release/ 2>/dev/null || echo "No LICENSE found"

# Create a small example file if none exists
if [ ! -f ./test_file.txt ]; then
    echo "Creating example test file..."
    echo "This is a test file for the Steganographic Data Handling." > ./release/test_file.txt
else
    cp ./test_file.txt ./release/
fi

# Create a simple README if it doesn't exist
if [ ! -f ./release/README.md ]; then
    echo "Creating basic README..."
    cat > ./release/README.md << EOF
# Steganographic Data Handling

A tool for encoding any file into binary video format and decoding it back.

## Usage

### Embed (encode)
\`\`\`
./steganographic_data_handling embed -i your_file.txt
\`\`\`

### Dislodge (decode)
\`\`\`
./steganographic_data_handling dislodge -i video.binvid -o extracted_file.bin
\`\`\`

### Download YouTube video
\`\`\`
./steganographic_data_handling download -u "https://youtu.be/example"
\`\`\`

### Interactive Mode
Simply run without arguments:
\`\`\`
./steganographic_data_handling
\`\`\`
EOF
fi

# Create zip file
echo "Creating ZIP file..."
cd release
zip -r ../steganographic_data_handling_kali.zip *
cd ..

echo "Build complete! The release package is available at: steganographic_data_handling_kali.zip"
echo "You can upload this file to your GitHub repository."