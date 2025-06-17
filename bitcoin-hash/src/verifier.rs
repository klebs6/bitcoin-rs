// ---------------- [ File: bitcoin-hash/src/verifier.rs ]
crate::ix!();

/// Reads data from an underlying stream,
/// while hashing the read data.
///
/// Stream wrapper that feeds every byte read
/// into an internal `HashWriter`.
pub struct HashVerifier<S> {
    base:   HashWriter,
    source: S,
}

impl<S> HashVerifier<S>
where
    S: Debug + Read,
{
    #[instrument(level = "trace")]
    pub fn new(source: S) -> Self {
        Self {
            base: HashWriter::new(SER_GETHASH as i32, 0),
            source,
        }
    }

    #[instrument(level = "trace", skip(self, buf))]
    pub fn read(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        self.source.read_exact(buf)?;
        self.base.write(buf);
        Ok(())
    }

    #[instrument(level = "trace", skip(self))]
    pub fn ignore(&mut self, mut n: usize) -> std::io::Result<()> {
        let mut tmp = [0u8; 1024];
        while n != 0 {
            let now = n.min(tmp.len());
            self.read(&mut tmp[..now])?;
            n -= now;
        }
        Ok(())
    }
}

impl<Src, T> Shr<&mut T> for HashVerifier<Src>
where
    Src: Read + Debug,
    T: AsMut<[u8]>,
{
    type Output = HashVerifier<Src>;

    /// Read exactly `obj.as_mut().len()` bytes from the wrapped
    /// stream, hash them, and store them in `obj`.
    #[inline]
    fn shr(mut self, rhs: &mut T) -> Self::Output {
        let buf = rhs.as_mut();
        // Ignore I/O errors – propagate via panic just like C++’s
        // stream exceptions.
        self.read(buf).expect("HashVerifier::read failed");
        self
    }
}

#[cfg(test)]
mod verifier_shr_spec {
    use super::*;
    use std::io::Cursor;

    #[traced_test]
    fn shr_reads_exactly_into_buffer() {
        let src = Cursor::new(b"xyz");
        let mut verifier = HashVerifier::new(src);
        let mut buf = [0u8; 3];
        let _ = verifier >> &mut buf;
        assert_eq!(&buf, b"xyz");
    }
}
