@echo off
setlocal enabledelayedexpansion

REM Create directory for logs if it doesn’t exist
set "log_dir=script_logs/compile_all_targets"
if not exist "%log_dir%" mkdir "%log_dir%"

REM Generate a timestamped log file name
for /f "tokens=1-5 delims=/: " %%d in ("%date% %time%") do (
    set "log_file=%log_dir%/%%d-%%e-%%f_%%g-%%h.log"
)

REM Define an array of targets
set targets=wasm32-unknown-unknown i686-pc-windows-msvc x86_64-pc-windows-msvc mobile-build

REM Initialize counters and an array for unsuccessful targets
set "unsuccessful_targets="
set "success_count=0"
set "failure_count=0"

REM Log startup info in both console and log file
REM Into console, add an empty line in order to clean the line
REM Indeed, sometimes the console can contain some letters in the first line
echo.
echo -------------------------------
echo Starting compilation process...
echo Targets to compile: %targets%
echo -------------------------------
echo.

REM We aren't adding here an empty line because that's a text file
REM And files are operated with text editors, no bugs should occur
echo ------------------------------- >> "%log_file%"
echo Starting compilation process... >> "%log_file%"
echo Targets to compile: %targets% >> "%log_file%"
echo ------------------------------- >> "%log_file%"
echo. >> "%log_file%"

REM Compile each target
for %%t in (%targets%) do (
    echo.
    echo. >> "%log_file%"
    echo -------------------------------
    echo ------------------------------- >> "%log_file%"
    echo Compiling for target: %%t
    echo Compiling for target: %%t >> "%log_file%"

    REM Handle mobile build specifically
    if %%t==mobile-build (
        REM Check if cargo-apk is installed
        cargo apk version >nul 2>&1
        if !ERRORLEVEL! neq 0 (
            echo 'cargo-apk' is not installed. Installing...
            echo -------------------------------
            echo 'cargo-apk' is not installed. Installing... >> "%log_file%"
            cargo install cargo-apk >> "%log_file%" 2>&1
            echo ------------------------------- >> "%log_file%"
        ) else (
            echo 'cargo-apk' is already installed.
            echo 'cargo-apk' is already installed. >> "%log_file%"
        )

        echo Building mobile APK...
        echo Building mobile APK... >> "%log_file%"
        cargo apk build -p mobile > output.log 2>&1
    ) else (
        REM Standard Rust compilation for other targets
        rustup target add %%t > output.log 2>&1
        cargo build --target %%t >> output.log 2>&1
    )

    REM Append the full output log to the main log file
    type output.log >> "%log_file%"

    REM Display last 25 lines of output log in the console for readability
    echo Showing last 25 lines of output log for %%t:
    powershell -Command "Get-Content output.log -Tail 25"

    REM Check for errors
    if errorlevel 1 (
        echo Compilation failed for %%t
        echo Compilation failed for %%t >> "%log_file%"
        set "unsuccessful_targets=!unsuccessful_targets! %%t"
        set /a failure_count+=1
    ) else (
        echo Compilation successful for %%t
        echo Compilation successful for %%t >> "%log_file%"
        set /a success_count+=1
    )

    echo ------------------------------- >> "%log_file%"
    echo. >> "%log_file%"
    echo -------------------------------
    echo.
)

REM Display summary and log it
echo.
echo -------------------------------
echo. >> "%log_file%"
echo ------------------------------- >> "%log_file%"
echo Compilation process completed.
echo Compilation process completed. >> "%log_file%"
if defined unsuccessful_targets (
    echo The following targets failed to compile: !unsuccessful_targets!
    echo The following targets failed to compile: !unsuccessful_targets! >> "%log_file%"
) else (
    echo All targets compiled successfully!
    echo All targets compiled successfully! >> "%log_file%"
)
echo ------------------------------- >> "%log_file%"
echo. >> "%log_file%"
echo -------------------------------
echo.

echo.
echo -------------------------------
echo. >> "%log_file%"
echo ------------------------------- >> "%log_file%"
echo Summary:
echo Success: !success_count!
echo Failed: !failure_count!
echo Summary: >> "%log_file%"
echo Success: !success_count! >> "%log_file%"
echo Failed: !failure_count! >> "%log_file%"
echo ------------------------------- >> "%log_file%"
echo. >> "%log_file%"
echo -------------------------------
echo.

echo Log saved to: %log_file%

endlocal
pause
