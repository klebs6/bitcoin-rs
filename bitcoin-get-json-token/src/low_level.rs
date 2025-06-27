// ---------------- [ File: bitcoin-get-json-token/src/low_level.rs ]
crate::ix!();

/* ------------------------------------------------------------------------- */
/* helper: one‑byte structural tokens                                        */
/* ------------------------------------------------------------------------- */
#[inline]
#[instrument(level = "trace", skip(p, consumed))]
pub unsafe fn single_byte_token(
    p:        &mut *const u8,
    start:    *const u8,
    consumed: &mut u32,
    kind:     JTokenType,
) -> JTokenType {
    *p = (*p).add(1);                                       // advance cursor
    *consumed = (*p as usize - start as usize) as u32;
    trace!(?kind, consumed, "single_byte_token emitted");
    kind
}

/* ===================================================================== */
/*  ────────────────  low‑level helpers – no semantic state  ───────────  */
/* ===================================================================== */

/// Skip JSON whitespace (`SP`, `HT`, `LF`, `CR`) **and** trailing NUL
/// padding.  Returns the first non‑whitespace, non‑NUL position.
#[inline]
#[instrument(level = "trace", skip_all)]
pub unsafe fn skip_ws_nul(mut p: *const u8, end: *const u8) -> *const u8 {
    while p < end && (json_isspace(*p as i32) || *p == 0) {
        p = p.add(1);
    }
    p
}

/// Helper used by *all* sub‑lexers to compute the byte count that was
/// consumed for the current token.
#[inline]
pub unsafe fn bytes_consumed(start: *const u8, after: *const u8) -> u32 {
    (after as usize - start as usize) as u32
}

#[cfg(test)]
mod single_byte_spec {
    use super::*;

    #[traced_test]
    fn emits_expected_meta() {
        unsafe {
            let buf   = b"{";
            let mut p = buf.as_ptr();
            let mut n = 0u32;
            let tok   = single_byte_token(&mut p, buf.as_ptr(), &mut n, JTokenType::JTOK_OBJ_OPEN);
            assert_eq!(tok, JTokenType::JTOK_OBJ_OPEN);
            assert_eq!(n, 1);
            assert_eq!(p, buf.as_ptr().add(1));
        }
    }
}
