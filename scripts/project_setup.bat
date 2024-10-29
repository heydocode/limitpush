@echo off
setlocal enabledelayedexpansion

REM Initialize log directory and file
set "log_dir=script_logs/project_setup"
if not exist "%log_dir%" mkdir "%log_dir%"

REM Generate timestamped log file name
for /f "tokens=1-5 delims=/: " %%d in ("%date% %time%") do (
    set "log_file=%log_dir%/%%d-%%e-%%f_%%g-%%h.log"
)

REM Start logging process
echo ------------------------------- >> "%log_file%"
echo Starting project setup... >> "%log_file%"
echo ------------------------------- >> "%log_file%"
echo Log file: %log_file%
echo -------------------------------
echo.
echo.
echo -------------------------------
echo Starting project setup...
echo -------------------------------
echo.
echo. >> "%log_file%"

REM Update dependencies
echo. >> "%log_file%"
echo ------------------------------- >> "%log_file%"
echo -------------------------------
echo Updating dependencies...
echo Updating dependencies... >> "%log_file%"
cargo update >> "%log_file%" 2>&1
if %ERRORLEVEL% neq 0 (
    echo Failed to update dependencies. >> "%log_file%"
    echo Failed to update dependencies.
) else (
    echo Dependencies updated. >> "%log_file%"
    echo Dependencies updated.
)
echo ------------------------------- >> "%log_file%"
echo -------------------------------
echo. >> "%log_file%"

REM Check for vulnerabilities
echo. >> "%log_file%"
echo ------------------------------- >> "%log_file%"
echo -------------------------------
echo Checking for vulnerabilities...
echo Checking for vulnerabilities... >> "%log_file%"

REM Install 'cargo-audit' if missing
cargo audit --version >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo 'cargo-audit' is not installed. Installing...
    echo 'cargo-audit' is not installed. Installing... >> "%log_file%"
    cargo install cargo-audit >> "%log_file%" 2>&1
) else (
    echo 'cargo-audit' is already installed.
    echo 'cargo-audit' is already installed. >> "%log_file%"
)

echo Running vulnerability check...
echo Running vulnerability check... >> "%log_file%"
cargo audit >> "%log_file%" 2>&1
if %ERRORLEVEL% neq 0 (
    echo Vulnerability check completed with issues. >> "%log_file%"
    echo Vulnerability check completed with issues.
) else (
    echo Vulnerability check completed. >> "%log_file%"
    echo Vulnerability check completed.
)
echo ------------------------------- >> "%log_file%"
echo -------------------------------
echo. >> "%log_file%"

REM Format code
echo. >> "%log_file%"
echo ------------------------------- >> "%log_file%"
echo -------------------------------
echo Formatting code...
echo Formatting code... >> "%log_file%"
cargo fmt >> "%log_file%" 2>&1
if %ERRORLEVEL% neq 0 (
    echo Code formatting failed. >> "%log_file%"
    echo Code formatting failed.
) else (
    echo Code formatted. >> "%log_file%"
    echo Code formatted.
)
echo ------------------------------- >> "%log_file%"
echo -------------------------------
echo. >> "%log_file%"

REM Run Clippy linter
echo. >> "%log_file%"
echo ------------------------------- >> "%log_file%"
echo -------------------------------
echo Running Clippy linter...
echo Running Clippy linter... >> "%log_file%"

REM Install 'clippy' if missing
cargo clippy --version >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo 'clippy' is not installed. Installing...
    echo 'clippy' is not installed. Installing... >> "%log_file%"
    cargo install clippy >> "%log_file%" 2>&1
) else (
    echo 'clippy' is already installed.
    echo 'clippy' is already installed. >> "%log_file%"
)

echo Running Clippy...
echo Running Clippy... >> "%log_file%"
cargo clippy --workspace --all-targets --all-features -- -Dwarnings >> "%log_file%" 2>&1 || echo "Clippy found issues, but continuing..." >> "%log_file%"
echo Clippy finished. >> "%log_file%"
echo Clippy finished.
echo ------------------------------- >> "%log_file%"
echo -------------------------------
echo. >> "%log_file%"

REM Build project
echo. >> "%log_file%"
echo ------------------------------- >> "%log_file%"
echo -------------------------------
echo Building project...
echo Building project... >> "%log_file%"
cargo build >> "%log_file%" 2>&1
if %ERRORLEVEL% neq 0 (
    echo Project build failed. >> "%log_file%"
    echo Project build failed.
) else (
    echo Project setup complete. >> "%log_file%"
    echo Project setup complete.
)
echo ------------------------------- >> "%log_file%"
echo -------------------------------
echo.

echo -------------------------------
echo Happy coding!
echo Log saved to: %log_file%
echo -------------------------------
echo.

endlocal
pause
