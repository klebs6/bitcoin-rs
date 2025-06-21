// ---------------- [ File: bitcoin-string/src/convert_bits.rs ]
crate::ix!();

//--------------------[ bitcoin-string/src/decode.rs ]--------------------
//--------------------[ bitcoin-string/src/encode.rs ]--------------------

/// Generic bit‑group converter used by the Base{32,64} encoders/decoders.
///
/// * `FROM` – size of input symbol in bits  
/// * `TO`   – size of output symbol in bits  
/// * `PAD`  – whether residual bits are zero‑padded (`true`)
///
/// Returns `false` when the input violates the size/overflow rules described
/// by BIP‑173 (§5) and RFC 4648.  
/// The supplied `out` callback is invoked for every produced symbol.
pub fn convert_bits<
    const FROM: usize,
    const TO: usize,
    const PAD: bool,
    I,
    O,
>(
    input: I,
    mut out: O,
) -> bool
where
    I: IntoIterator<Item = u8>,
    O: FnMut(u8),
{
    trace!("convert_bits<{FROM},{TO},{PAD}>: start");
    debug_assert!(FROM > 0 && FROM <= 8 && TO > 0 && TO <= 8);

    let max_v: usize = (1 << TO) - 1;
    let max_acc: usize = (1 << (FROM + TO - 1)) - 1;

    let mut acc: usize = 0;
    let mut bits: usize = 0;

    for value in input {
        if (value as usize) >> FROM != 0 {
            trace!("convert_bits: value overflow = {value}");
            return false; // symbol too large for FROM‑bit representation
        }
        acc = ((acc << FROM) | value as usize) & max_acc;
        bits += FROM;
        while bits >= TO {
            bits -= TO;
            let out_sym = ((acc >> bits) & max_v) as u8;
            trace!("convert_bits: emit {out_sym}");
            out(out_sym);
        }
    }

    if PAD {
        if bits > 0 {
            let out_sym = ((acc << (TO - bits)) & max_v) as u8;
            trace!("convert_bits: pad emit {out_sym}");
            out(out_sym);
        }
        true
    } else if bits < FROM && ((acc << (TO - bits)) & max_v) == 0 {
        true
    } else {
        trace!("convert_bits: non‑zero leftover bits = {bits}");
        false
    }
}

#[cfg(test)]
mod tests_bit_group_conversion {
    use super::*;

    #[traced_test]
    fn roundtrip_8_to_5_to_8_all_bytes() {
        let original: Vec<u8> = (0u8..=255u8).collect();
        let mut five_bit = Vec::<u8>::new();

        assert!(convert_bits::<8, 5, true, _, _>(original.iter().copied(), |b| {
            five_bit.push(b)
        }));

        let mut roundtrip = Vec::<u8>::new();
        assert!(convert_bits::<5, 8, true, _, _>(five_bit.into_iter(), |b| {
            roundtrip.push(b)
        }));

        let (prefix, extra) = roundtrip.split_at(original.len());
        assert_eq!(prefix, original.as_slice());
        assert!(
            extra.is_empty() || extra == [0],
            "at most one trailing zero may be added by padding"
        );
        info!("8→5→8 round‑trip verified");
    }

    #[traced_test]
    fn reject_leftover_bits_when_no_padding() {
        let mut out = Vec::<u8>::new();
        let ok = convert_bits::<8, 5, false, _, _>(
            [0xFFu8].into_iter(),
            |b| out.push(b),
        );
        trace!("ok={ok}, out={out:?}");
        assert!(!ok);
        assert_eq!(out, [0x1F], "completed symbol must still be emitted");
    }
}
