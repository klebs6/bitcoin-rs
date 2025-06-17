// ---------------- [ File: bitcoin-bitstream/src/data_stream.rs ]
crate::ix!();

type ZeroAfterFreeVecIter = std::vec::IntoIter<u8, ZeroAfterFreeAllocator>;

#[derive(Getters, Setters, Builder)]
#[getset(get = "pub", set = "pub")]
pub struct DataStream {

    vch: SerializeData,

    #[builder(default)]
    n_read_pos: u32,

    #[builder(default)]
    n_type: i32,

    #[builder(default)]
    n_version: i32,
}

impl std::ops::Index<usize> for DataStream {
    type Output = u8;

    #[inline]
    fn index(&self, pos: usize) -> &Self::Output {
        &self.vch[pos + self.n_read_pos as usize]
    }
}

impl std::ops::IndexMut<usize> for DataStream {
    #[inline]
    fn index_mut(&mut self, pos: usize) -> &mut Self::Output {
        &mut self.vch[pos + self.n_read_pos as usize]
    }
}

impl StreamInto for DataStream {
    #[inline]
    fn stream_into<Item>(&self, _rhs: &mut Item) {
        trace!("DataStream::stream_into called, but not implemented in this translation");
        // Equivalent to ::Serialize(*this, obj) in C++
        // We do not have a direct "Serialize" function in this snippet, so it's a placeholder.
    }
}

impl StreamItems for DataStream {
    #[inline]
    fn stream<Item>(&mut self, _x: Item) {
        trace!("DataStream::stream called, but not implemented in this translation");
        // Equivalent to ::Unserialize(*this, obj) in C++
        // We do not have a direct "Unserialize" function in this snippet, so it's a placeholder.
    }
}

impl DataStream {

    #[instrument(level = "trace")]
    pub fn new(n_type_in: i32, n_version_in: i32) -> Self {
        info!("Constructing DataStream (empty), type={} version={}", n_type_in, n_version_in);
        Self {
            vch: Vec::<u8,ZeroAfterFreeAllocator>::new_in(ZeroAfterFreeAllocator),
            n_read_pos: 0,
            n_type: n_type_in,
            n_version: n_version_in,
        }
    }

    #[instrument(level = "trace", skip(sp))]
    pub fn new_with_slice(sp: &[u8], n_type_in: i32, n_version_in: i32) -> Self {
        info!("Constructing DataStream from slice, type={} version={}", n_type_in, n_version_in);
        let mut v: SerializeData = Vec::<u8,ZeroAfterFreeAllocator>::new_in(ZeroAfterFreeAllocator);
        v.extend_from_slice(sp);

        Self {
            vch: v,
            n_read_pos: 0,
            n_type: n_type_in,
            n_version: n_version_in,
        }
    }

    #[instrument(level = "trace", skip(_args))]
    pub fn new_with_args<Args>(n_type_in: i32, n_version_in: i32, _args: Args) -> Self {
        info!("Constructing DataStream with args, type={} version={}", n_type_in, n_version_in);
        // In C++ code, this calls ::SerializeMany(*this, args...).
        // This snippet is a placeholder, as we do not have the real definition for SerializeMany.
        Self {
            vch: Vec::<u8,ZeroAfterFreeAllocator>::new_in(ZeroAfterFreeAllocator),
            n_read_pos: 0,
            n_type: n_type_in,
            n_version: n_version_in,
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub fn str_(&self) -> String {
        info!("DataStream::str_ called, returning string from internal buffer");
        String::from_utf8_lossy(&self.vch[self.n_read_pos as usize..]).to_string()
    }

    // ----------------- Vector subset  -----------------

    #[instrument(level = "trace", skip(self))]
    pub fn begin(&self) -> std::slice::Iter<'_, u8> {
        self.vch[self.n_read_pos as usize..].iter()
    }

    #[instrument(level = "trace", skip(self))]
    pub fn begin_mut(&mut self) -> std::slice::IterMut<'_, u8> {
        let start = self.n_read_pos as usize;
        self.vch[start..].iter_mut()
    }

    #[instrument(level = "trace", skip(self))]
    pub fn end(&self) -> std::slice::Iter<'_, u8> {
        // This returns an empty iterator at the end
        self.vch[self.vch.len()..].iter()
    }

    #[instrument(level = "trace", skip(self))]
    pub fn end_mut(&mut self) -> std::slice::IterMut<'_, u8> {
        let end = self.vch.len();
        self.vch[end..].iter_mut()
    }

    #[instrument(level = "trace", skip(self))]
    pub fn size(&self) -> usize {
        let sz = self.vch.len().saturating_sub(self.n_read_pos as usize);
        trace!("DataStream::size returning {}", sz);
        sz
    }

    #[instrument(level = "trace", skip(self))]
    pub fn empty(&self) -> bool {
        let is_empty = self.n_read_pos as usize == self.vch.len();
        trace!("DataStream::empty returning {}", is_empty);
        is_empty
    }

    #[instrument(level = "trace", skip(self))]
    pub fn resize(&mut self, n: usize, c: Option<u8>) {
        let fill = c.unwrap_or(0);
        info!("DataStream::resize to {}, fill={}", n, fill);
        self.vch.resize(n + self.n_read_pos as usize, fill);
    }

    #[instrument(level = "trace", skip(self))]
    pub fn reserve(&mut self, n: usize) {
        info!("DataStream::reserve to {}", n);
        self.vch.reserve(n + self.n_read_pos as usize);
    }

    #[instrument(level = "trace", skip(self))]
    pub fn clear(&mut self) {
        info!("DataStream::clear called");
        self.vch.clear();
        self.n_read_pos = 0;
    }

    #[instrument(level = "trace", skip(self, it))]
    pub fn insert_item(
        &mut self,
        it: std::slice::Iter<'_, u8>,
        x:  u8,
    ) -> ZeroAfterFreeVecIter {
        info!("DataStream::insert_item called, single byte={}", x);
        let idx = (it.as_slice().as_ptr() as usize)
            .saturating_sub(self.vch.as_ptr() as usize);
        self.vch.insert(idx, x);
        self.vch.clone().into_iter()
    }

    #[instrument(level = "trace", skip(self))]
    pub fn insert_multi(
        &mut self,
        it: std::slice::Iter<'_, u8>,
        n:  usize,
        x:  u8,
    ) {
        info!("DataStream::insert_multi called, inserting {} copies of {}", n, x);
        let idx = (it.as_slice().as_ptr() as usize)
            .saturating_sub(self.vch.as_ptr() as usize);
        self.vch.splice(idx..idx, std::iter::repeat(x).take(n));
    }

    #[instrument(level = "trace", skip(self))]
    pub fn as_slice(&self) -> &[u8] {
        let start = self.n_read_pos as usize;
        &self.vch[start..]
    }

    #[instrument(level = "trace", skip(self))]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        let start = self.n_read_pos as usize;
        &mut self.vch[start..]
    }

    #[instrument(level = "trace", skip(self))]
    pub fn data_mut(&mut self) -> *mut u8 {
        let start = self.n_read_pos as usize;
        let ptr = self.vch.as_mut_ptr().wrapping_add(start);
        trace!("DataStream::data_mut returning pointer={:?}", ptr);
        ptr
    }

    #[instrument(level = "trace", skip(self))]
    pub fn data(&self) -> *const u8 {
        let start = self.n_read_pos as usize;
        let ptr = self.vch.as_ptr().wrapping_add(start);
        trace!("DataStream::data returning pointer={:?}", ptr);
        ptr
    }

    #[instrument(level = "trace", skip(self, first, last))]
    pub fn insert_with_iterator_range(
        &mut self,
        it:    std::slice::Iter<'_, u8>,
        mut first: std::slice::Iter<'_, u8>,
        mut last:  std::slice::Iter<'_, u8>,
    ) {
        info!("DataStream::insert_with_iterator_range called");
        let first_ptr = first.as_slice().as_ptr() as usize;
        let last_ptr  = last.as_slice().as_ptr() as usize;
        if last_ptr == first_ptr {
            trace!("Nothing to insert; last == first");
            return;
        }
        let count = last_ptr.saturating_sub(first_ptr);
        let idx   = (it.as_slice().as_ptr() as usize)
            .saturating_sub(self.vch.as_ptr() as usize);

        // In the original C++: if it==vch.begin()+nReadPos and count <= nReadPos, do special front insert
        let front_idx = self.n_read_pos as usize;
        if idx == front_idx && count <= front_idx {
            trace!("Performing special front insertion with memmove");
            self.n_read_pos -= count as u32;
            // Move the data into the front region:
            unsafe {
                let dst = self.vch.as_mut_ptr().add(self.n_read_pos as usize);
                let src = first.as_slice().as_ptr();
                std::ptr::copy(src, dst, count);
            }
        } else {
            let src_slice = &first.as_slice()[..count];
            self.vch.splice(idx..idx, src_slice.iter().cloned());
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub fn insert_with_pointer_range(
        &mut self,
        it: std::slice::Iter<'_, u8>,
        first: *const u8,
        last:  *const u8,
    ) {
        info!("DataStream::insert_with_pointer_range called");
        let count = unsafe { last.offset_from(first) };
        if count <= 0 {
            trace!("Nothing to insert; last == first or invalid range");
            return;
        }
        let idx = (it.as_slice().as_ptr() as usize)
            .saturating_sub(self.vch.as_ptr() as usize);

        let front_idx = self.n_read_pos as usize;
        if idx == front_idx && (count as u32) <= self.n_read_pos {
            trace!("Performing special front insertion with memmove");
            self.n_read_pos -= count as u32;
            unsafe {
                let dst = self.vch.as_mut_ptr().add(self.n_read_pos as usize);
                std::ptr::copy_nonoverlapping(first, dst, count as usize);
            }
        } else {
            let src_slice = unsafe {
                std::slice::from_raw_parts(first, count as usize)
            };
            self.vch.splice(idx..idx, src_slice.iter().cloned());
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub fn erase(
        &mut self,
        it: std::slice::Iter<'_, u8>,
    ) -> ZeroAfterFreeVecIter {
        info!("DataStream::erase called");
        let idx = (it.as_slice().as_ptr() as usize)
            .saturating_sub(self.vch.as_ptr() as usize);

        if idx == self.n_read_pos as usize {
            self.n_read_pos += 1;
            if self.n_read_pos as usize >= self.vch.len() {
                trace!("Erasing everything; clearing data");
                self.n_read_pos = 0;
                self.vch.clear();
            }
        } else {
            self.vch.remove(idx);
        }
        self.vch.clone().into_iter()
    }

    #[instrument(level = "trace", skip(self))]
    pub fn erase_range(
        &mut self,
        first: std::slice::Iter<'_, u8>,
        last:  std::slice::Iter<'_, u8>,
    ) -> ZeroAfterFreeVecIter {
        info!("DataStream::erase_range called");
        let first_idx = (first.as_slice().as_ptr() as usize)
            .saturating_sub(self.vch.as_ptr() as usize);
        let last_idx  = (last.as_slice().as_ptr() as usize)
            .saturating_sub(self.vch.as_ptr() as usize);

        if first_idx == self.n_read_pos as usize && last_idx == self.vch.len() {
            trace!("Front‑to‑end erase; clearing");
            self.n_read_pos = 0;
            self.vch.clear();
        } else if first_idx == self.n_read_pos as usize {
            trace!("Front‑to‑mid erase");
            self.n_read_pos = last_idx as u32;
        } else {
            self.vch.drain(first_idx..last_idx);
        }
        self.vch.clone().into_iter()
    }

    #[instrument(level = "trace", skip(self))]
    pub fn compact(&mut self) {
        info!("DataStream::compact called");
        let front_idx = self.n_read_pos as usize;
        if front_idx > 0 && front_idx <= self.vch.len() {
            self.vch.drain(0..front_idx);
        }
        self.n_read_pos = 0;
    }

    #[instrument(level = "trace", skip(self))]
    pub fn rewind(&mut self, n: Option<usize>) -> bool {
        info!("DataStream::rewind called with n={:?}", n);
        match n {
            None => {
                self.n_read_pos = 0;
                true
            }
            Some(val) => {
                if val > self.n_read_pos as usize {
                    trace!("Cannot rewind; requested more than n_read_pos");
                    false
                } else {
                    self.n_read_pos -= val as u32;
                    true
                }
            }
        }
    }

    // ----------------- Stream subset  -----------------

    #[instrument(level = "trace", skip(self))]
    pub fn eof(&self) -> bool {
        let end = self.size() == 0;
        trace!("DataStream::eof returning {}", end);
        end
    }

    #[instrument(level = "trace", skip(self))]
    pub fn rdbuf(&mut self) -> *mut DataStream {
        let ptr = self as *mut DataStream;
        trace!("DataStream::rdbuf returning self pointer={:?}", ptr);
        ptr
    }

    #[instrument(level = "trace", skip(self))]
    pub fn in_avail(&self) -> i32 {
        let avail = self.size() as i32;
        trace!("DataStream::in_avail returning {}", avail);
        avail
    }

    #[instrument(level = "trace", skip(self))]
    pub fn set_type(&mut self, n: i32) {
        info!("DataStream::set_type to {}", n);
        self.n_type = n;
    }

    #[instrument(level = "trace", skip(self))]
    pub fn get_type(&self) -> i32 {
        trace!("DataStream::get_type returning {}", self.n_type);
        self.n_type
    }

    #[instrument(level = "trace", skip(self))]
    pub fn set_version(&mut self, n: i32) {
        info!("DataStream::set_version to {}", n);
        self.n_version = n;
    }

    #[instrument(level = "trace", skip(self))]
    pub fn get_version(&self) -> i32 {
        trace!("DataStream::get_version returning {}", self.n_version);
        self.n_version
    }

    #[instrument(level = "trace", skip(self, pch))]
    pub fn read(&mut self, pch: *mut u8, n_size: usize) {
        info!("DataStream::read called, n_size={}", n_size);
        if n_size == 0 {
            trace!("Nothing to read, returning early");
            return;
        }
        let n_read_pos_next = self.n_read_pos as usize + n_size;
        if n_read_pos_next > self.vch.len() {
            error!("DataStream::read end of data: requested beyond buffer");
            panic!("DataStream::read(): end of data");
        }
        unsafe {
            let src_slice = &self.vch[self.n_read_pos as usize..n_read_pos_next];
            std::ptr::copy_nonoverlapping(src_slice.as_ptr(), pch, n_size);
        }
        if n_read_pos_next == self.vch.len() {
            trace!("Read consumed entire buffer; clearing");
            self.n_read_pos = 0;
            self.vch.clear();
            return;
        }
        self.n_read_pos = n_read_pos_next as u32;
    }

    #[instrument(level = "trace", skip(self))]
    pub fn ignore(&mut self, n_size: i32) {
        info!("DataStream::ignore called, n_size={}", n_size);
        if n_size < 0 {
            error!("Negative n_size passed to DataStream::ignore");
            panic!("DataStream::ignore(): nSize negative");
        }
        let next = self.n_read_pos as i32 + n_size;
        if next as usize >= self.vch.len() {
            if next as usize > self.vch.len() {
                error!("Attempted to ignore beyond end of data in DataStream");
                panic!("DataStream::ignore(): end of data");
            }
            trace!("Ignoring up to the end; clearing");
            self.n_read_pos = 0;
            self.vch.clear();
            return;
        }
        self.n_read_pos = next as u32;
    }

    #[instrument(level = "trace", skip(self, pch))]
    pub fn write(&mut self, pch: *const u8, n_size: usize) {
        info!("DataStream::write called, n_size={}", n_size);
        unsafe {
            let slice = std::slice::from_raw_parts(pch, n_size);
            self.vch.extend_from_slice(slice);
        }
    }

    #[instrument(level = "trace", skip(self, _s))]
    pub fn serialize<Stream>(&self, _s: &mut Stream) {
        info!("DataStream::serialize placeholder called");
        // In C++: if !vch.empty(), s.write((char*)vch.data(), vch.size());
        // We have no direct definition of Stream::write in this snippet, so we omit the real call.
    }

    #[instrument(level = "trace", skip(self, key))]
    pub fn xor(&mut self, key: &Vec<u8>) {
        info!("DataStream::xor called with key len={}", key.len());
        if key.is_empty() {
            trace!("Key is empty, nothing to XOR");
            return;
        }
        let mut j = 0usize;
        let ksize = key.len();
        for i in 0..self.size() {
            self.vch[i + self.n_read_pos as usize] ^= key[j];
            j += 1;
            if j == ksize {
                j = 0;
            }
        }
    }
}

#[cfg(test)]
mod test_data_stream {
    use super::*;
    use traced_test::traced_test;

    #[traced_test]
    fn test_basic_read_write() {
        let mut ds = DataStream::new(0, 0);
        let data = b"HelloWorld";
        ds.write(data.as_ptr(), data.len());

        let mut output = vec![0u8; 5];
        ds.read(output.as_mut_ptr(), 5);
        assert_eq!(&output, b"Hello");

        assert!(!ds.empty());
        assert_eq!(ds.size(), 5);

        ds.ignore(2);
        let mut final_part = vec![0u8; 3];
        ds.read(final_part.as_mut_ptr(), 3);
        assert_eq!(&final_part, b"rld");

        assert!(ds.empty());
    }

    #[traced_test]
    fn test_xor() {
        let mut ds = DataStream::new(0, 0);
        let data = vec![0x00, 0xFF, 0xAA];
        ds.write(data.as_ptr(), data.len());
        ds.xor(&vec![0xFF]);
        // After XOR with 0xFF => 0xFF, 0x00, 0x55
        assert_eq!(ds.as_slice(), &[0xFF, 0x00, 0x55]);
    }
}
