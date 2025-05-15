crate::ix!();

impl<const BITS: usize> From<u8> for BaseBlob<BITS>
where
    [u8; (BITS % 8) + usize::MAX]: ,
    [(); base_blob_width::<BITS>()]:
{
    /**
      | constructor for constants between
      | 1 and 255
      |
    */
    fn from(v: u8) -> Self {
        debug!(
            "Constructing BaseBlob<{}> from u8=0x{:02X}; only the first byte is set to v",
            BITS,
            v
        );

        let mut out = Self::default();
        out.data[0] = v;
        out
    }
}

impl<const BITS: usize> From<&Vec<u8>> for BaseBlob<BITS>
where
    [u8; (BITS % 8) + usize::MAX]: ,
    [(); base_blob_width::<BITS>()]:
{
    fn from(vch: &Vec<u8>) -> Self {
        debug!(
            "Constructing BaseBlob<{}> from &Vec<u8> of length={}",
            BITS,
            vch.len()
        );

        let expected_len = base_blob_width::<BITS>();
        assert_eq!(
            vch.len(),
            expected_len,
            "Input Vec<u8> must match base_blob_width for BITS={}",
            BITS
        );

        let mut out = Self::default();
        out.data.copy_from_slice(&vch[..]);
        out
    }
}
