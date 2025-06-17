// ---------------- [ File: bitcoin-bitstream/src/bitstream_reader.rs ]
crate::ix!();

#[derive(Getters, Setters, Builder)]
#[getset(get = "pub", set = "pub")]
pub struct BitStreamReader<IStream: Default + Read> {
    #[builder(default)]
    istream: Rc<RefCell<IStream>>,

    #[builder(default)]
    buffer:  u8,

    #[builder(default)]
    offset:  i32,
}

impl<IStream: Default + Read> BitStreamReader<IStream> {

    #[instrument(level = "trace", skip(istream))]
    pub fn new(istream: Rc<RefCell<IStream>>) -> Self {
        info!("Constructing BitStreamReader");
        Self {
            istream,
            buffer: 0,
            offset: 8, // match the original c++ default
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub fn read(&mut self, mut nbits: i32) -> u64 {
        use std::io::{Read as _, Error};          // bring trait into scope

        info!("Reading {} bits from BitStreamReader", nbits);

        if !(0..=64).contains(&nbits) {
            error!("Invalid nbits: {}", nbits);
            panic!("nbits must be between 0 and 64");
        }

        let mut data: u64 = 0;
        while nbits > 0 {
            if self.offset == 8 {
                trace!("Offset == 8; fetching next byte from underlying stream");
                let mut one_byte = [0u8; 1];
                self.istream
                    .borrow_mut()
                    .read_exact(&mut one_byte)
                    .expect("BitStreamReader: underlying stream read failed");
                self.buffer = one_byte[0];
                self.offset = 0;
            }

            let bits = (8 - self.offset).min(nbits);
            data <<= bits;

            let shifted = ((self.buffer << self.offset) & 0xFF) >> (8 - bits);
            data |= shifted as u64;

            self.offset += bits;
            nbits       -= bits;
        }

        debug!("BitStreamReader returning data={}", data);
        data
    }
}

#[cfg(test)]
mod test_bitstream_reader {
    use super::*;

    // A trivial "stream" that just returns 
    //  the data from an internal buffer
    #[derive(Default)]
    struct MockInput {
        data: Vec<u8>,
        pos:  usize,
    }

    impl MockInput {
        fn new(data: Vec<u8>) -> Self { Self { data, pos: 0 } }
    }

    impl Read for MockInput {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let remaining = self.data.len().saturating_sub(self.pos);
            if remaining == 0 { return Ok(0); }
            let n = remaining.min(buf.len());
            buf[..n].copy_from_slice(&self.data[self.pos..self.pos + n]);
            self.pos += n;
            Ok(n)
        }
    }

    #[traced_test]
    fn test_read_bits_simple() {
        let mock_data = vec![0b1010_1010, 0b1100_0011];
        let rc_stream = std::rc::Rc::new(std::cell::RefCell::new(MockInput::new(mock_data)));

        let mut reader = BitStreamReader::new(rc_stream);

        // Read first 4 bits
        let val1 = reader.read(4);
        assert_eq!(val1, 0b1010);

        // Read next 4 bits
        let val2 = reader.read(4);
        assert_eq!(val2, 0b1010);

        // Read next 8 bits (from next byte)
        let val3 = reader.read(8);
        assert_eq!(val3, 0b1100_0011);
    }
}
