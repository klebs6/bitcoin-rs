crate::ix!();

/**
  | Convert a byte to sliced form, storing
  | it corresponding to given row and column
  | in s
  |
  */
pub fn load_byte(
    s:        &mut AESState,
    mut byte: u8,
    r:        i32,
    c:        i32) 
{
    let mut i: i32 = 0;

    for i in 0..8 {

        s.slice[i] |= u16::from((byte & 1) << (r * 4 + c));

        byte >>= 1;
    }
}

/// Load 16 bytes into their bit‑sliced representation (8× u16 words ≈ AES state).
#[inline(always)]
pub fn load_bytes(state: *mut AESState, mut data16: *const u8) {
    tracing::trace!(
        target: "aes",
        "load_bytes – entry; state_ptr = {:p}, data_ptr = {:p}",
        state,
        data16
    );

    // Safety: caller guarantees both pointers are valid for the required size.
    unsafe {
        for c in 0..4 {
            for r in 0..4 {
                let byte = *data16;
                data16 = data16.add(1);
                load_byte(&mut *state, byte, r as i32, c as i32);
            }
        }
    }

    tracing::trace!(target: "aes", "load_bytes – exit");
}

/// Convert a bit‑sliced `AESState` back into its canonical 16‑byte form.
#[inline(always)]
pub fn save_bytes(mut data16: *mut u8, state: *const AESState) {
    tracing::trace!(
        target: "aes",
        "save_bytes – entry; state_ptr = {:p}, data_ptr = {:p}",
        state,
        data16
    );

    // Safety: caller guarantees both pointers are valid for the required size.
    unsafe {
        for c in 0..4 {
            for r in 0..4 {
                let mut v: u8 = 0;
                for b in 0..8 {
                    let bit = (((*state).slice[b] >> (r * 4 + c)) & 1) as u8;
                    v |= bit << b;
                }
                *data16 = v;
                data16 = data16.add(1);
            }
        }
    }

    tracing::trace!(target: "aes", "save_bytes – exit");
}
