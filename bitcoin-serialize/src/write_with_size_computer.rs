// ---------------- [ File: bitcoin-serialize/src/write_with_size_computer.rs ]
crate::ix!();

#[inline] pub fn write_var_int_with_size_computer<I>(
        s: &mut SizeComputer,
        n: I)  {

    todo!();
        /*
            s.seek(GetSizeOfVarInt<I>(n));
        */
}

#[inline] pub fn write_compact_size_with_size_computer(
        s:      &mut SizeComputer,
        n_size: u64)  {
    
    todo!();
        /*
            s.seek(GetSizeOfCompactSize(nSize));
        */
}
