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
