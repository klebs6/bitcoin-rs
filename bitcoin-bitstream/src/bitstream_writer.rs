// ---------------- [ File: bitcoin-bitstream/src/bitstream_writer.rs ]
crate::ix!();

#[derive(Getters, Setters, Builder)]
#[getset(get = "pub", set = "pub")]
pub struct BitStreamWriter<OStream: Default + Write> {

    #[builder(default)]
    ostream: Rc<RefCell<OStream>>,

    #[builder(default)]
    buffer:  u8,

    #[builder(default)]
    offset:  i32,
}

impl<OStream: Default + Write> Drop for BitStreamWriter<OStream> {

    #[instrument(level = "trace", skip(self))]
    fn drop(&mut self) {
        info!("Dropping BitStreamWriter, flushing pending bits");
        self.flush();
    }
}

impl<OStream: Default + Write> BitStreamWriter<OStream> {

    #[instrument(level = "trace", skip(ostream))]
    pub fn new(ostream: Rc<RefCell<OStream>>) -> Self {
        info!("Constructing BitStreamWriter");
        Self {
            ostream,
            buffer: 0,
            offset: 0,
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub fn write(&mut self, data: u64, mut nbits: i32) {
        info!("BitStreamWriter writing {} bits from data={}", nbits, data);

        if nbits < 0 || nbits > 64 {
            error!("Invalid nbits: {}", nbits);
            panic!("nbits must be between 0 and 64");
        }

        while nbits > 0 {
            let bits = (8 - self.offset).min(nbits);
            // from c++: m_buffer |= (data << (64 - nbits)) >> (64 - 8 + m_offset);

            let shift_amount = 64 - nbits;
            let partial = ((data << shift_amount) >> (64 - 8 + self.offset)) as u8;
            self.buffer |= partial;

            self.offset += bits;
            nbits       -= bits;

            if self.offset == 8 {
                self.flush();
            }
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub fn flush(&mut self) {
        use std::io::Write as _;                  // bring trait into scope

        debug!("BitStreamWriter flush called, offset={}", self.offset);

        if self.offset == 0 {
            trace!("Nothing to flush (offset = 0)");
            return;
        }

        let one_byte = [self.buffer];
        self.ostream
            .borrow_mut()
            .write_all(&one_byte)
            .expect("BitStreamWriter: underlying stream write failed");

        self.buffer = 0;
        self.offset = 0;
    }
}

#[cfg(test)]
mod test_bitstream_writer {
    use super::*;
    use traced_test::traced_test;

    // A trivial "stream" that just appends 
    //  any written bytes to an internal buffer
    #[derive(Default)]
    struct MockOutput {
        data: Vec<u8>,
    }

    impl MockOutput {
        fn new() -> Self {
            Self { data: vec![] }
        }
    }

    impl Write for MockOutput {

        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[traced_test]
    fn test_write_bits_simple() {
        let mock_out = std::rc::Rc::new(std::cell::RefCell::new(MockOutput::new()));
        let mut writer = BitStreamWriter::new(mock_out.clone());

        // Write 4 bits: 0b1010
        writer.write(0b1010, 4);

        // Write 4 bits: 0b0101
        writer.write(0b0101, 4);

        // By now we should have 1 full byte: 0b1010_0101 = 0xA5
        assert_eq!(mock_out.borrow().data, vec![0xA5]);

        // Write 8 bits: 0b11110000
        writer.write(0b11110000, 8);

        // We should have 2 bytes total: 0xA5, then 0b11110000 = 0xF0
        assert_eq!(mock_out.borrow().data, vec![0xA5, 0xF0]);
    }
}
