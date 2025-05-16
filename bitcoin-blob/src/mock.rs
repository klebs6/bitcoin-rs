crate::ix!();

/// A helper that builds a `BaseBlob<B>` from a slice.  
/// Note that `B` is chosen at **compile time** via `make_blob::<128>(data)`.
pub fn make_blob<const B: usize>(data: &[u8]) -> BaseBlob<B>
where
    [u8; (B % 8) + usize::MAX]:,
    [(); base_blob_width::<B>()]:
{
    let mut blob = BaseBlob::<B>::default();
    assert_eq!(
        data.len(),
        base_blob_width::<B>(),
        "Input slice must match base_blob_width<B>"
    );
    blob.data.copy_from_slice(data);
    blob
}

// --------------------------------------------------------------------------------
// Helper for eq tests: compute the actual width in bytes for arbitrary bits.
pub fn base_blob_width_for(bits: usize) -> usize {
    bits / 8
}

pub const fn base_blob_width<const BITS: usize>() -> usize 
{
    BITS / 8
}
