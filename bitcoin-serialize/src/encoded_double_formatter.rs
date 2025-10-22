// ---------------- [ File: bitcoin-serialize/src/encoded_double_formatter.rs ]
crate::ix!();

#[inline]
pub fn encode_double(v: f64) -> u64 { v.to_bits() }

#[inline]
pub fn decode_double(x: u64) -> f64 { f64::from_bits(x) }

/// Bit-exact IEEE-754 `f64` serializer used by Core:
/// writes the raw `u64` payload **little-endian**.
#[derive(Default, Copy, Clone)]
pub struct EncodedDoubleFormatter;

impl EncodedDoubleFormatter {
    #[inline]
    pub fn ser_stream<S: Write>(&mut self, s: &mut S, v: f64) {
        let enc = encode_double(v).to_le_bytes();
        s.write_all(&enc).expect("write f64-encoded u64");
    }

    #[inline]
    pub fn unser_stream<S: Read>(&mut self, s: &mut S, v: &mut f64) {
        let mut buf = [0u8; core::mem::size_of::<u64>()];
        s.read_exact(&mut buf).expect("read f64-encoded u64");
        let encoded = u64::from_le_bytes(buf);
        *v = decode_double(encoded);
    }
}

impl ValueFormatter<f64> for EncodedDoubleFormatter {
    #[inline]
    fn ser<S: Write>(&mut self, s: &mut S, v: &f64) {
        self.ser_stream(s, *v);
    }

    #[inline]
    fn unser<S: Read>(&mut self, s: &mut S, v: &mut f64) {
        self.unser_stream(s, v);
    }
}


// ---------------- [ File: bitcoin-serialize/src/encoded_double_formatter.rs ] (continued)
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn rt_single(x: f64) {
        let mut fmt = EncodedDoubleFormatter::default();
        let mut buf = Cursor::new(Vec::<u8>::new());
        // write
        fmt.ser(&mut buf, &x);
        // check LE encoding matches raw to_le_bytes
        let want = x.to_bits().to_le_bytes().to_vec();
        assert_eq!(buf.get_ref(), &want, "LE encoding mismatch");

        // read back
        buf.set_position(0);
        let mut y = 0.0;
        fmt.unser(&mut buf, &mut y);

        // bitwise equality (includes -0.0 vs +0.0, NaN payloads)
        assert_eq!(x.to_bits(), y.to_bits(), "roundtrip bit mismatch");
    }

    #[test]
    fn known_scalars() {
        rt_single(0.0);
        rt_single(-0.0);
        rt_single(1.0);
        rt_single(-1.0);
        rt_single(f64::MIN);
        rt_single(f64::MAX);
        rt_single(f64::EPSILON);
        rt_single(std::f64::consts::PI);
        rt_single(std::f64::consts::E);
        rt_single(f64::INFINITY);
        rt_single(f64::NEG_INFINITY);

        // Subnormals: smallest positive subnormal and a mid payload
        rt_single(f64::from_bits(0x0000_0000_0000_0001));
        rt_single(f64::from_bits(0x0000_0000_000F_FFFF));

        // Several NaN payloads (signaling/quiet range)
        rt_single(f64::from_bits(0x7ff0_0000_0000_0001)); // sNaN-ish
        rt_single(f64::from_bits(0x7ff8_0000_0000_0000)); // qNaN canonical
        rt_single(f64::from_bits(0x7fff_ffff_ffff_ffff)); // max payload
        rt_single(f64::from_bits(0xfff8_0000_0000_0000)); // negative NaN
    }

    #[test]
    fn broad_sweep_lcg() {
        // Deterministic LCG across the 64-bit space; ~40k samples is plenty.
        let mut state: u64 = 0x0123_4567_89ab_cdef;
        let mul:   u64 = 6364136223846793005;
        let inc:   u64 = 1;

        for _ in 0..40_000 {
            state = state.wrapping_mul(mul).wrapping_add(inc);
            // Skip encodings that produce signaling NaNs vs qNaNs? Not necessary:
            // we test bitwise roundtrip, not math ops.
            let x = f64::from_bits(state);
            rt_single(x);
        }
    }

    #[test]
    fn dense_low16_exhaustive_in_various_regions() {
        // Exhaustively test all low 16-bit tails in several exponent/sign regions.
        let prefixes = [
            0x0000_0000_0000_0000u64, // +zero neighborhood
            0x8000_0000_0000_0000u64, // -zero neighborhood
            0x0010_0000_0000_0000u64, // small normal region
            0x7ff0_0000_0000_0000u64, // +Inf/NaN exponent region
            0xfff0_0000_0000_0000u64, // -Inf/NaN exponent region
            0x3ff0_0000_0000_0000u64, // around 1.0
            0x4000_0000_0000_0000u64, // around 2.0
            0xbff0_0000_0000_0000u64, // around -1.0
        ];

        for &p in &prefixes {
            for tail in 0u64..=0xffff {
                let bits = p | tail;
                let x = f64::from_bits(bits);
                rt_single(x);
            }
        }
    }

    #[test]
    fn vectorformatter_integration_vec() {
        let mut buf = Cursor::new(Vec::<u8>::new());
        let mut vf = VectorFormatter::<EncodedDoubleFormatter>::default();

        // Build vector with tricky cases including special NaNs
        let v = vec![
            0.0, -0.0, 1.0, -1.0,
            f64::from_bits(0x7ff0_0000_0000_0001),
            f64::from_bits(0x7ff8_0000_0000_0000),
            f64::from_bits(0x0000_0000_0000_0001),
            f64::INFINITY, f64::NEG_INFINITY,
        ];

        vf.ser(&mut buf, &v);

        buf.set_position(0);
        let mut out = Vec::<f64>::new();
        vf.unser(&mut buf, &mut out);

        assert_eq!(v.len(), out.len());
        for (a, b) in v.iter().zip(out.iter()) {
            assert_eq!(a.to_bits(), b.to_bits());
        }
    }

    #[test]
    fn vectorformatter_integration_matrix() {
        let mut buf = Cursor::new(Vec::<u8>::new());
        let mut mf = VectorFormatter::<VectorFormatter<EncodedDoubleFormatter>>::default();

        let m: Vec<Vec<f64>> = vec![
            vec![0.0, 1.0, f64::from_bits(0x7ff8_0000_0000_0000)],
            vec![f64::from_bits(0x0000_0000_0000_0001), -0.0],
        ];

        mf.ser(&mut buf, &m);

        buf.set_position(0);
        let mut out = Vec::<Vec<f64>>::new();
        mf.unser(&mut buf, &mut out);

        assert_eq!(m.len(), out.len());
        for (row_a, row_b) in m.iter().zip(out.iter()) {
            assert_eq!(row_a.len(), row_b.len());
            for (a, b) in row_a.iter().zip(row_b.iter()) {
                assert_eq!(a.to_bits(), b.to_bits());
            }
        }
    }
}
