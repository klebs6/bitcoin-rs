// ---------------- [ File: bitcoin-blob/src/from_bytes.rs ]
crate::ix!();

#[macro_export]
macro_rules! define_base_blob_from_bytes {

    ($blob_ty:ident, $bits:expr, $bytes:expr) => {

        impl From<u8> for $blob_ty {
            fn from(v: u8) -> Self {
                debug!(
                    "Constructing BaseBlob<{}> from u8=0x{:02X}; only the first byte is set to v",
                    $bits,
                    v
                );
                let mut out = Self::default();
                if $bytes > 0 {
                    out.data[0] = v;
                }
                out
            }
        }

        impl From<&Vec<u8>> for $blob_ty {
            fn from(vch: &Vec<u8>) -> Self {
                debug!(
                    "Constructing BaseBlob<{}> from &Vec<u8> of length={}",
                    $bits,
                    vch.len()
                );
                assert_eq!(
                    vch.len(),
                    $bytes,
                    "Input Vec<u8> must match base_blob_width for BITS={}",
                    $bits
                );
                let mut out = Self::default();
                out.data.copy_from_slice(&vch[..]);
                out
            }
        }
    }
}

#[cfg(test)]
mod from_bytes_exhaustive_tests {
    use super::*;
    use tracing::{info, debug};

    /// Test `From<u8>` => we confirm only the first byte is set, for B=8, B=64, B=256.
    #[traced_test]
    fn test_from_u8() {
        info!("Testing From<u8> => BaseBlob8, BaseBlob64, BaseBlob256...");

        // B=8
        {
            let test_values = [0u8, 1, 127, 128, 255];
            for &val in test_values.iter() {
                let blob = BaseBlob8::from(val);
                assert_eq!(blob.data.len(), 1);
                assert_eq!(blob.data[0], val, "First byte must match val, B=8");
                if val == 0 {
                    assert!(blob.is_null(), "If 0 => is_null()=true, B=8");
                } else {
                    assert!(!blob.is_null(), "If !=0 => is_null()=false, B=8");
                }
            }
        }

        // B=64
        {
            let test_values = [0u8, 1, 127, 128, 255];
            for &val in test_values.iter() {
                let blob = BaseBlob64::from(val);
                // only data[0] is set to val, rest are zero
                assert_eq!(blob.data[0], val, "BaseBlob64 => data[0]=val");
                for i in 1..8 {
                    assert_eq!(blob.data[i], 0, "others=0, B=64");
                }
                if val == 0 {
                    assert!(blob.is_null(), "If 0 => is_null()=true, B=64");
                } else {
                    assert!(!blob.is_null(), "If !=0 => is_null()=false, B=64");
                }
            }
        }

        // B=256
        {
            let test_values = [0u8, 1, 127, 128, 255];
            for &val in test_values.iter() {
                let blob = BaseBlob256::from(val);
                // only data[0] is set, rest zero
                assert_eq!(blob.data[0], val, "BaseBlob256 => data[0]=val");
                for i in 1..32 {
                    assert_eq!(blob.data[i], 0, "others=0, B=256");
                }
                if val == 0 {
                    assert!(blob.is_null(), "If 0 => is_null()=true, B=256");
                } else {
                    assert!(!blob.is_null(), "If !=0 => is_null()=false, B=256");
                }
            }
        }

        info!("From<u8> tests concluded successfully.");
    }

    /// Test `From<&Vec<u8>>` => must exactly match the blob's size. If length is wrong => panic.
    #[traced_test]
    fn test_from_vec_u8() {
        info!("Testing From<&Vec<u8>> => BaseBlob8, BaseBlob64, BaseBlob256...");

        // B=8 => length must be 1
        {
            let correct = vec![0xAA];
            let blob_good = BaseBlob8::from(&correct);
            assert_eq!(blob_good.data[0], 0xAA, "B=8 => single byte copy");
            // short or long => panic
            {
                let short_vec = vec![];
                let caught = std::panic::catch_unwind(|| {
                    let _ = BaseBlob8::from(&short_vec);
                });
                assert!(caught.is_err(), "Short Vec => panic, B=8");
            }
            {
                let long_vec = vec![0x11, 0x22];
                let caught = std::panic::catch_unwind(|| {
                    let _ = BaseBlob8::from(&long_vec);
                });
                assert!(caught.is_err(), "Long Vec => panic, B=8");
            }
        }

        // B=64 => length must be 8
        {
            let mut correct = vec![0u8; 8];
            for (i,b) in correct.iter_mut().enumerate() {
                *b = i as u8;
            }
            let blob_good = BaseBlob64::from(&correct);
            assert_eq!(blob_good.data, correct[..], "B=64 => copied 8 bytes");
            // short
            {
                let short_vec = vec![1,2,3,4,5,6,7];
                let caught = std::panic::catch_unwind(|| {
                    let _ = BaseBlob64::from(&short_vec);
                });
                assert!(caught.is_err(), "Short => panic, B=64");
            }
            // long
            {
                let long_vec = vec![0u8; 9];
                let caught = std::panic::catch_unwind(|| {
                    let _ = BaseBlob64::from(&long_vec);
                });
                assert!(caught.is_err(), "Long => panic, B=64");
            }
        }

        // B=256 => length must be 32
        {
            let mut correct = vec![0u8; 32];
            for (i,b) in correct.iter_mut().enumerate() {
                *b = (i as u8).wrapping_mul(7);
            }
            let blob_good = BaseBlob256::from(&correct);
            assert_eq!(blob_good.data, correct[..], "B=256 => copied 32 bytes");
            // short
            {
                let short_vec = vec![0u8; 31];
                let caught = std::panic::catch_unwind(|| {
                    let _ = BaseBlob256::from(&short_vec);
                });
                assert!(caught.is_err(), "Short => panic, B=256");
            }
            // long
            {
                let long_vec = vec![0u8; 33];
                let caught = std::panic::catch_unwind(|| {
                    let _ = BaseBlob256::from(&long_vec);
                });
                assert!(caught.is_err(), "Long => panic, B=256");
            }
        }

        info!("From<&Vec<u8>> tests concluded successfully.");
    }
}
