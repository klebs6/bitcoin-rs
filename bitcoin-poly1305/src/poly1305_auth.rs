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
    use tracing::{info, trace};

    info!(len = msg.len(), "poly1305_auth: start");

    // 1. key expansion
    let (r, s) = expand_key(key);

    // 2. state ‑‑ h0…h4
    let mut h: LimbArr5 = [0; 5];

    // 3. full 16‑byte blocks
    let mut m = msg;
    while m.len() >= 16 {
        accumulate_block(&mut h, array_ref::array_ref![m, 0, 16], true);
        multiply_and_reduce(&mut h, &r, &s);
        trace!(block = ?&m[..16], "processed full block");
        m = &m[16..];
    }

    // 4. tail (≤15 bytes)
    if !m.is_empty() {
        let mut tail = [0u8; 16];
        tail[..m.len()].copy_from_slice(m);
        tail[m.len()] = 1;
        accumulate_block(&mut h, &tail, false);
        multiply_and_reduce(&mut h, &r, &s);
        trace!(tail_len = m.len(), "processed tail block");
    }

    // 5. final carry & conditional subtraction
    final_carry_and_sub_p(&mut h);

    // 6. add pad & write tag
    add_pad_serialize(out, &h, key);
    info!("poly1305_auth: finished");
}
