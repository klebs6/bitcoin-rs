// ---------------- [ File: bitcoinleveldb-batch/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

x!{delete}
x!{handler}
x!{internal}
x!{iterate}
x!{mem_table_inserter}
x!{put}
x!{write_batch}
x!{test_util}

#[cfg(test)]
mod lib_rs_exhaustive_contract_suite {
    use super::*;
    use write_batch_test_harness_utilities::*;

    #[traced_test]
    fn crate_exports_allow_end_to_end_write_batch_usage_through_public_api() {
        tracing::trace!("crate_exports_allow_end_to_end_write_batch_usage_through_public_api: begin");

        assert_eq!(HEADER, 12);

        let mut batch = WriteBatch::new();
        batch.put(
            &bitcoinleveldb_slice::Slice::from("k"),
            &bitcoinleveldb_slice::Slice::from("v"),
        );
        batch.delete(&bitcoinleveldb_slice::Slice::from("x"));

        write_batch_internal::set_sequence(
            &mut batch as *mut WriteBatch,
            42,
        );

        let state = format_memtable_state_for_batch(
            &mut batch as *mut WriteBatch,
        );

        // Keys are ordered by memtable comparator: "k" then "x".
        // Sequences assigned in record order: Put(k)@42, Delete(x)@43.
        assert_eq!(state, "Put(k, v)@42Delete(x)@43");

        tracing::trace!("crate_exports_allow_end_to_end_write_batch_usage_through_public_api: end");
    }
}
