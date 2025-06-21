// ---------------- [ File: bitcoin-poly1305/src/poly1305_auth.rs ]
crate::ix!();

/// -----------------------------------------------------------------------------
/// [poly1305] public API (refactored driver)
/// -----------------------------------------------------------------------------
///
/// Compute a Poly1305 one‑time authenticator.
///
/// * `out` – where the 16‑byte tag is written  
/// * `msg` – message to authenticate  
/// * `key` – 32‑byte (r ‖ s) key; **must be unique per‑message**
///
/// The implementation is a direct, constant‑time translation of the original
/// Bitcoin Core `poly1305-donna-unrolled.c` routine, with carries, clamps and
/// reductions preserved exactly.  Logging is limited to high‑level progress so
/// that the function remains branch‑predictable.
pub fn poly1305_auth(
    out: &mut [u8; POLY1305_TAGLEN],
    msg: &[u8],
    key: &[u8; POLY1305_KEYLEN],
) {
    use core::convert::TryInto;

    tracing::info!(msg_len = msg.len(), "poly1305_auth: start");

    let (r, s) = expand_key(key);
    let mut h: LimbArr5 = [0; 5];

    // full 16‑byte blocks
    let mut m = msg;
    let mut blk_idx = 0usize;
    while m.len() >= 16 {
        let block: &[u8; 16] = m[..16].try_into().unwrap();
        tracing::trace!(blk_idx, block = ?*block, "poly1305_auth: process full block");
        accumulate_block(&mut h, block, true);
        multiply_and_reduce(&mut h, &r, &s);
        blk_idx += 1;
        m = &m[16..];
    }

    // tail (≤ 15 B)
    if !m.is_empty() {
        let mut tail = [0u8; 16];
        tail[..m.len()].copy_from_slice(m);
        tail[m.len()] = 1;
        tracing::trace!(tail_len = m.len(), tail = ?tail, "poly1305_auth: process tail");
        accumulate_block(&mut h, &tail, false);
        multiply_and_reduce(&mut h, &r, &s);
    }

    final_carry_and_sub_p(&mut h);
    tracing::debug!(h_final = ?h, "poly1305_auth: after final carry reduction");

    add_pad_serialize(out, &h, key);
    tracing::info!(tag = ?*out, "poly1305_auth: finished");
}
