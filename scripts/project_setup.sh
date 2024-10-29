#!/bin/bash

# Initialize log directory and file
log_dir="script_logs/project_setup"
mkdir -p "$log_dir"

# Generate timestamped log file name
timestamp=$(date +"%Y-%m-%d_%H-%M-%S")
log_file="$log_dir/${timestamp}.log"

# Start logging process
echo "-------------------------------" | tee -a "$log_file"
echo "Starting project setup..." | tee -a "$log_file"
echo "-------------------------------" | tee -a "$log_file"
echo "Log file: $log_file"
echo "-------------------------------"
echo ""

# Update dependencies
echo "-------------------------------" | tee -a "$log_file"
echo "Updating dependencies..." | tee -a "$log_file"
if cargo update >> "$log_file" 2>&1; then
    echo "Dependencies updated." | tee -a "$log_file"
else
    echo "Failed to update dependencies." | tee -a "$log_file"
fi
echo "-------------------------------" | tee -a "$log_file"
echo ""

# Check for vulnerabilities
echo "-------------------------------" | tee -a "$log_file"
echo "Checking for vulnerabilities..." | tee -a "$log_file"

# Install 'cargo-audit' if missing
if ! cargo audit --version > /dev/null 2>&1; then
    echo "'cargo-audit' is not installed. Installing..." | tee -a "$log_file"
    cargo install cargo-audit >> "$log_file" 2>&1
else
    echo "'cargo-audit' is already installed." | tee -a "$log_file"
fi

echo "Running vulnerability check..." | tee -a "$log_file"
if cargo audit >> "$log_file" 2>&1; then
    echo "Vulnerability check completed." | tee -a "$log_file"
else
    echo "Vulnerability check completed with issues." | tee -a "$log_file"
fi
echo "-------------------------------" | tee -a "$log_file"
echo ""

# Format code
echo "-------------------------------" | tee -a "$log_file"
echo "Formatting code..." | tee -a "$log_file"
if cargo fmt >> "$log_file" 2>&1; then
    echo "Code formatted." | tee -a "$log_file"
else
    echo "Code formatting failed." | tee -a "$log_file"
fi
echo "-------------------------------" | tee -a "$log_file"
echo ""

# Run Clippy linter
echo "-------------------------------" | tee -a "$log_file"
echo "Running Clippy linter..." | tee -a "$log_file"

# Install 'clippy' if missing
if ! cargo clippy --version > /dev/null 2>&1; then
    echo "'clippy' is not installed. Installing..." | tee -a "$log_file"
    rustup component add clippy >> "$log_file" 2>&1
else
    echo "'clippy' is already installed." | tee -a "$log_file"
fi

echo "Running Clippy..." | tee -a "$log_file"
cargo clippy --workspace --all-targets --all-features -- -Dwarnings >> "$log_file" 2>&1 || echo "Clippy found issues, but continuing..." | tee -a "$log_file"
echo "Clippy finished." | tee -a "$log_file"
echo "-------------------------------" | tee -a "$log_file"
echo ""

# Build project
echo "-------------------------------" | tee -a "$log_file"
echo "Building project..." | tee -a "$log_file"
if cargo build >> "$log_file" 2>&1; then
    echo "Project setup complete." | tee -a "$log_file"
else
    echo "Project build failed." | tee -a "$log_file"
fi
echo "-------------------------------" | tee -a "$log_file"
echo ""

echo "-------------------------------"
echo "Happy coding!"
echo "Log saved to: $log_file"
echo "-------------------------------"
