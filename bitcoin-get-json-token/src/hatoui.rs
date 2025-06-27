// ---------------- [ File: bitcoin-get-json-token/src/hatoui.rs ]
crate::ix!();

/**
  | Convert a hexadecimal string (given by
  | *first* … *last*) into an unsigned `u32`.
  | The function writes the parsed value into
  | *out* and returns a pointer to the first
  | byte that was **not** consumed.
  */
#[instrument(level = "trace", skip_all)]
pub fn hatoui(first: *const u8, last: *const u8, out: &mut u32) -> *const u8 {
    let mut result: u32 = 0;
    let mut ptr = first;

    unsafe {
        while ptr != last {
            let ch = *ptr as char;
            let digit_opt = match ch {
                '0'..='9' => Some(ch as u32 - '0' as u32),
                'a'..='f' => Some(ch as u32 - 'a' as u32 + 10),
                'A'..='F' => Some(ch as u32 - 'A' as u32 + 10),
                _ => None,
            };

            match digit_opt {
                Some(d) => {
                    result = result.wrapping_mul(16).wrapping_add(d);
                    ptr = ptr.add(1);
                }
                None => break,
            }
        }

        *out = result;
        ptr
    }
}

#[cfg(test)]
mod hatoui_spec {
    use super::*;

    #[traced_test]
    fn parses_hex_until_non_hex() {
        let bytes = b"1aFZ";           // stops at 'Z'
        let mut out = 0u32;

        let start = bytes.as_ptr();
        let end   = unsafe { start.add(bytes.len()) };

        let ret = unsafe { hatoui(start, end, &mut out) };

        assert_eq!(out, 0x1AF);
        let consumed = (ret as usize) - (start as usize);
        assert_eq!(consumed, 3);       // "1aF"
    }
}
