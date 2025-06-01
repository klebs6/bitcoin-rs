// ---------------- [ File: bitcoin-blob/src/basic.rs ]
crate::ix!();

#[macro_export]
macro_rules! define_base_blob_basic {
    (
        $blob_ty:ident,
        $bits:expr,
        $bytes:expr
    ) => {

        impl $blob_ty {

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
                    $bits,
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
                    $bits
                );
                self.data.fill(0);
            }

            #[inline]
            pub fn compare(&self, other: &$blob_ty) -> i32 {
                trace!(
                    "compare() => comparing BaseBlob<{}> with another, by bytes.",
                    $bits
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
                    $bits
                );
                self.data.as_ptr()
            }

            pub fn data_mut(&mut self) -> *mut u8 {
                trace!(
                    "Returning mut pointer to BaseBlob<{}>.data_mut()",
                    $bits
                );
                self.data.as_mut_ptr()
            }

            pub fn begin_mut(&mut self) -> *mut u8 {
                trace!(
                    "begin_mut(): &mut self.data[0] for BaseBlob<{}>",
                    $bits
                );
                &mut self.data[0] as *mut _
            }

            pub fn end_mut(&mut self) -> *mut u8 {
                trace!(
                    "end_mut(): pointer to one-past-last byte of BaseBlob<{}>",
                    $bits
                );
                unsafe { self.data.as_mut_ptr().add($bytes) }
            }

            pub fn begin(&self) -> *const u8 {
                trace!(
                    "begin(): &self.data[0] for BaseBlob<{}>",
                    $bits
                );
                &self.data[0] as *const _
            }

            pub fn end(&self) -> *const u8 {
                trace!(
                    "end(): pointer to one-past-last byte of BaseBlob<{}>",
                    $bits
                );
                unsafe { self.data.as_ptr().add($bytes) }
            }

            pub fn size(&self) -> u32 {
                let sz = $bytes as u32;
                trace!(
                    "size() => returning {} for BaseBlob<{}>",
                    sz,
                    $bits
                );
                sz
            }
        }
    }
}

#[cfg(test)]
mod base_blob_basic_methods_exhaustive_tests {
    use super::*;
    use tracing::{info, trace, debug};

    #[traced_test]
    fn test_is_null_and_set_null() {
        info!("Testing is_null() & set_null() for B=8, B=64, B=256...");
        test_is_null_and_set_null_8();
        test_is_null_and_set_null_64();
        test_is_null_and_set_null_256();
        info!("is_null() & set_null() tests concluded successfully.");
    }

    fn test_is_null_and_set_null_8() {
        let blob = BaseBlob8::default();
        assert!(blob.is_null(), "Default zero => is_null()=true, B=8");
        let mut nonzero_blob = BaseBlob8::default();
        nonzero_blob.data[0] = 0xAA;
        assert!(!nonzero_blob.is_null(), "Random data => is_null()=false, B=8");
        nonzero_blob.set_null();
        assert!(nonzero_blob.is_null(), "After set_null(), is_null()=true, B=8");
    }
    fn test_is_null_and_set_null_64() {
        let blob = BaseBlob64::default();
        assert!(blob.is_null(), "Default zero => is_null()=true, B=64");
        let mut nonzero_blob = BaseBlob64::default();
        for (i, b) in nonzero_blob.data.iter_mut().enumerate() {
            *b = i as u8;
        }
        assert!(!nonzero_blob.is_null(), "Random data => is_null()=false, B=64");
        nonzero_blob.set_null();
        assert!(nonzero_blob.is_null(), "After set_null(), is_null()=true, B=64");
    }
    fn test_is_null_and_set_null_256() {
        let blob = BaseBlob256::default();
        assert!(blob.is_null(), "Default zero => is_null()=true, B=256");
        let mut nonzero_blob = BaseBlob256::default();
        for (i, b) in nonzero_blob.data.iter_mut().enumerate() {
            *b = i as u8;
        }
        assert!(!nonzero_blob.is_null(), "Random data => is_null()=false, B=256");
        nonzero_blob.set_null();
        assert!(nonzero_blob.is_null(), "After set_null(), is_null()=true, B=256");
    }

    #[traced_test]
    fn test_compare() {
        info!("Testing compare() for B=8, B=64, B=256...");
        test_compare_8();
        test_compare_64();
        test_compare_256();
        info!("compare() tests concluded successfully.");
    }

    fn test_compare_8() {
        let zero_blob = BaseBlob8::default();
        let mut ones_blob = BaseBlob8::default();
        ones_blob.data[0] = 0xFF;
        let mut mid_blob = BaseBlob8::default();
        mid_blob.data[0] = 0x7F;

        assert!(zero_blob.compare(&mid_blob) < 0, "compare(zero, mid) => negative, B=8");
        assert!(mid_blob.compare(&ones_blob) < 0, "compare(mid, ones) => negative, B=8");
        assert!(zero_blob.compare(&ones_blob) < 0, "compare(zero, ones) => negative, B=8");
        assert!(ones_blob.compare(&zero_blob) > 0, "compare(ones, zero) => positive, B=8");
        assert_eq!(mid_blob.compare(&mid_blob), 0, "compare(mid, mid) => 0, B=8");
    }
    fn test_compare_64() {
        let zero_blob = BaseBlob64::default();
        let mut ones_blob = BaseBlob64::default();
        for b in ones_blob.data.iter_mut() {
            *b = 0xFF;
        }
        let mut mid_blob = BaseBlob64::default();
        let half = 8/2;
        for b in mid_blob.data[half..].iter_mut() {
            *b = 0xFF;
        }

        assert!(zero_blob.compare(&mid_blob) < 0, "compare(zero, mid) => negative, B=64");
        assert!(mid_blob.compare(&ones_blob) < 0, "compare(mid, ones) => negative, B=64");
        assert!(zero_blob.compare(&ones_blob) < 0, "compare(zero, ones) => negative, B=64");
        assert!(ones_blob.compare(&zero_blob) > 0, "compare(ones, zero) => positive, B=64");
        assert_eq!(mid_blob.compare(&mid_blob), 0, "compare(mid, mid) => 0, B=64");
    }
    fn test_compare_256() {
        let zero_blob = BaseBlob256::default();
        let mut ones_blob = BaseBlob256::default();
        for b in ones_blob.data.iter_mut() {
            *b = 0xFF;
        }
        let mut mid_blob = BaseBlob256::default();
        let half = 32/2;
        for b in mid_blob.data[half..].iter_mut() {
            *b = 0xFF;
        }

        assert!(zero_blob.compare(&mid_blob) < 0, "compare(zero, mid) => negative, B=256");
        assert!(mid_blob.compare(&ones_blob) < 0, "compare(mid, ones) => negative, B=256");
        assert!(zero_blob.compare(&ones_blob) < 0, "compare(zero, ones) => negative, B=256");
        assert!(ones_blob.compare(&zero_blob) > 0, "compare(ones, zero) => positive, B=256");
        assert_eq!(mid_blob.compare(&mid_blob), 0, "compare(mid, mid) => 0, B=256");
    }

    #[traced_test]
    fn test_data_pointers() {
        info!("Testing pointer-based methods for B=8, B=64, B=256...");
        test_data_pointers_8();
        test_data_pointers_64();
        test_data_pointers_256();
        info!("data pointer tests concluded successfully.");
    }

    fn test_data_pointers_8() {
        let mut blob = BaseBlob8::default();
        let ptr_const = blob.data();
        assert!(!ptr_const.is_null(), "data() => pointer not null, B=8");
        let ptr_mut = blob.data_mut();
        assert!(!ptr_mut.is_null(), "data_mut() => pointer not null, B=8");
        let begin_const = blob.begin();
        let end_const = blob.end();
        let diff_const = (end_const as usize) - (begin_const as usize);
        assert_eq!(diff_const, 1, "end() - begin() => 1, B=8");
        let begin_mut = blob.begin_mut();
        let end_mut = blob.end_mut();
        let diff_mut = (end_mut as usize) - (begin_mut as usize);
        assert_eq!(diff_mut, 1, "end_mut() - begin_mut() => 1, B=8");
    }
    fn test_data_pointers_64() {
        let mut blob = BaseBlob64::default();
        let ptr_const = blob.data();
        assert!(!ptr_const.is_null(), "data() => pointer not null, B=64");
        let ptr_mut = blob.data_mut();
        assert!(!ptr_mut.is_null(), "data_mut() => pointer not null, B=64");
        let begin_const = blob.begin();
        let end_const = blob.end();
        let diff_const = (end_const as usize) - (begin_const as usize);
        assert_eq!(diff_const, 8, "end() - begin() => 8, B=64");
        let begin_mut = blob.begin_mut();
        let end_mut = blob.end_mut();
        let diff_mut = (end_mut as usize) - (begin_mut as usize);
        assert_eq!(diff_mut, 8, "end_mut() - begin_mut() => 8, B=64");
    }
    fn test_data_pointers_256() {
        let mut blob = BaseBlob256::default();
        let ptr_const = blob.data();
        assert!(!ptr_const.is_null(), "data() => pointer not null, B=256");
        let ptr_mut = blob.data_mut();
        assert!(!ptr_mut.is_null(), "data_mut() => pointer not null, B=256");
        let begin_const = blob.begin();
        let end_const = blob.end();
        let diff_const = (end_const as usize) - (begin_const as usize);
        assert_eq!(diff_const, 32, "end() - begin() => 32, B=256");
        let begin_mut = blob.begin_mut();
        let end_mut = blob.end_mut();
        let diff_mut = (end_mut as usize) - (begin_mut as usize);
        assert_eq!(diff_mut, 32, "end_mut() - begin_mut() => 32, B=256");
    }

    #[traced_test]
    fn test_size() {
        info!("Testing size() for B=8, B=64, B=256...");
        test_size_8();
        test_size_64();
        test_size_256();
        info!("size() tests concluded successfully.");
    }

    fn test_size_8() {
        let blob = BaseBlob8::default();
        let got_sz = blob.size();
        assert_eq!(
            got_sz, 1,
            "size() => 1 for B=8"
        );
    }
    fn test_size_64() {
        let blob = BaseBlob64::default();
        let got_sz = blob.size();
        assert_eq!(
            got_sz, 8,
            "size() => 8 for B=64"
        );
    }
    fn test_size_256() {
        let blob = BaseBlob256::default();
        let got_sz = blob.size();
        assert_eq!(
            got_sz, 32,
            "size() => 32 for B=256"
        );
    }
}
