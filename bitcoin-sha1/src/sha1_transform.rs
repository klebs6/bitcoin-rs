crate::ix!();

/**
  | Perform a SHA‑1 transformation on one
  | 64‑byte block.
  */
#[inline]
pub fn sha1_transform(state: *mut u32, chunk: *const u8) {
    // ──────────────────────────── Constants
    const K1: u32 = 0x5A82_7999;
    const K2: u32 = 0x6ED9_EBA1;
    const K3: u32 = 0x8F1B_BCDC;
    const K4: u32 = 0xCA62_C1D6;

    unsafe {
        trace!(
            "sha1_transform: entry  \
             s0={:#010x} s1={:#010x} s2={:#010x} s3={:#010x} s4={:#010x}",
            *state.add(0),
            *state.add(1),
            *state.add(2),
            *state.add(3),
            *state.add(4)
        );

        // ─────────────── Load working variables
        let mut a = *state.add(0);
        let mut b = *state.add(1);
        let mut c = *state.add(2);
        let mut d = *state.add(3);
        let mut e = *state.add(4);

        // Message schedule as 16‑word rolling window
        let mut w = [0u32; 16];
        for i in 0..16 {
            w[i] = read_be32(chunk.add(i * 4));
        }

        // ─────────────── Main loop (80 rounds)
        for t in 0..80 {
            // Determine f and k per phase
            let (f, k) = match t {
                0..=19 => (sha1_f1(b, c, d), K1),
                20..=39 => (sha1_f2(b, c, d), K2),
                40..=59 => (sha1_f3(b, c, d), K3),
                _ => (sha1_f2(b, c, d), K4),
            };

            // Extend message schedule after the first 16 words
            let wt = if t < 16 {
                w[t]
            } else {
                let val = sha1_left(
                    w[(t + 13) & 15] ^ w[(t + 8) & 15] ^ w[(t + 2) & 15] ^ w[t & 15],
                );
                w[t & 15] = val;
                val
            };

            // Core round operation
            let temp = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(wt);

            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;

            if cfg!(debug_assertions) && t % 20 == 19 {
                debug!(
                    "round {:02}: a={:#010x} b={:#010x} c={:#010x} d={:#010x} e={:#010x}",
                    t,
                    a,
                    b,
                    c,
                    d,
                    e
                );
            }
        }

        // ─────────────── Add this chunk’s hash to result so far
        *state.add(0) = (*state.add(0)).wrapping_add(a);
        *state.add(1) = (*state.add(1)).wrapping_add(b);
        *state.add(2) = (*state.add(2)).wrapping_add(c);
        *state.add(3) = (*state.add(3)).wrapping_add(d);
        *state.add(4) = (*state.add(4)).wrapping_add(e);

        trace!(
            "sha1_transform: exit   \
             s0={:#010x} s1={:#010x} s2={:#010x} s3={:#010x} s4={:#010x}",
            *state.add(0),
            *state.add(1),
            *state.add(2),
            *state.add(3),
            *state.add(4)
        );
    }
}
