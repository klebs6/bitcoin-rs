// ---------------- [ File: bitcoin-ripemd/src/finalize.rs ]
crate::ix!();

impl Ripemd160 {
    /**
      | Complete the message‑digest computation and
      | place the 20‑byte result into `hash`.
      */
    pub fn finalize(&mut self, hash: &mut [u8; RIPEMD160_OUTPUT_SIZE]) {

        /* 1 — append mandatory padding: 0x80 then zeroes */
        const PAD: [u8; 64] = [
            0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let mut sizedesc = [0u8; 8];
        let bitlen = self.bytes() << 3;
        unsafe { writele64(sizedesc.as_mut_ptr(), bitlen) };

        /* pad so that (message_len + padding) ≡ 56 (mod 64) */
        let padlen = 1 + ((119 - (self.bytes() % 64)) % 64) as usize;
        unsafe {
            self.write(PAD.as_ptr(), padlen);
            self.write(sizedesc.as_ptr(), 8);
        }

        /* 2 — encode little‑endian state words into caller buffer */
        unsafe {
            writele32(hash.as_mut_ptr().add(0), self.s()[0]);
            writele32(hash.as_mut_ptr().add(4), self.s()[1]);
            writele32(hash.as_mut_ptr().add(8), self.s()[2]);
            writele32(hash.as_mut_ptr().add(12), self.s()[3]);
            writele32(hash.as_mut_ptr().add(16), self.s()[4]);
        }

        tracing::info!(target: "ripemd160::core", "digest finalised");
    }
}

#[cfg(test)]
mod spec_finalize {
    use super::*;

    fn hex(b: &[u8]) -> String {
        b.iter().map(|v| format!("{:02x}", v)).collect()
    }

    #[traced_test]
    fn official_test_vectors() {
        let vectors = [
            ("", "9c1185a5c5e9fc54612808977ee8f548b2258d31"),
            ("a", "0bdc9d2d256b3ee9daae347be6f4dc835a467ffe"),
            ("abc", "8eb208f7e05d987a9b044a8e98c6b087f15a0bfc"),
            ("message digest", "5d0689ef49d2fae572b881b123a85ffa21595f36"),
            (
                "abcdefghijklmnopqrstuvwxyz",
                "f71c27109c692c1b56bbdceb5b9d2865b3708dbc",
            ),
        ];

        for (msg, expect_hex) in vectors {
            let mut hasher = Ripemd160::default();
            hasher.update(msg.as_bytes());

            let mut out = [0u8; RIPEMD160_OUTPUT_SIZE];
            hasher.finalize(&mut out);

            assert_eq!(
                hex(&out),
                expect_hex,
                "vector '{}' did not match reference digest",
                msg
            );
        }
    }
}
