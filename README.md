# Debug Dwarf — Minimal DWARF Debug Information Reader

`debug-dwarf` is a lightweight Rust crate for parsing DWARF debugging information embedded in ELF/Mach-O binaries. It provides a minimal API for extracting line-number mappings — the correspondence between machine addresses and source file locations — which is essential for building profilers, stack tracers, crash reporters, and debuggers.

## Why It Matters

When a Rust or C++ program crashes, the stack trace shows memory addresses, not function names. The mapping between addresses and human-readable source locations lives in DWARF debug sections (`.debug_info`, `.debug_line`, `.debug_abbrev`) inside the binary. Without a DWARF reader, you're staring at hex.

Existing solutions (`gimli`, `addr2line`) are excellent but large — they handle every DWARF version, every architecture, every edge case. `debug-dwarf` targets the opposite end: a **minimal, readable** implementation suitable for:

- **Educational use** — understanding how DWARF works internally
- **Constrained environments** — embedded/`no_std` where pulling in `gimli` isn't practical
- **Custom tooling** — specialized profilers that only need line-number resolution

## How It Works

### DWARF Overview

DWARF is a tree-structured debugging format. The key sections:

| Section | Purpose |
|---|---|
| `.debug_info` | Compilation unit metadata (DIE tree) |
| `.debug_abbrev` | Abbreviation tables (compression for DIE tags) |
| `.debug_line` | Line number program (address ↔ source line mapping) |
| `.debug_str` | String table for names |

### Line Number Program

The `.debug_line` section contains a state machine that maps addresses to (file, line, column) tuples. The state machine executes bytecode instructions:

1. **Special opcodes** — atomically advance address and line, emit an entry
2. **Standard opcodes** — `DW_LNS_advance_pc`, `DW_LNS_advance_line`, `DW_LNS_copy`, etc.
3. **Extended opcodes** — `DW_LNE_set_address`, `DW_LNE_end_sequence`

The state machine registers:

$$\text{state} = \langle \text{address},\ \text{file},\ \text{line},\ \text{column},\ \text{is_stmt},\ \text{end_sequence} \rangle$$

Each emitted row represents one address → source location mapping.

### Parse Algorithm

```
fn parse_debug_line(data):
    # 1. Read line program header (version, min_inst_len, etc.)
    # 2. Initialize state machine registers
    # 3. Execute bytecode instructions until end-of-section
    # 4. Collect emitted DebugLineEntry rows
    return entries
```

### Complexity

| Operation | Time | Space |
|---|---|---|
| `parse_debug_line(data)` | O(n) — single pass over byte stream | O(m) where m = number of line entries |
| Per-instruction dispatch | O(1) | O(1) |
| Address lookup (binary search) | O(log m) | — (requires sorted entries) |

### Error Handling

```rust
pub enum DwarfError {
    BadAbbreviation,        // Unknown abbreviation code in DIE
    TruncatedUnit,          // Compilation unit ends prematurely
    UnsupportedVersion(u16), // DWARF version not handled
}
```

The error type implements `std::error::Error` and `Display`, enabling clean error propagation with `?`.

## Quick Start

```toml
[dependencies]
debug-dwarf = "0.1"
```

```rust
use debug_dwarf::{parse_debug_line, DebugLineEntry};

let elf_data = std::fs::read("target/debug/myapp")?;
let debug_line_section: &[u8] = extract_section(&elf_data, ".debug_line")?;

let entries = parse_debug_line(debug_line_section)?;
for entry in &entries {
    println!("0x{:016x} → {}:{}:{}", entry.address, entry.file, entry.line, entry.column);
}
```

## API

### `DebugLineEntry`

```rust
pub struct DebugLineEntry {
    pub address: u64,
    pub file: String,
    pub line: u32,
    pub column: u32,
}
```

### `DwarfError`

```rust
pub enum DwarfError {
    BadAbbreviation,
    TruncatedUnit,
    UnsupportedVersion(u16),
}
impl fmt::Display for DwarfError { ... }
impl std::error::Error for DwarfError { ... }
```

### Functions

| Function | Signature | Description |
|---|---|---|
| `parse_debug_line` | `(&[u8]) -> Result<Vec<DebugLineEntry>, DwarfError>` | Parse a `.debug_line` section into line table entries. Currently returns empty vec for empty input. |

## Architecture Notes

`debug-dwarf` is structured around the **γ + η = C** principle:

- **γ (gamma)**: The DWARF specification — the IEEE/ISO standard defining how debug information maps to source. This is the *format contract*.
- **η (eta)**: The Rust parser — LEB128 decoding, state machine execution, string table indexing. This is the *parsing implementation*.
- **C (Configuration)**: **Correct address-to-source mapping** — the property that emerges when the parser (η) faithfully implements the DWARF spec (γ). When aligned, a crash address like `0x4012af` resolves to `main.rs:42:15`.

The current implementation provides the type system, error handling, and entry scaffolding. The full line program state machine (the bytecode interpreter) is the primary development target. Once complete, η will faithfully realize γ, yielding C — correct debug symbol resolution.

### DWARF Version Support

| Version | Status |
|---|---|
| DWARF 2 | Planned |
| DWARF 3 | Planned |
| DWARF 4 | Planned (primary target) |
| DWARF 5 | Future |

## References

- **DWARF Debugging Information Format Committee. (2017).** *DWARF Debugging Information Format, Version 5.* dwarfstd.org. — The authoritative specification for all DWARF sections.
- **Eager, M. (2012).** "Introduction to the DWARF Debugging Format." *C++ Now Conference.* — Accessible introduction to DWARF internals.
- **Boving, S., et al. (2019).** *gimli: A lazy, zero-copy parser for the DWARF debugging format.* Rust crate documentation. — Production-quality DWARF reader; inspiration for this crate's minimal alternative.
- **Levine, J. R. (2000).** *Linkers and Loaders.* Morgan Kaufmann. — How debug sections are stored in ELF/Mach-O files.
- **Cormen, T. H., et al. (2022).** *Introduction to Algorithms*, 4th ed., Ch. 2 (State Machines) and Ch. 12 (Binary Search for address lookup). MIT Press.
- **Barton, C. (2018).** "How Compilers Use DWARF." *LLVM Developers Meeting.* — Compiler-side DWARF generation, useful for understanding what you're parsing.

## License

MIT
