#!/bin/bash

# Function to install git if not installed
install_git() {
    if ! command -v git &> /dev/null; then
        echo "Git is not installed. Installing git..."

        if [[ "$OSTYPE" == "linux-gnu"* ]]; then
            # Linux: Install git using apt, yum, or dnf
            if command -v apt &> /dev/null; then
                sudo apt update && sudo apt install -y git
            elif command -v yum &> /dev/null; then
                sudo yum install -y git
            elif command -v dnf &> /dev/null; then
                sudo dnf install -y git
            else
                echo "No suitable package manager found. Please install git manually."
                exit 1
            fi

        elif [[ "$OSTYPE" == "darwin"* ]]; then
            # macOS: Install git using Homebrew or Xcode CLI tools
            if command -v brew &> /dev/null; then
                brew install git
            else
                echo "Installing Xcode Command Line Tools for Git..."
                xcode-select --install
            fi

        else
            echo "Unsupported OS. Please install git manually."
            exit 1
        fi
    else
        echo "Git is already installed."
    fi
}

# Function to install Rust if not installed
install_rust() {
    if ! command -v rustc &> /dev/null; then
        echo "Rust is not installed. Installing Rust..."

        # Download and install Rust using rustup
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

        # Source the Cargo environment to load Rust immediately after installation
        source "$HOME/.cargo/env"
        
        echo "Rust installation complete!"
    else
        echo "Rust is already installed. Version: $(rustc --version)"
    fi
}

# Function to install development tools (build-essential on Linux)
install_dev_tools() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if command -v apt &> /dev/null; then
            echo "Installing development tools (build-essential)..."
            sudo apt update && sudo apt install -y build-essential
        elif command -v yum &> /dev/null; then
            sudo yum groupinstall "Development Tools" -y
        elif command -v dnf &> /dev/null; then
            sudo dnf groupinstall "Development Tools" -y
        else
            echo "No suitable package manager found. Please install build tools manually."
            exit 1
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        # For macOS, Xcode Command Line Tools already includes development tools.
        echo "Installing Xcode Command Line Tools..."
        xcode-select --install
    else
        echo "Unsupported OS. Please install development tools manually."
        exit 1
    fi
}

# Main logic starts here
# Define the GitHub repository URL and binary name
REPO_URL="https://github.com/Lukas-Les/vinseers.git"
BINARY_NAME="vinseers-gui"
REPOSITORY_DIR="/tmp/vinseers"

# Install Git if needed
install_git

# Install Rust if needed
install_rust

# Install development tools if needed
install_dev_tools

# Create a temporary directory to clone the repository
mkdir -p $REPOSITORY_DIR

# Clone the repository
git clone "$REPO_URL" "/tmp/vinseers"


# Change directory to the cloned repo (assuming the repo's directory name matches the binary name)
cd "${REPOSITORY_DIR}" || { echo "Repository directory not found"; exit 1; }

# Compile the Rust code
echo "Compiling Rust project..."
cargo build --bin "$BINARY_NAME" --release

# Find the compiled binary
BINARY_PATH="$REPOSITORY_DIR/target/release/$BINARY_NAME"
if [ -f "$BINARY_PATH" ]; then
    echo "Binary compiled successfully at $BINARY_PATH"
else
    echo "Compilation failed. Binary not found."
    exit 1
fi

# Move the binary to a directory in the user's PATH
TARGET_DIR="$HOME/.local/bin"
if [ ! -d "$TARGET_DIR" ]; then
    mkdir -p "$TARGET_DIR"
fi

echo "Moving binary to $TARGET_DIR..."
mv "$BINARY_PATH" "$TARGET_DIR/"


echo "Binary moved to $TARGET_DIR and is ready to use!"
echo "You can now run '$BINARY_NAME' from any terminal."

echo "Cleaning up temporary files..."
rm -rf "$REPOSITORY_DIR"

echo "Installation complete! Run '$BINARY_NAME' to start the application."
