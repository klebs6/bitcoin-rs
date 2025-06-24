// ---------------- [ File: bitcoin-golombrice/src/golombrice.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/golombrice.h]

pub fn golomb_rice_encode<OStream>(
    bitwriter: &mut BitStreamWriter<OStream>,
    p:         u8,
    x:         u64,
) where
    OStream: Default + Write,
{
    // Write quotient as unary-encoded: q 1's
    // followed by one 0.
    let mut q = x >> p;

    // unary‑encode the quotient: q × ‘1’, then a single ‘0’
    while q > 0 {
        let nbits = if q <= 64 { q as i32 } else { 64 };
        bitwriter.write(!0, nbits);
        q -= nbits as u64;
    }
    bitwriter.write(0, 1);

    // Write the remainder in P bits. Since the
    // remainder is just the bottom P bits of x,
    // there is no need to mask first.
    bitwriter.write(x, p.into());
}

pub fn golomb_rice_decode<IStream>(
    bitreader: &mut BitStreamReader<IStream>,
    p:         u8,
) -> u64
where
    IStream: Default + Read,
{
    // unary‑decode the quotient
    let mut q = 0u64;
    while bitreader.read(1) == 1 {
        q += 1;
    }

    // read the P‑bit remainder
    let r = bitreader.read(p.into());
    (q << p) + r
}

#[cfg(test)]
mod golombrice_encode_decode_tests {
    use super::*;
    use std::{
        cell::RefCell,
        io::{Read, Write},
        rc::Rc,
    };

    /// Simple in‑memory sink/source that satisfies
    /// the I/O bounds of BitStreamWriter/Reader.
    #[derive(Default)]
    struct VecSink {
        data: Vec<u8>,
        pos:  usize,
    }
    impl Write for VecSink {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
    }
    impl Read for VecSink {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let remain = self.data.len().saturating_sub(self.pos);
            let n = remain.min(buf.len());
            buf[..n].copy_from_slice(&self.data[self.pos..self.pos + n]);
            self.pos += n;
            Ok(n)
        }
    }

    /// Helper: encode → flush → decode.
    fn roundtrip(p: u8, x: u64) -> u64 {
        let sink = Rc::new(RefCell::new(VecSink::default()));
        {
            let mut bw = BitStreamWriter::new(sink.clone());
            golomb_rice_encode(&mut bw, p, x);
            bw.flush();
        }
        let mut br = BitStreamReader::new(sink);
        golomb_rice_decode(&mut br, p)
    }

    /// Exhaustive check for tiny domain: fast.
    #[traced_test]
    fn exhaustive_small_values() {
        for p in 0..=6 {
            for x in 0..=127 {
                assert_eq!(roundtrip(p, x), x,
                    "round‑trip failed for P={p}, x={x}");
            }
        }
    }

    /// Selected edge scenarios that **complete quickly**.
    #[traced_test]
    fn zero_and_reasonable_large_values() {
        // x = 0 must survive any P.
        assert_eq!(roundtrip(0, 0), 0);

        // A “large” value where the quotient is big but still
        // practical (≈ 1 million unary bits when P = 3).
        let p  = 3;
        let x  = (1u64 << p) * 131_072; // q = 131 072
        assert_eq!(roundtrip(p, x), x);

        // Another large-ish value with higher P so the quotient is tiny.
        let p_big = 20;
        let x_big = (1u64 << p_big) * 42; // q = 42
        assert_eq!(roundtrip(p_big, x_big), x_big);
    }
}
