# AGENTS.md - Development Guidelines for Dead Store Elimination (DSE) Pass

## Build & Test Commands
- **Build**: `cargo build` or `cargo build --release`
- **Run**: `cargo run --release -- -i <input.ll> -o <output.bc>`
- **Test (all)**: `cargo test --verbose`
- **Test (single)**: `cargo test test_dse_target_optimization` or `cargo test <test_name>`
- **Lint**: `cargo clippy --all-targets --all-features -- -D warnings`
- **Format**: `cargo fmt --all -- --check` (check) or `cargo fmt` (fix)
- **Docs**: `cargo doc --no-deps --document-private-items`
- **Benchmark**: `./scripts/run_benchmark.sh`

## Code Style & Conventions
- **Edition**: Rust 2024, use latest stable features
- **Imports**: Group std/external/crate imports, use explicit paths
- **Error Handling**: Use `anyhow::Result` for main/CLI, avoid unwrap in library code
- **Naming**: `snake_case` for functions/vars, `PascalCase` for types, descriptive names (e.g., `dead_instructions`, `live_set`)
- **Types**: Explicit lifetimes for LLVM values (`<'ctx>`), use `PointerValue`, `InstructionValue` properly
- **Comments**: Document public APIs with `///`, explain algorithms with inline comments, include correctness invariants
- **Safety**: Never eliminate volatile stores, preserve program semantics, use conservative analysis when uncertain

## Project Context
This is an LLVM optimization pass implementing Dead Store Elimination using backward data flow analysis. Key modules: `analysis.rs` (DSE algorithm), `transform.rs` (IR modification), `main.rs` (CLI). Test fixtures in `tests/fixtures/*.ll` validate correctness and edge cases (volatile protection).
