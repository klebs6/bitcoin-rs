// ---------------- [ File: bitcoin-serialize/src/get_serialize_size.rs ]
crate::ix!();

pub fn get_serialize_size<T>(t: &T, n_version: Option<i32>) -> usize 
where T: BtcSerialize<crate::size_computer::SizeComputer>
{
    let n_version: i32 = n_version.unwrap_or(0);
    (SizeComputer::new(n_version) << t).size()
}

/// Compute the combined serialised size of an arbitrary tuple of values.
///
/// ```ignore
/// let sz = get_serialize_size_many(0, &(header, tx_count, &txs[..]));
/// ```
pub fn get_serialize_size_many<Args>(n_version: i32, args: &Args) -> usize
where
    Args: SerializeMany<SizeComputer>,
{
    let mut sc = SizeComputer::new(n_version);
    args.serialize_many(&mut sc);
    sc.size()
}

#[cfg(test)]
mod get_serialize_size_tests {
    use super::*;
    use std::io::Cursor;

    /// Helper: serialise `value` into an in‑memory buffer and return the
    /// number of bytes written.
    fn actual_size<T: BtcSerialize<Cursor<Vec<u8>>>>(value: &T) -> usize {
        let mut buf = Cursor::new(Vec::<u8>::new());
        value.serialize(&mut buf);
        buf.get_ref().len()
    }

    #[traced_test]
    fn scalar_size_matches_actual_bytes() {
        let sample_u32: u32 = 0x12_34_56_78;
        assert_eq!(
            get_serialize_size(&sample_u32, None),
            actual_size(&sample_u32)
        );

        let sample_bool = true;
        assert_eq!(
            get_serialize_size(&sample_bool, Some(42)), // arbitrary version
            actual_size(&sample_bool)
        );
    }

    /// Validate the *tuple* helper (`get_serialize_size_many`) for up to
    /// three heterogeneous elements.
    #[traced_test]
    fn tuple_size_matches_manual_concatenation() {
        let tpl = (0xABu8, 0xCDEFu16, false);

        let mut buf = Cursor::new(Vec::<u8>::new());
        tpl.0.serialize(&mut buf);
        tpl.1.serialize(&mut buf);
        tpl.2.serialize(&mut buf);
        let manual_len = buf.get_ref().len();

        let computed_len = get_serialize_size_many(0, &tpl);
        assert_eq!(manual_len, computed_len);
    }
}
