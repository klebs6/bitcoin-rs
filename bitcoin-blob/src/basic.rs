// ---------------- [ File: bitcoin-blob/src/basic.rs ]
crate::ix!();

impl<const BITS: usize> BaseBlob<BITS>
where
    [u8; (BITS % 8) + usize::MAX]: ,
    [(); base_blob_width::<BITS>()]:
{
    /// Returns a borrowed slice of all the bytes for read-only access.
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Returns a borrowed slice of all the bytes for mutable access.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn is_null(&self) -> bool {
        trace!(
            "Checking is_null() on BaseBlob<{}>; data={:X?}",
            BITS,
            self.data
        );

        for &byte in self.data.iter() {
            if byte != 0 {
                return false;
            }
        }
        true
    }

    pub fn set_null(&mut self) {
        trace!(
            "set_null() => filling BaseBlob<{}> with zeros.",
            BITS
        );
        self.data.fill(0);
    }

    #[inline]
    pub fn compare(&self, other: &BaseBlob<BITS>) -> i32 {
        trace!(
            "compare() => comparing BaseBlob<{}> with another, by bytes.",
            BITS
        );

        match self.data.cmp(&other.data) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }

    pub fn data(&self) -> *const u8 {
        trace!(
            "Returning const pointer to BaseBlob<{}>.data()",
            BITS
        );
        self.data.as_ptr()
    }

    pub fn data_mut(&mut self) -> *mut u8 {
        trace!(
            "Returning mut pointer to BaseBlob<{}>.data_mut()",
            BITS
        );
        self.data.as_mut_ptr()
    }

    pub fn begin_mut(&mut self) -> *mut u8 {
        trace!(
            "begin_mut(): &mut self.data[0] for BaseBlob<{}>",
            BITS
        );
        &mut self.data[0] as *mut _
    }

    pub fn end_mut(&mut self) -> *mut u8 {
        trace!(
            "end_mut(): pointer to one-past-last byte of BaseBlob<{}>",
            BITS
        );
        let width = base_blob_width::<BITS>();
        // This is safe for a pointer to the "end" of the array
        unsafe { self.data.as_mut_ptr().add(width) }
    }

    pub fn begin(&self) -> *const u8 {
        trace!(
            "begin(): &self.data[0] for BaseBlob<{}>",
            BITS
        );
        &self.data[0] as *const _
    }

    pub fn end(&self) -> *const u8 {
        trace!(
            "end(): pointer to one-past-last byte of BaseBlob<{}>",
            BITS
        );
        let width = base_blob_width::<BITS>();
        // Similarly safe pointer to the end
        unsafe { self.data.as_ptr().add(width) }
    }

    pub fn size(&self) -> u32 {
        // The total size in bytes
        let sz = base_blob_width::<BITS>() as u32;
        trace!(
            "size() => returning {} for BaseBlob<{}>",
            sz,
            BITS
        );
        sz
    }
}

#[cfg(test)]
mod base_blob_basic_methods_exhaustive_tests {
    use super::*;

    // ------------------------------------------------------------------------
    // is_null() & set_null()

    #[traced_test]
    fn test_is_null_and_set_null() {
        info!("Testing is_null() & set_null() for B=8, B=64, B=256...");
        test_is_null_and_set_null_gen::<8>();
        test_is_null_and_set_null_gen::<64>();
        test_is_null_and_set_null_gen::<256>();
        info!("is_null() & set_null() tests concluded successfully.");
    }

    fn test_is_null_and_set_null_gen<const B: usize>()
    where
        [u8; (B % 8) + usize::MAX]:,
        [(); base_blob_width::<B>()]:
    {
        // 1) Start with a default (zero) blob => is_null() => true
        let blob = BaseBlob::<B>::default();
        assert!(
            blob.is_null(),
            "Default zero blob => is_null() should be true for B={}",
            B
        );

        // 2) Fill a second blob with some nonzero data => is_null() => false
        let mut nonzero_blob = make_test_blob::<B>(true);
        assert!(
            !nonzero_blob.is_null(),
            "Random data => is_null() should be false for B={}",
            B
        );

        // 3) call set_null => is_null() => true
        nonzero_blob.set_null();
        assert!(
            nonzero_blob.is_null(),
            "After set_null(), is_null() should be true for B={}",
            B
        );
    }

    // ------------------------------------------------------------------------
    // compare()

    #[traced_test]
    fn test_compare() {
        info!("Testing compare() for B=8, B=64, B=256...");
        test_compare_gen::<8>();
        test_compare_gen::<64>();
        test_compare_gen::<256>();
        info!("compare() tests concluded successfully.");
    }

    fn test_compare_gen<const B: usize>()
    where
        [u8; (B % 8) + usize::MAX]:,
        [(); base_blob_width::<B>()]:
    {
        let width = base_blob_width::<B>();

        // zero
        let zero_blob = BaseBlob::<B>::default();

        // ones
        let mut ones_blob = BaseBlob::<B>::default();
        for b in ones_blob.data.iter_mut() {
            *b = 0xFF;
        }

        // "mid" => if width=1 (B=8), we pick 0x7F; else half=0 and half=0xFF
        let mut mid_blob = BaseBlob::<B>::default();
        if width == 1 {
            // single byte: make it 0x7F => strictly between 0x00 and 0xFF
            mid_blob.data[0] = 0x7F;
        } else {
            let half = width / 2;
            for b in mid_blob.data[half..].iter_mut() {
                *b = 0xFF;
            }
        }

        // zero < mid
        assert!(
            zero_blob.compare(&mid_blob) < 0,
            "compare(zero, mid) => expected negative for B={}",
            B
        );

        // mid < ones
        assert!(
            mid_blob.compare(&ones_blob) < 0,
            "compare(mid, ones) => expected negative for B={}",
            B
        );

        // zero < ones
        assert!(
            zero_blob.compare(&ones_blob) < 0,
            "compare(zero, ones) => expected negative for B={}",
            B
        );

        // ones > zero
        assert!(
            ones_blob.compare(&zero_blob) > 0,
            "compare(ones, zero) => expected positive for B={}",
            B
        );

        // mid == mid => 0
        assert_eq!(
            mid_blob.compare(&mid_blob),
            0,
            "compare(mid, mid) => expected 0 for B={}",
            B
        );
    }

    // ------------------------------------------------------------------------
    // data() / data_mut() / begin() / end() / begin_mut() / end_mut()

    #[traced_test]
    fn test_data_pointers() {
        info!("Testing pointer-based methods for B=8, B=64, B=256...");
        test_data_pointers_gen::<8>();
        test_data_pointers_gen::<64>();
        test_data_pointers_gen::<256>();
        info!("data pointer tests concluded successfully.");
    }

    fn test_data_pointers_gen<const B: usize>()
    where
        [u8; (B % 8) + usize::MAX]:,
        [(); base_blob_width::<B>()]:
    {
        let mut blob = make_test_blob::<B>(true);
        let width = base_blob_width::<B>();

        // data() => non-null
        let ptr_const = blob.data();
        assert!(
            !ptr_const.is_null(),
            "data() => pointer must not be null for B={}",
            B
        );

        // data_mut() => non-null
        let ptr_mut = blob.data_mut();
        assert!(
            !ptr_mut.is_null(),
            "data_mut() => pointer must not be null for B={}",
            B
        );

        // begin() / end() => difference == width
        let begin_const = blob.begin();
        let end_const = blob.end();
        let diff_const = (end_const as usize) - (begin_const as usize);
        assert_eq!(
            diff_const, width,
            "end() - begin() => {} for B={}",
            width,
            B
        );

        // begin_mut() / end_mut() => difference == width
        let begin_mut = blob.begin_mut();
        let end_mut = blob.end_mut();
        let diff_mut = (end_mut as usize) - (begin_mut as usize);
        assert_eq!(
            diff_mut, width,
            "end_mut() - begin_mut() => {} for B={}",
            width,
            B
        );
    }

    // ------------------------------------------------------------------------
    // size()

    #[traced_test]
    fn test_size() {
        info!("Testing size() for B=8, B=64, B=256...");
        test_size_gen::<8>();
        test_size_gen::<64>();
        test_size_gen::<256>();
        info!("size() tests concluded successfully.");
    }

    fn test_size_gen<const B: usize>()
    where
        [u8; (B % 8) + usize::MAX]:,
        [(); base_blob_width::<B>()]:
    {
        let blob = make_test_blob::<B>(true);
        let expected_sz = base_blob_width::<B>() as u32;
        let got_sz = blob.size();
        assert_eq!(
            got_sz, expected_sz,
            "size() => {} but expected {} for B={}",
            got_sz, expected_sz, B
        );
    }

    // ------------------------------------------------------------------------
    // Helper: create a BaseBlob<B> with either zero data (if fill_nonzero=false)
    // or a deterministic pattern. For single-byte (B=8), we set it to 0xAA if fill_nonzero
    // is true, ensuring is_null() becomes false.
    fn make_test_blob<const B: usize>(fill_nonzero: bool) -> BaseBlob<B>
    where
        [u8; (B % 8) + usize::MAX]:,
        [(); base_blob_width::<B>()]:
    {
        let mut blob = BaseBlob::<B>::default();
        if fill_nonzero {
            let width = base_blob_width::<B>();
            if width == 1 {
                // B=8 => single byte => set to 0xAA
                blob.data[0] = 0xAA;
            } else {
                // For bigger widths, fill each byte with i
                for (i, b) in blob.data.iter_mut().enumerate() {
                    *b = i as u8;
                }
            }
        }
        blob
    }
}
