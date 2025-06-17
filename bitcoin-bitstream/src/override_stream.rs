// ---------------- [ File: bitcoin-bitstream/src/override_stream.rs ]
crate::ix!();

#[derive(Getters, Setters, Builder)]
#[getset(get = "pub", set = "pub")]
pub struct OverrideStream<Stream> {

    #[builder(default)]
    stream: *mut Stream,

    #[builder(default)]
    n_type: i32,

    #[builder(default)]
    n_version: i32,
}

impl<Stream> StreamInto for OverrideStream<Stream> {

    #[inline]
    fn stream_into<Item>(&self, _rhs: &mut Item) {
        todo!("OverrideStream::stream_into placeholder called");
        // ::Serialize(*this, obj) in C++ (not implemented here)
    }
}

impl<Stream> StreamItems for OverrideStream<Stream> {

    #[inline]
    fn stream<Item>(&mut self, _x: Item) {
        todo!("OverrideStream::stream placeholder called");
        // ::Unserialize(*this, obj) in C++ (not implemented here)
    }
}

impl<Stream> GetType for OverrideStream<Stream> {

    fn get_type(&self) -> i32 {
        self.n_type
    }
}

impl<Stream> GetVersion for OverrideStream<Stream> {

    fn get_version(&self) -> i32 {
        self.n_version
    }
}

/// Backend API required by `OverrideStream`.
pub trait Backend: Read + Write {
    fn size(&self) -> usize;
    fn ignore(&mut self, amount: usize);
}

impl<Stream: Backend> OverrideStream<Stream> {

    #[instrument(level = "trace")]
    pub fn new(stream_ptr: *mut Stream, n_type_in: i32, n_version_in: i32) -> Self {
        info!("Constructing OverrideStream, type={} version={}", n_type_in, n_version_in);
        Self {
            stream: stream_ptr,
            n_type: n_type_in,
            n_version: n_version_in,
        }
    }

    #[instrument(level = "trace", skip(self, pch))]
    pub fn write(&mut self, pch: *const u8, n_size: usize) {
        info!("OverrideStream::write called, n_size={}", n_size);
        unsafe {
            if let Some(s) = self.stream.as_mut() {
                let slice = std::slice::from_raw_parts(pch, n_size);
                s.write_all(slice).expect("OverrideStream backend write failed");
            } else {
                error!("OverrideStream has null pointer to Stream");
                panic!("OverrideStream::write called on null stream pointer");
            }
        }
    }

    #[instrument(level = "trace", skip(self, pch))]
    pub fn read(&mut self, pch: *mut u8, n_size: usize) {
        info!("OverrideStream::read called, n_size={}", n_size);
        unsafe {
            if let Some(s) = self.stream.as_mut() {
                let slice = std::slice::from_raw_parts_mut(pch, n_size);
                s.read_exact(slice).expect("OverrideStream backend read failed");
            } else {
                error!("OverrideStream has null pointer to Stream");
                panic!("OverrideStream::read called on null stream pointer");
            }
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub fn size(&self) -> usize {
        unsafe {
            self.stream.as_ref().map_or(0, |s| s.size())
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub fn ignore(&mut self, amount: usize) {
        info!("OverrideStream::ignore called, amount={}", amount);
        unsafe {
            if let Some(s) = self.stream.as_mut() {
                s.ignore(amount);
            } else {
                error!("OverrideStream has null pointer to Stream");
                panic!("OverrideStream::ignore called on null stream pointer");
            }
        }
    }
}

#[cfg(test)]
mod test_override_stream {
    use super::*;

    // Mock "Stream" type for demonstration
    #[derive(Default)]
    struct MockStream {
        data: Vec<u8>,
        pos:  usize,
    }

    impl MockStream {

        fn new() -> Self { 
            Self { data: vec![], pos: 0 }
        }

        fn size(&self) -> usize {
            self.data.len().saturating_sub(self.pos)
        }

        fn ignore(&mut self, amt: usize) {
            if self.pos + amt > self.data.len() {
                panic!("MockStream ignore beyond end of data");
            }
            self.pos += amt;
        }
    }

    impl Read for MockStream {

        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let remain = self.data.len().saturating_sub(self.pos);
            let n = remain.min(buf.len());
            buf[..n].copy_from_slice(&self.data[self.pos..self.pos + n]);
            self.pos += n;
            Ok(n)
        }
    }

    impl Write for MockStream {

        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl Backend for MockStream {

        fn size(&self) -> usize { 
            self.data.len() - self.pos 
        }

        fn ignore(&mut self, n: usize) { 
            self.pos = (self.pos + n).min(self.data.len()); 
        }
    }

    #[traced_test]
    fn test_override_stream_usage() {
        let mut mock = MockStream::new();
        let ptr = &mut mock as *mut MockStream;

        let mut over_stream = OverrideStream::new(ptr, 99, 101);
        assert_eq!(over_stream.get_type(), 99);
        assert_eq!(over_stream.get_version(), 101);

        // Write
        let data = b"Hello";
        over_stream.write(data.as_ptr(), data.len());

        // Read
        let mut buf = vec![0u8; 5];
        over_stream.read(buf.as_mut_ptr(), 5);
        assert_eq!(&buf, b"Hello");

        // size should be 0 now
        assert_eq!(over_stream.size(), 0);
    }
}
