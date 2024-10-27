# depgraph

This directory provides a **clear dependency graph** to visualize project structure and its dependencies.

## Updating the Dependency Graph

To update the dependency graph and refresh dependencies, run the `update_workspace` script.

### Installation and Setup

Ensure that **Graphviz** is installed on your system, as the scripts require `dot`, a Graphviz utility. You can download Graphviz [here](https://graphviz.org).

### Windows

1. Install Graphviz.
2. Edit the `update_workspace.bat` file to provide the correct path to `dot.exe`.
3. Run the script:
   ```powershell
   .\scripts\update_workspace.bat
   ```

### MacOS / Linux

1. Install Graphviz.
2. Update the `update_workspace.sh` script to ensure the correct path to `dot` is set.
3. Run the following commands to make the script executable and then run it:
   ```bash
   chmod +x ./scripts/update_workspace.sh
   ./scripts/update_workspace.sh
   ```