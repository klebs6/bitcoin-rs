// ---------------- [ File: bitcoin-serialize/src/get_serialize_size.rs ]
crate::ix!();

pub fn get_serialize_size<T>(
        t:         &T,
        n_version: Option<i32>) -> usize {

    let n_version: i32 = n_version.unwrap_or(0);
    (SizeComputer::new(n_version) << &t).size()
}

pub fn get_serialize_size_many<T>(
        n_version: i32,
        t:         &T) -> usize {

    todo!();
        /*
            CSizeComputer sc(nVersion);
        SerializeMany(sc, t...);
        return sc.size();
        */
}
