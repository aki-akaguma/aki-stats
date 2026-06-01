# Code Review: aki-stats

## Overview
`aki-stats` is a CLI tool designed to provide text statistics similar to the `wc` command, with specialized features such as excluding line terminators from byte counts and supporting localized number formatting. The codebase is well-structured, follows Rust idioms, and leverages robust libraries for I/O abstraction (`runnel`) and CLI parsing (`flood_tide`).

## Strengths

### 1. Robust Architecture
The project demonstrates a clear separation of concerns:
- `main.rs`: Entry point and environment setup.
- `lib.rs`: Library interface and top-level execution coordination.
- `run.rs`: Core processing logic and statistics calculation.
- `conf/`: Command-line option parsing and configuration management.
- `util/`: Reusable utility modules for error handling and specialized parameter types.

### 2. Effective I/O Abstraction
The use of `RunnelIoe` for abstracting standard input, output, and error streams is a highlight. This design significantly improves testability by allowing the tool to operate on virtual streams during integration tests.

### 3. Graceful Error Handling
The implementation of the `BrokenPipeError` trait for `anyhow::Error` and `anyhow::Result` ensures that the tool handles `SIGPIPE` gracefully. This is essential for CLI tools intended to be used in shell pipelines (e.g., `aki-stats | head`).

### 4. High-Quality Documentation
The project is well-documented with extensive doc comments in `lib.rs` and a comprehensive `README.md`. The inclusion of runnable examples in the library documentation (`#[test]`-able examples) ensures the documentation remains accurate.

### 5. Performance Considerations
The core loop in `run.rs` is optimized to avoid character-level iteration when only line and byte counts are requested, which improves performance for large files.

## Recommendations for Improvement

### 1. Simplify Vector Iteration in `run.rs`
In `run_0`, the ASCII map output is processed using `reverse()` and `pop()`:
```rust
let mut vec = make_out_s_from_map_ascii_2(&map_ascii)?;
vec.reverse();
while let Some(v) = vec.pop() {
    sioe.pg_out().write_line(v)?;
}
```
This can be simplified to a standard iterator, which is more idiomatic and readable:
```rust
for v in make_out_s_from_map_ascii_2(&map_ascii)? {
    sioe.pg_out().write_line(v)?;
}
```

### 2. Minor Redundancy in `main.rs`
In `main.rs`, the program name is extracted from `env::args()` but then overridden by `CARGO_PKG_NAME`:
```rust
let mut env_args: Vec<String> = std::env::args().collect();
let _program = env_args.remove(0);
let program = env!("CARGO_PKG_NAME");
```
Using `std::env::args().skip(1)` would be a slightly cleaner way to collect the arguments without needing to manually remove and ignore the first element.

### 3. Efficiency of `run_00` with Multiple Flags
When multiple flags (e.g., `-c`, `-w`, and `--map-ascii`) are active, `run_00` iterates over Unicode characters. While correct for character and word counts, the ASCII mapping logic within the same loop checks `c.is_ascii()` for every character. 
If performance becomes a bottleneck for very large files with many non-ASCII characters, consider if the ASCII mapping can be performed on the raw bytes independently, although the current approach of a single pass over characters is likely acceptable for most use cases.

### 4. UTF-8 Dependency
The tool relies on `sioe.pg_in().lines()`, which will return an error if the input is not valid UTF-8. While this is explicitly documented in the "Specifications" section of `lib.rs`, users of a `wc`-like tool might expect basic byte counting to work even on non-UTF-8 or binary files. If strict UTF-8 support is the intentional design, the current implementation is correct.

## Conclusion
The `aki-stats` project is a high-quality implementation of a specialized text utility. It demonstrates a sophisticated use of the Rust ecosystem and adheres to CLI development best practices. The suggested improvements are minor refinements aimed at enhancing idiomatic quality and readability.

---
Review Date: 2026-06-01
Reviewer: Gemini CLI Agent
