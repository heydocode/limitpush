@echo off
cargo update
cargo depgraph --all-deps --depth 1 --hide bevy --dedup-transitive-deps | "D:\graphviz\bin\dot.exe" -Tpng -o "depgraph\implicit_graph.png"
cargo depgraph --all-deps --depth 1 --hide bevy | "D:\graphviz\bin\dot.exe" -Tpng -o "depgraph\explicit_graph.png"
cargo depgraph --all-deps | "D:\graphviz\bin\dot.exe" -Tpng -o "depgraph\full_explicit_graph.png"
cargo depgraph --all-deps --dedup-transitive-deps | "D:\graphviz\bin\dot.exe" -Tpng -o "depgraph\full_implicit_graph.png"
cargo build