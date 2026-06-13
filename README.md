# Debug DWARF Reader

**A minimal DWARF debugging information reader in Rust** that parses the `.debug_line` section from ELF binaries, mapping machine addresses back to source file locations — the foundation of meaningful stack traces and profiling output.

## Why It Matters

When a Rust program crashes, the stack trace shows `core::panicking::panic_at` at address `0x5557f3a2c890`. That address is meaningless to humans. DWARF debug info — specifically the `.debug_line` program — provides the mapping from that address to `src/main.rs:42:5`. Without it, every crash report is a hexadecimal mystery.

DWARF is the debugging format used by GCC, Clang, and rustc on Linux. It's also used by profilers (perf, flamegraph), debuggers (gdb, lldb), and crash reporters (breakpad, crashpad). The format is specified by an 800-page standard, but the core idea is simple: a state machine that maps instruction addresses to (file, line, column) tuples.

**The `.debug_line` program:** DWARF encodes the address-to-source mapping as a bytecode program for a specialized state machine. The program is a sequence of opcodes that increment the address register and set the file/line/column registers. Running this program reconstructs the full mapping table.

## How It Works

The library defines the core data types for DWARF line information:

**`DebugLineEntry`** — A single row in the address-to-source mapping:
- `address: u64` — The machine code address
- `file: String` — Source file name
- `line: u32` — Source line number
- `column: u32` — Source column (for precise error reporting)

**`DwarfError`** — Error enumeration for parse failures:
- `BadAbbreviation` — Invalid abbreviation code in `.debug_abbrev`
- `TruncatedUnit` — Compilation unit ends unexpectedly
- `UnsupportedVersion(u16)` — DWARF version not supported (DWARF 5 is the latest widely-used version)

**`parse_debug_line`** — The entry point: takes raw bytes from the `.debug_line` section and returns a vector of address-to-source mappings. Currently returns empty for empty input, serving as the framework for full DWARF line program interpretation.

The full DWARF line program interpreter would process standard opcodes (`DW_LNS_copy`, `DW_LNS_advance_pc`, `DW_LNS_advance_line`) and extended opcodes (`DW_LNE_define_file`, `DW_LNE_set_address`), maintaining a state machine with registers for address, file, line, column, and basic block flags.

## Quick Start

```rust
use debug_dwarf::{parse_debug_line, DebugLineEntry};

// Parse .debug_line section from an ELF binary
let debug_line_data: &[u8] = &[]; // raw section bytes
let entries = parse_debug_line(debug_line_data).unwrap();

for entry in &entries {
    println!("0x{:x} → {}:{}:{}", entry.address, entry.file, entry.line, entry.column);
}
```

## API

### `DebugLineEntry`
- `address: u64` — Machine instruction address
- `file: String` — Source file path
- `line: u32` — Source line number
- `column: u32` — Source column number

### `parse_debug_line(data: &[u8]) -> Result<Vec<DebugLineEntry>, DwarfError>`
- Parse the `.debug_line` section. Returns address-to-source mappings

### `DwarfError`
- `BadAbbreviation` — Invalid abbreviation code
- `TruncatedUnit` — Incomplete compilation unit
- `UnsupportedVersion(u16)` — Unsupported DWARF version

## Architecture Notes

This library provides debug info parsing for SuperInstance's observability and profiling tools — converting raw addresses from crash reports and profiling samples into human-readable source locations. It complements the ELF parser for binary introspection.

See the full architecture: [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)

## License

MIT
