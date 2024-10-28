#!/bin/bash

# Update cargo dependencies
cargo update

# Generate dependency graphs
cargo depgraph --all-deps --depth 1 --hide bevy --dedup-transitive-deps | dot -Tpng -o depgraph/implicit_graph.png
cargo depgraph --all-deps --depth 1 --hide bevy | dot -Tpng -o depgraph/explicit_graph.png
cargo depgraph --all-deps | dot -Tpng -o depgraph/full_explicit_graph.png
cargo depgraph --all-deps --dedup-transitive-deps | dot -Tpng -o depgraph/full_implicit_graph.png

# Build the project
cargo build
