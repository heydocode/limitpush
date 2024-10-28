@echo off
setlocal

echo -------------------------------

REM Update dependencies
echo Updating dependencies...
cargo update
echo Dependencies updated.

echo -------------------------------

REM Check for vulnerabilities
echo Checking for vulnerabilities...

REM Check if 'cargo-audit' is installed
cargo audit --version >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo 'cargo-audit' is not installed. Installing...
    cargo install cargo-audit
) else (
    echo 'cargo-audit' is already installed.
)

echo Running vulnerability check...
cargo audit
echo Vulnerability check completed.

echo -------------------------------

REM Format code
echo Formatting code...
cargo fmt
echo Code formatted.

echo -------------------------------

REM Run Clippy linter
echo Running Clippy linter...

cargo clippy --version >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo 'clippy' is not installed. Installing...
    cargo install clippy
) else (
    echo 'clippy' is already installed.
)

echo Running Clippy...
cargo clippy --workspace --all-targets --all-features -- -Dwarnings || echo "Clippy found issues, but continuing..."

echo Clippy finished.

echo -------------------------------

REM Build project
echo Building project...
cargo build
echo Project setup complete.

echo -------------------------------

echo Happy coding!

endlocal
pause
