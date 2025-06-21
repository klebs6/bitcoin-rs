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
    /// length actually written.
    fn bytes_written<T: BtcSerialize<Cursor<Vec<u8>>>>(value: &T) -> usize {
        let mut cur = Cursor::new(Vec::<u8>::new());
        BtcSerialize::serialize(value, &mut cur);
        cur.get_ref().len()
    }

    #[traced_test]
    fn scalar_size_matches_actual() {
        let sample_u32: u32 = 0x12_34_56_78;
        assert_eq!(
            get_serialize_size(&sample_u32, None),
            bytes_written(&sample_u32)
        );

        let sample_bool = true;
        assert_eq!(
            get_serialize_size(&sample_bool, Some(123)),
            bytes_written(&sample_bool)
        );
    }

    /// Validate the *tuple* helper.
    #[traced_test]
    fn tuple_size_matches_manual_sum() {
        let tpl = (0xAAu8, 0xBEEF_u16, false);

        // manual concatenation
        let mut buf = Cursor::new(Vec::<u8>::new());
        BtcSerialize::serialize(&tpl.0, &mut buf);
        BtcSerialize::serialize(&tpl.1, &mut buf);
        BtcSerialize::serialize(&tpl.2, &mut buf);
        let manual = buf.get_ref().len();

        let via_helper = crate::get_serialize_size_many(0, &tpl);
        assert_eq!(manual, via_helper);
    }
}
