//! debug-dwarf — minimal DWARF debug info reader
use std::fmt;

#[derive(Debug, Clone)]
pub struct DebugLineEntry {
    pub address: u64,
    pub file: String,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug)]
pub enum DwarfError {
    BadAbbreviation,
    TruncatedUnit,
    UnsupportedVersion(u16),
}

impl fmt::Display for DwarfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DwarfError::BadAbbreviation => write!(f, "bad abbreviation code"),
            DwarfError::TruncatedUnit => write!(f, "truncated compilation unit"),
            DwarfError::UnsupportedVersion(v) => write!(f, "unsupported DWARF version {v}"),
        }
    }
}

impl std::error::Error for DwarfError {}

pub fn parse_debug_line(_data: &[u8]) -> Result<Vec<DebugLineEntry>, DwarfError> {
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_line_info() { assert!(parse_debug_line(&[]).is_ok()); }
}

/// FNV-1a 64 — the digest every substrate in the SuperInstance fleet agrees on.
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x100000001b3;

#[inline]
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = FNV_OFFSET;
    for &b in bytes {
        h = (h ^ b as u64).wrapping_mul(FNV_PRIME);
    }
    h
}

/// True if this crate's FNV-1a still agrees with the rest of the fleet.
pub fn canary_holds() -> bool {
    fnv1a64("café Δ 日本語".as_bytes()) == 0x024a555471370b18d
}
