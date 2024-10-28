#!/bin/bash
# === Configuration: Set Path to Dot ===
dot="/usr/local/bin/dot"  # Adjust path to 'dot' as needed

# === Dependency Graph Generation for Limitpush Bevy Game Template ===
# This script generates dependency graphs using cargo-depgraph and Graphviz's dot tool.

# Generate implicit dependency graph with deduplicated transitive dependencies (depth = 1)
cargo depgraph --all-deps --depth 1 --dedup-transitive-deps | "$dot" -Tpng -o "depgraph/implicit_graph.png"

# Generate explicit dependency graph without deduplicated transitive dependencies (depth = 1)
cargo depgraph --all-deps --depth 1 | "$dot" -Tpng -o "depgraph/explicit_graph.png"

# Generate full explicit dependency graph without deduplication
cargo depgraph --all-deps | "$dot" -Tpng -o "depgraph/full_explicit_graph.png"

# Generate full implicit dependency graph with deduplicated transitive dependencies
cargo depgraph --all-deps --dedup-transitive-deps | "$dot" -Tpng -o "depgraph/full_implicit_graph.png"

# Generate graph for internal crates only
cargo depgraph --workspace-only | "$dot" -Tpng -o "depgraph/internal_crates_graph.png"

# === Bevy Game-Specific Crate Focused Graphs ===
# Each graph focuses on a critical crate within the Bevy ecosystem.
# Note: Crate-focus graphs are always implicit by default.

# Focus on wgpu-core crate for Bevy game development
cargo depgraph --all-deps --dedup-transitive-deps --focus "wgpu-core" | "$dot" -Tpng -o "depgraph/crates-focus/wgpu-core_graph.png"

# Focus on Bevy crate
cargo depgraph --all-deps --dedup-transitive-deps --focus "bevy" | "$dot" -Tpng -o "depgraph/crates-focus/bevy_graph.png"

# Focus on bevy_render crate
cargo depgraph --all-deps --dedup-transitive-deps --focus "bevy_render" | "$dot" -Tpng -o "depgraph/crates-focus/bevy_render_graph.png"

# Script Complete
echo "Dependency graphs have been generated successfully."
