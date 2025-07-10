crate::ix!();

/// Perform **exactly one** SHA‑256 compression on a 64‑byte chunk.
///
/// This is the canonical, single‑block engine used by all higher‑level
/// wrappers.  It is deliberately `#[inline(always)]` so that the public
/// callers incur *zero* overhead in release builds.
///
/// # Safety
/// * `s` **must** point to **≥ 8** writable `u32` words.
/// * `chunk` **must** point to **exactly 64 readable bytes**.
/// * The two regions must not overlap.
#[inline(always)]
pub unsafe fn sha256_transform_block(s: *mut u32, chunk: *const u8) {
    #[inline(always)]
    unsafe fn read_be32(p: *const u8) -> u32 {
        u32::from_be_bytes([*p, *p.add(1), *p.add(2), *p.add(3)])
    }

    // --- Load working variables -------------------------------------------------
    let mut a = *s.add(0);
    let mut b = *s.add(1);
    let mut c = *s.add(2);
    let mut d = *s.add(3);
    let mut e = *s.add(4);
    let mut f = *s.add(5);
    let mut g = *s.add(6);
    let mut h = *s.add(7);

    // --- Message‑schedule words --------------------------------------------------
    let mut w0  = read_be32(chunk.add( 0));
    let mut w1  = read_be32(chunk.add( 4));
    let mut w2  = read_be32(chunk.add( 8));
    let mut w3  = read_be32(chunk.add(12));
    let mut w4  = read_be32(chunk.add(16));
    let mut w5  = read_be32(chunk.add(20));
    let mut w6  = read_be32(chunk.add(24));
    let mut w7  = read_be32(chunk.add(28));
    let mut w8  = read_be32(chunk.add(32));
    let mut w9  = read_be32(chunk.add(36));
    let mut w10 = read_be32(chunk.add(40));
    let mut w11 = read_be32(chunk.add(44));
    let mut w12 = read_be32(chunk.add(48));
    let mut w13 = read_be32(chunk.add(52));
    let mut w14 = read_be32(chunk.add(56));
    let mut w15 = read_be32(chunk.add(60));

    // --- Rounds 0 – 15 -----------------------------------------------------------
    sha256_round(a, b, c, &mut d, e, f, g, &mut h, 0x428a2f98, w0);
    sha256_round(h, a, b, &mut c, d, e, f, &mut g, 0x71374491, w1);
    sha256_round(g, h, a, &mut b, c, d, e, &mut f, 0xb5c0fbcf, w2);
    sha256_round(f, g, h, &mut a, b, c, d, &mut e, 0xe9b5dba5, w3);
    sha256_round(e, f, g, &mut h, a, b, c, &mut d, 0x3956c25b, w4);
    sha256_round(d, e, f, &mut g, h, a, b, &mut c, 0x59f111f1, w5);
    sha256_round(c, d, e, &mut f, g, h, a, &mut b, 0x923f82a4, w6);
    sha256_round(b, c, d, &mut e, f, g, h, &mut a, 0xab1c5ed5, w7);
    sha256_round(a, b, c, &mut d, e, f, g, &mut h, 0xd807aa98, w8);
    sha256_round(h, a, b, &mut c, d, e, f, &mut g, 0x12835b01, w9);
    sha256_round(g, h, a, &mut b, c, d, e, &mut f, 0x243185be, w10);
    sha256_round(f, g, h, &mut a, b, c, d, &mut e, 0x550c7dc3, w11);
    sha256_round(e, f, g, &mut h, a, b, c, &mut d, 0x72be5d74, w12);
    sha256_round(d, e, f, &mut g, h, a, b, &mut c, 0x80deb1fe, w13);
    sha256_round(c, d, e, &mut f, g, h, a, &mut b, 0x9bdc06a7, w14);
    sha256_round(b, c, d, &mut e, f, g, h, &mut a, 0xc19bf174, w15);

    // --- Rounds 16 – 63 ----------------------------------------------------------
    macro_rules! schedule_and_round {
        ($k:expr, $w_cur:ident, $w_im14:ident, $w_im9:ident, $w_im1:ident,
         $next_a:ident, $next_b:ident, $next_c:ident, $next_d:ident,
         $next_e:ident, $next_f:ident, $next_g:ident, $next_h:ident) => {
            $w_cur = $w_cur
                .wrapping_add(sha256_sigma1($w_im14))
                .wrapping_add($w_im9)
                .wrapping_add(sha256_sigma0($w_im1));
            sha256_round(
                $next_a, $next_b, $next_c, &mut $next_d,
                $next_e, $next_f, $next_g, &mut $next_h,
                $k, $w_cur,
            );
        };
    }

    schedule_and_round!(0xe49b69c1, w0,  w14, w9,  w1,  a, b, c, d, e, f, g, h);
    schedule_and_round!(0xefbe4786, w1,  w15, w10, w2,  h, a, b, c, d, e, f, g);
    schedule_and_round!(0x0fc19dc6, w2,  w0,  w11, w3,  g, h, a, b, c, d, e, f);
    schedule_and_round!(0x240ca1cc, w3,  w1,  w12, w4,  f, g, h, a, b, c, d, e);
    schedule_and_round!(0x2de92c6f, w4,  w2,  w13, w5,  e, f, g, h, a, b, c, d);
    schedule_and_round!(0x4a7484aa, w5,  w3,  w14, w6,  d, e, f, g, h, a, b, c);
    schedule_and_round!(0x5cb0a9dc, w6,  w4,  w15, w7,  c, d, e, f, g, h, a, b);
    schedule_and_round!(0x76f988da, w7,  w5,  w0,  w8,  b, c, d, e, f, g, h, a);

    schedule_and_round!(0x983e5152, w8,  w6,  w1,  w9,  a, b, c, d, e, f, g, h);
    schedule_and_round!(0xa831c66d, w9,  w7,  w2,  w10, h, a, b, c, d, e, f, g);
    schedule_and_round!(0xb00327c8, w10, w8,  w3,  w11, g, h, a, b, c, d, e, f);
    schedule_and_round!(0xbf597fc7, w11, w9,  w4,  w12, f, g, h, a, b, c, d, e);
    schedule_and_round!(0xc6e00bf3, w12, w10, w5,  w13, e, f, g, h, a, b, c, d);
    schedule_and_round!(0xd5a79147, w13, w11, w6,  w14, d, e, f, g, h, a, b, c);
    schedule_and_round!(0x06ca6351, w14, w12, w7,  w15, c, d, e, f, g, h, a, b);
    schedule_and_round!(0x14292967, w15, w13, w8,  w0,  b, c, d, e, f, g, h, a);

    schedule_and_round!(0x27b70a85, w0,  w14, w9,  w1,  a, b, c, d, e, f, g, h);
    schedule_and_round!(0x2e1b2138, w1,  w15, w10, w2,  h, a, b, c, d, e, f, g);
    schedule_and_round!(0x4d2c6dfc, w2,  w0,  w11, w3,  g, h, a, b, c, d, e, f);
    schedule_and_round!(0x53380d13, w3,  w1,  w12, w4,  f, g, h, a, b, c, d, e);
    schedule_and_round!(0x650a7354, w4,  w2,  w13, w5,  e, f, g, h, a, b, c, d);
    schedule_and_round!(0x766a0abb, w5,  w3,  w14, w6,  d, e, f, g, h, a, b, c);
    schedule_and_round!(0x81c2c92e, w6,  w4,  w15, w7,  c, d, e, f, g, h, a, b);
    schedule_and_round!(0x92722c85, w7,  w5,  w0,  w8,  b, c, d, e, f, g, h, a);

    schedule_and_round!(0xa2bfe8a1, w8,  w6,  w1,  w9,  a, b, c, d, e, f, g, h);
    schedule_and_round!(0xa81a664b, w9,  w7,  w2,  w10, h, a, b, c, d, e, f, g);
    schedule_and_round!(0xc24b8b70, w10, w8,  w3,  w11, g, h, a, b, c, d, e, f);
    schedule_and_round!(0xc76c51a3, w11, w9,  w4,  w12, f, g, h, a, b, c, d, e);
    schedule_and_round!(0xd192e819, w12, w10, w5,  w13, e, f, g, h, a, b, c, d);
    schedule_and_round!(0xd6990624, w13, w11, w6,  w14, d, e, f, g, h, a, b, c);
    schedule_and_round!(0xf40e3585, w14, w12, w7,  w15, c, d, e, f, g, h, a, b);
    schedule_and_round!(0x106aa070, w15, w13, w8,  w0,  b, c, d, e, f, g, h, a);

    // --- Feed‑forward ------------------------------------------------------------
    *s.add(0) = (*s.add(0)).wrapping_add(a);
    *s.add(1) = (*s.add(1)).wrapping_add(b);
    *s.add(2) = (*s.add(2)).wrapping_add(c);
    *s.add(3) = (*s.add(3)).wrapping_add(d);
    *s.add(4) = (*s.add(4)).wrapping_add(e);
    *s.add(5) = (*s.add(5)).wrapping_add(f);
    *s.add(6) = (*s.add(6)).wrapping_add(g);
    *s.add(7) = (*s.add(7)).wrapping_add(h);
}
