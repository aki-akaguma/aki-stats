# Code Review for aki-stats

## Overview
`aki-stats` is a well-structured Rust CLI tool that provides text statistics similar to the Linux `wc` command, with additional features like locale-aware number formatting and ASCII character distribution mapping. The project demonstrates a high level of proficiency in Rust, utilizing effective abstractions for I/O and command-line parsing.

## Review Findings

### 1. Correctness and Compatibility
*   **UTF-8 Dependency**: The current implementation uses `RunnelIoe::pg_in().lines()`, which yields `String` results. This design limits the tool to valid UTF-8 input. A true `wc` replacement should ideally handle arbitrary byte streams. If a non-UTF-8 file is encountered, the tool will return an error instead of processing it.
*   **Byte Count Accuracy**: The `lines()` iterator strips line terminators (e.g., `\n`, `\r\n`). Since the byte count is calculated based on the length of these stripped strings, the resulting `bytes` count will be lower than the actual file size as it excludes the line endings.
*   **Word Count Consistency**: The word count logic (`prev_c.is_ascii_whitespace() && !c.is_ascii_whitespace()`) is simple and effective for most cases. However, it's worth verifying if it aligns perfectly with `wc`'s behavior, especially regarding various Unicode whitespace characters.

### 2. Performance and Efficiency
*   **Multiple Iterations**: In the core processing loop (`run_00`), the code iterates over the same line multiple times if different flags are set (once for characters/words and once for ASCII mapping). These could be consolidated into a single pass to improve performance on large files.
*   **Stack vs Heap for Stats**: The `StatsAscii` struct uses a `Vec<u64>` of size 128. Since this size is constant, using a fixed-size array `[u64; 128]` would be more idiomatic and slightly more efficient by avoiding heap allocation and indirection.

### 3. Architecture and Design
*   **I/O Abstraction**: The use of `runnel` for I/O abstraction (`RunnelIoe`) is an excellent design choice. It facilitates unit testing by allowing easy redirection of input/output streams.
*   **Boilerplate Management**: The use of `xtask` for generating CLI-related code (`cmd.help.rs.txt`, etc.) is a clean way to handle repetitive command-line option logic, keeping the main source files focused.
*   **Error Handling**: Leveraging `anyhow` and specifically handling `BrokenPipeError` shows good attention to CLI usability details.
*   **Feature Set**: The inclusion of locale-aware formatting via `num-format` and the ASCII mapping feature are valuable additions that distinguish this tool from standard utilities.

### 4. Code Style and Idioms
*   **Module Structure**: The project is logically divided into `conf`, `run`, and `util` modules, making it easy to navigate.
*   **Redundancy**: In `src/conf/mod.rs`, there's a slight redundancy in imports (`use crate::conf::CmdOptConf;` and `pub use parse::CmdOptConf;`).
*   **Formatting**: The code generally follows standard Rust formatting conventions, with `rustfmt::skip` used appropriately on generated or tabular code.

## Recommendations
1.  **Support Binary/Non-UTF-8 Input**: Transition from line-based `String` processing to byte-based processing if full compatibility with `wc` or support for arbitrary encodings is a goal.
2.  **Include Line Endings in Byte Count**: If the intention is to match `wc -c`, ensure that the bytes of the line terminators are included in the total count.
3.  **Consolidate Loops**: Refactor `run_00` to process characters and bytes in a single iteration over the input data.
4.  **Use Fixed-Size Arrays**: Replace `Vec<u64>` with `[u64; 128]` in `StatsAscii`.

---
Review Date: 2026-05-19
Reviewer: Gemini CLI Agent
