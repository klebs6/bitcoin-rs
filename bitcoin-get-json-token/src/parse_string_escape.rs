// ---------------- [ File: bitcoin-get-json-token/src/parse_string_escape.rs ]
crate::ix!();

#[inline]
pub unsafe fn parse_string_escape(
    backslash: *const u8,
    end: *const u8,
) -> Option<(String, *const u8)> {
    //  backslash points at the '\', so we need the char _after_ it
    let mut cur = backslash.add(1);
    if cur >= end {
        return None;
    }
    let mut out = String::new();
    match *cur as char {
        '"'  => out.push('"'),
        '\\' => out.push('\\'),
        '/'  => out.push('/'),
        'b'  => out.push('\u{0008}'),
        'f'  => out.push('\u{000C}'),
        'n'  => out.push('\n'),
        'r'  => out.push('\r'),
        't'  => out.push('\t'),
        'u'  => {
            /* ensure at least uXXXX available */
            if cur.add(5) > end {
                return None;
            }
            let mut cp = 0u32;
            let next = hatoui(cur.add(1), cur.add(5), &mut cp);
            if next != cur.add(5) {
                return None;
            }
            /* -------- surrogate pair handling (unchanged) -------- */
            match cp {
                0xD800..=0xDBFF => {
                    if cur.add(11) > end || *cur.add(5) != b'\\' || *cur.add(6) != b'u' {
                        return None;
                    }
                    let mut low = 0u32;
                    let next2 = hatoui(cur.add(7), cur.add(11), &mut low);
                    if next2 != cur.add(11) || !(0xDC00..=0xDFFF).contains(&low) {
                        return None;
                    }
                    let full = 0x10000 + ((cp - 0xD800) << 10) + (low - 0xDC00);
                    out.push(char::from_u32(full)?);
                    cur = next2;
                }
                0xDC00..=0xDFFF => return None,
                _ => {
                    out.push(char::from_u32(cp)?);
                    cur = next;
                }
            }
            cur = cur.sub(1);          // compensate for main loop’s +1
        }
        _ => return None,
    }
    cur = cur.add(1);
    Some((out, cur))
}
