// ---------------- [ File: bitcoin-serialize/src/get_serialize_size.rs ]
crate::ix!();

pub fn get_serialize_size<T>(t: &T, n_version: Option<i32>) -> usize 
where T: crate::serialize::Serialize<crate::size_computer::SizeComputer>
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
