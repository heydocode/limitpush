#!/bin/bash

# Update dependencies
echo "Updating dependencies..."
cargo update
echo "Dependencies updated."

# Check for vulnerabilities (requires 'cargo-audit' crate)
echo "Checking for vulnerabilities..."
cargo install cargo-audit &>/dev/null
cargo audit
echo "Vulnerability check completed."

# Format code
echo "Formatting code..."
cargo fmt
echo "Code formatted."

# Run Clippy linter, ignore errors to continue script
echo "Running Clippy linter..."
cargo install clippy &>/dev/null
cargo clippy --workspace --all-targets --all-features -- -Dwarnings || echo "Clippy found issues, but continuing..."

# Build project to save compilation time later
echo "Building project..."
cargo build
echo "Project setup complete."
