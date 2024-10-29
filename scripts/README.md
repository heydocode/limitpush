# Development Scripts

This project contains scripts to help automate your workflow when developing with the Bevy Limitpush template. Below are four scripts designed to streamline setup, compilation, tool installation, and graph updates.

## Script 1: Project Setup & Pre-Build

This script sets up your project by:

- Updating dependencies
- Checking for vulnerable dependencies
- Formatting code
- Running Clippy to catch any linting issues
- Building the project

**Run it on Windows:**
```powershell
.\scripts\project_setup.bat
```

**Run it on MacOS/Linux:**
```bash
./scripts/project_setup.sh
```

## Script 2: Compile for Multiple Targets

This script compiles the project for multiple targets specified at the beginning of the script. Add or remove targets as needed.

**Run it on Windows:**
```powershell
.\scripts\compile_all_targets.bat
```

**Run it on MacOS/Linux:**
```bash
./scripts/compile_all_targets.sh
```

## Script 3: Update Dependency Graphs

This script generates various dependency graphs using the `cargo-depgraph` crate and Graphviz’s `dot` tool. The generated graphs provide visual insights into the project’s dependencies, including implicit and explicit dependencies and focus graphs for critical crates like `wgpu-core`, `bevy`, and `bevy_render`.

**Run it on Windows:**
```powershell
.\scripts\update_graphs.bat
```

**Run it on MacOS/Linux:**
```bash
./scripts/update_graphs.sh
```

### Graphs Generated

- **Implicit Dependency Graph**: Deduplicated transitive dependencies with depth = 1.
- **Explicit Dependency Graph**: All dependencies, no deduplication, depth = 1.
- **Full Explicit Graph**: Complete dependency graph without deduplication.
- **Full Implicit Graph**: Complete dependency graph with deduplicated transitive dependencies.
- **Internal Crates Graph**: Dependency graph focusing only on internal workspace crates.
- **Focus Graphs**: Dedicated graphs for critical crates (`wgpu-core`, `bevy`, and `bevy_render`).

These graphs are saved in the `depgraph` directory. Each graph file is named according to the dependencies it visualizes, e.g., `implicit_graph.png`, `internal_crates_graph.png`, etc.

Each script provides clear output for each step, so you know what is being executed and when it completes. Happy coding!