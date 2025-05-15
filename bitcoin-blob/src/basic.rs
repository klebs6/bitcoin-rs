crate::ix!();

impl<const BITS: usize> BaseBlob<BITS>
where
    [u8; (BITS % 8) + usize::MAX]: ,
    [(); base_blob_width::<BITS>()]:
{
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
