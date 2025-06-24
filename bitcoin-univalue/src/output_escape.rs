// ---------------- [ File: bitcoin-univalue/src/output_escape.rs ]
crate::ix!();

/// Compute the canonical escape sequence for a single byte according to
/// Bitcoin‑Core / RFC 7159 rules.  Returns `None` when the byte may be
/// embedded verbatim in a JSON‐string.
fn escape_for_byte(b: u8) -> Option<String> {
    match b {
        // core control‑character short‑cuts
        b'"'  => Some(r#"\""#.into()),
        b'\\' => Some(r#"\\"#.into()),
        0x08  => Some(r#"\b"#.into()),
        0x0C  => Some(r#"\f"#.into()),
        0x0A  => Some(r#"\n"#.into()),
        0x0D  => Some(r#"\r"#.into()),
        0x09  => Some(r#"\t"#.into()),
        0x00..=0x1F | 0x7F => Some(format!(r#"\u{:04x}"#, b)),
        _ => None,
    }
}

/// Stand‑alone generator that constructs **and returns** the complete
/// Rust source for the escape lookup‑table.  Keeping the generator
/// pure (it only produces a `String`) lets the unit‑tests inspect the
/// result directly without resorting to brittle child‑process hacks.
#[instrument(level = "trace", skip_all)]
pub fn generate_escapes_table() -> String {
    /// Emit a Rust string literal with minimal escaping.
    fn rust_quote(s: &str) -> String {
        s.replace('\\', r"\\").replace('"', r#"\""#)
    }

    use std::fmt::Write;

    let mut buf = String::new();

    writeln!(
        buf,
        "// Automatically generated file. Do not modify.\n\
         crate::ix!();\n\n\
         /// Lookup‑table used by `json_escape()`.\n\
         pub const escapes: [Option<&'static str>; 256] = ["
    )
    .unwrap();

    for (idx, byte) in (0u8..=255).enumerate() {
        match escape_for_byte(byte) {
            Some(s) => writeln!(buf, "    Some(\"{}\"),", rust_quote(&s)).unwrap(),
            None    => writeln!(buf, "    None,").unwrap(),
        }

        // readability: blank line after every eight entries
        if (idx + 1) % 8 == 0 {
            writeln!(buf).unwrap();
        }
    }

    writeln!(buf, "];").unwrap();
    buf
}

/// Emit the escape lookup‑table to **stdout**
/// (the legacy contract expected by `univalue_gen_main`),
/// now implemented by delegating to `generate_escapes_table()`.
#[instrument(level = "trace", skip_all)]
pub fn output_escape() {
    use std::io::{self, Write};

    let tbl = generate_escapes_table();
    io::stdout()
        .write_all(tbl.as_bytes())
        .expect("write escapes table");
}

#[cfg(test)]
mod output_escape_spec {
    use super::*;

    /// The generator must yield a well‑formed Rust source
    /// file containing **exactly** 256 table entries.
    #[traced_test]
    fn emits_valid_rust_table() {
        let src = generate_escapes_table();

        // ────────── structural sentinels ──────────
        assert!(
            src.starts_with("// Automatically generated file."),
            "missing prologue"
        );
        assert!(
            src.contains("pub const escapes: [Option<&'static str>; 256] = ["),
            "missing table declaration"
        );
        assert!(
            src.trim_end().ends_with("];"),
            "missing epilogue"
        );

        // ────────── row‑count sanity check ──────────
        let rows = src
            .lines()
            .filter(|l| {
                let t = l.trim_start();
                t.starts_with("Some(") || t.starts_with("None")
            })
            .count();
        assert_eq!(rows, 256, "table must have 256 entries");
    }
}
