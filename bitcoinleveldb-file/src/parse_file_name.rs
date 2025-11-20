// ---------------- [ File: bitcoinleveldb-file/src/parse_file_name.rs ]
crate::ix!();

/**
  | If filename is a leveldb file, store the type
  | of the file in *type.
  |
  | The number encoded in the filename is stored in
  | *number.  If the filename was successfully
  | parsed, returns true.  Else return false.
  |
  ----------------------
  | Owned filenames have the form:
  |    dbname/CURRENT
  |    dbname/LOCK
  |    dbname/LOG
  |    dbname/LOG.old
  |    dbname/MANIFEST-[0-9]+
  |    dbname/[0-9]+.(log|sst|ldb)
  */
pub fn parse_file_name(filename: &String, number: *mut u64, ty: *mut FileType) -> bool {
    use tracing::{debug, trace};

    #[inline]
    fn consume_decimal_number(input: &str) -> Option<(u64, &str)> {
        // Parse a non-empty run of decimal digits into u64 without locale
        // and with overflow checking; return (value, remaining_suffix).
        let mut v: u128 = 0;
        let mut idx = 0usize;
        for (i, b) in input.as_bytes().iter().enumerate() {
            if !b.is_ascii_digit() {
                break;
            }
            idx = i + 1;
            let d = (b - b'0') as u128;
            v = v
                .checked_mul(10)?
                .checked_add(d)?;
            if v > u64::MAX as u128 {
                return None;
            }
        }
        if idx == 0 {
            return None;
        }
        Some((v as u64, &input[idx..]))
    }

    trace!(filename = %filename, "parse_file_name enter");

    let mut rest = filename.as_str();

    // Simple fixed-name cases
    if rest == "CURRENT" {
        unsafe {
            *number = 0;
            *ty = FileType::CurrentFile;
        }
        debug!("parsed CURRENT -> type=CurrentFile, number=0");
        return true;
    } else if rest == "LOCK" {
        unsafe {
            *number = 0;
            *ty = FileType::DBLockFile;
        }
        debug!("parsed LOCK -> type=DBLockFile, number=0");
        return true;
    } else if rest == "LOG" || rest == "LOG.old" {
        unsafe {
            *number = 0;
            *ty = FileType::InfoLogFile;
        }
        debug!(variant = %rest, "parsed info log name -> type=InfoLogFile, number=0");
        return true;
    } else if let Some(tail) = rest.strip_prefix("MANIFEST-") {
        // MANIFEST-<number> with no trailing suffix
        if let Some((num, remain)) = consume_decimal_number(tail) {
            if remain.is_empty() {
                unsafe {
                    *number = num;
                    *ty = FileType::DescriptorFile;
                }
                debug!(number = num, "parsed MANIFEST -> type=DescriptorFile");
                return true;
            }
        }
        debug!("failed parsing MANIFEST-* pattern");
        return false;
    }

    // Numeric prefix followed by a known suffix
    if let Some((num, suffix)) = consume_decimal_number(rest) {
        let parsed = if suffix == ".log" {
            unsafe {
                *ty = FileType::LogFile;
                *number = num;
            }
            true
        } else if suffix == ".sst" || suffix == ".ldb" {
            unsafe {
                *ty = FileType::TableFile;
                *number = num;
            }
            true
        } else if suffix == ".dbtmp" {
            unsafe {
                *ty = FileType::TempFile;
                *number = num;
            }
            true
        } else {
            false
        };

        if parsed {
            debug!(number = num, suffix = %suffix, "parsed numeric file name");
        } else {
            debug!(suffix = %suffix, "unsupported suffix for numeric file name");
        }
        return parsed;
    }

    debug!("parse_file_name: no pattern matched");
    false
}
