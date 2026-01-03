// ---------------- [ File: bitcoinsecp256k1-scratch/src/scratch.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/scratch.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/scratch_impl.h]

/// Opaque data structure that holds rewriteable "scratch space"
/// 
/// The purpose of this structure is to replace dynamic memory allocations, because we target
/// architectures where this may not be available. 
///
/// It is essentially a resizable (within specified parameters) block of bytes, which is initially
/// created either by memory allocation or TODO as a pointer into some fixed rewritable space.
/// 
/// Unlike the context object, this cannot safely be shared between threads without additional
/// synchronization logic.
/// 
#[repr(C)]
pub struct Scratch {
    /// guard against interpreting this object as other types
    /// 
    pub(crate) magic: [u8; 8],
    /// actual allocated data
    /// 
    pub(crate) data: *mut c_void,
    /// amount that has been allocated (i.e. `data + offset` is the next available pointer)
    /// 
    pub(crate) alloc_size: usize,
    /// maximum size available to allocate
    /// 
    pub(crate) max_size: usize,
}

#[cfg(test)]
mod scratch_layout_and_alignment_invariant_test_suite {
    use super::*;

    #[traced_test]
    fn scratch_alignment_is_compatible_with_alignment_constant() {
        let a_scratch = core::mem::align_of::<Scratch>();
        debug!(
            target: "bitcoinsecp256k1_scratch::tests::scratch",
            a_scratch,
            alignment = ALIGNMENT,
            "checking Scratch alignment vs ALIGNMENT"
        );

        assert!(
            a_scratch <= ALIGNMENT,
            "Scratch alignment must not exceed ALIGNMENT for the allocation strategy"
        );
    }
}
