# depgraph

This directory contains a **dependency graph** that visualizes the project’s structure and its dependencies. This graph helps you gain insight into how different components interact within **Limitpush** bevy template. This is particularly useful when debugging, optimizing, or adapting the project for specific platforms.

## Why Use the Dependency Graph?

The dependency graph is an invaluable tool for debugging platform-specific issues and understanding hidden dependencies in your project. Here are some key benefits:

- **Debugging Platform-Specific Issues:** Dependencies might be bringing in features that don’t support all platforms, like WebAssembly (WASM). For example, a crate you’re indirectly using could have a non-cross-platform feature enabled, breaking WASM support. The graph reveals these implicit dependencies and helps you trace issues back to their source.
  
- **Optimizing Dependencies:** Sometimes, you may want to reduce the number of crates used in the project, especially those introduced as transitive dependencies (dependencies of dependencies). The graph provides a comprehensive view, so you know which crates are safe to remove or replace.
  
- **Facilitating Community Contributions:** With the dependency graph, it’s easier to identify areas for improvement or suggest changes to third-party crates. If you find a problem with a dependency, you might contribute a feature toggle to a crate’s repository, fix issues, or even fork it if necessary. These contributions can benefit not just your project but the broader Bevy community.

## Updating the Dependency Graph

You can update the dependency graph and refresh dependencies by running the appropriate script for your OS. The scripts are designed to automate the process, ensuring your graph stays current.

### Installation and Setup

To run these scripts, you’ll need **Graphviz** installed on your system, as the scripts require the `dot` utility from Graphviz. You can download it [here](https://graphviz.org).

### Windows

1. Install Graphviz.
2. Open `update_graphs.bat` and adjust the `dot` variable to the location of `dot.exe`.
3. Run the script:
   ```powershell
   .\scripts\update_graphs.bat
   ```

### MacOS / Linux

1. Install Graphviz.
2. Open `update_graphs.sh` and set the `dot` variable to the location of `dot` on your system.
3. Make the script executable and run it:
   ```bash
   chmod +x ./scripts/update_graphs.sh
   ./scripts/update_graphs.sh
   ```

With the dependency graph, you’re equipped to handle compatibility issues and dependency management with greater ease, helping your Bevy projects perform consistently across platforms. Happy coding!