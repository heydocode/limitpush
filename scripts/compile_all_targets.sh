#!/bin/bash
set -e

# Enable nullglob to handle cases where no files match a pattern
shopt -s nullglob

# Create directory for logs if it doesn’t exist
log_dir="script_logs/compile_all_targets"
mkdir -p "$log_dir"

# Generate a timestamped log file name
timestamp=$(date +"%Y-%m-%d_%H-%M-%S")
log_file="$log_dir/$timestamp.log"

# Define an array of targets
targets=("wasm32-unknown-unknown" "i686-pc-windows-msvc" "x86_64-pc-windows-msvc" "mobile-build")

# Initialize counters and an array for unsuccessful targets
unsuccessful_targets=()
success_count=0
failure_count=0

# Log startup info in both console and log file
echo -e "\n-------------------------------"
echo "Starting compilation process..."
echo "Targets to compile: ${targets[*]}"
echo "-------------------------------"
echo -e "\n" | tee -a "$log_file"

echo "-------------------------------" >> "$log_file"
echo "Starting compilation process..." >> "$log_file"
echo "Targets to compile: ${targets[*]}" >> "$log_file"
echo "-------------------------------" >> "$log_file"
echo "" >> "$log_file"

# Compile each target
for target in "${targets[@]}"; do
    echo -e "\n-------------------------------"
    echo "Compiling for target: $target"
    echo "-------------------------------" | tee -a "$log_file"

    # Handle mobile build specifically
    if [[ $target == "mobile-build" ]]; then
        # Check if cargo-apk is installed
        if ! cargo apk version &>/dev/null; then
            echo "'cargo-apk' is not installed. Installing..."
            echo "'cargo-apk' is not installed. Installing..." >> "$log_file"
            cargo install cargo-apk >> "$log_file" 2>&1
        else
            echo "'cargo-apk' is already installed."
            echo "'cargo-apk' is already installed." >> "$log_file"
        fi

        echo "Building mobile APK..."
        echo "Building mobile APK..." >> "$log_file"
        cargo apk build -p mobile > output.log 2>&1
    else
        # Standard Rust compilation for other targets
        rustup target add "$target" >> output.log 2>&1
        cargo build --target "$target" >> output.log 2>&1
    fi

    # Append the full output log to the main log file
    cat output.log >> "$log_file"

    # Display last 25 lines of output log in the console for readability
    echo "Showing last 25 lines of output log for $target:"
    tail -n 25 output.log

    # Check for errors
    if [[ $? -ne 0 ]]; then
        echo "Compilation failed for $target"
        echo "Compilation failed for $target" >> "$log_file"
        unsuccessful_targets+=("$target")
        ((failure_count++))
    else
        echo "Compilation successful for $target"
        echo "Compilation successful for $target" >> "$log_file"
        ((success_count++))
    fi

    echo "-------------------------------" >> "$log_file"
    echo "" >> "$log_file"
    echo "-------------------------------"
done

# Display summary and log it
echo -e "\n-------------------------------"
echo "Compilation process completed."
echo "Compilation process completed." >> "$log_file"

if [[ ${#unsuccessful_targets[@]} -gt 0 ]]; then
    echo "The following targets failed to compile: ${unsuccessful_targets[*]}"
    echo "The following targets failed to compile: ${unsuccessful_targets[*]}" >> "$log_file"
else
    echo "All targets compiled successfully!"
    echo "All targets compiled successfully!" >> "$log_file"
fi

echo "-------------------------------" >> "$log_file"
echo "" >> "$log_file"
echo "-------------------------------"

echo -e "\n-------------------------------"
echo "Summary:"
echo "Success: $success_count"
echo "Failed: $failure_count"
echo "Summary:" >> "$log_file"
echo "Success: $success_count" >> "$log_file"
echo "Failed: $failure_count" >> "$log_file"
echo "-------------------------------" >> "$log_file"
echo "" >> "$log_file"
echo "-------------------------------"

echo "Log saved to: $log_file"
