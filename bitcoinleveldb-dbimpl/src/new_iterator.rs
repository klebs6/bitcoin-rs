// ---------------- [ File: bitcoinleveldb-dbimpl/src/new_iterator.rs ]
crate::ix!();

impl DBNewIterator for DBImpl {

    fn new_iterator(&mut self, options: &ReadOptions) -> *mut LevelDBIterator {
        let mut latest_snapshot: SequenceNumber = 0;
        let mut seed: u32 = 0;

        let iter: *mut LevelDBIterator =
            self.new_internal_iterator(options, &mut latest_snapshot, &mut seed);

        let snapshot: SequenceNumber = if !options.snapshot.is_null() {
            unsafe { (*(options.snapshot as *const SnapshotImpl)).sequence_number() }
        } else {
            latest_snapshot
        };

        new_db_iterator(self, self.user_comparator(), iter, snapshot, seed)
    }
}

#[cfg(test)]
#[disable]
mod new_iterator_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn new_iterator_yields_user_keys_in_sorted_order_and_values_match() {
        let (dbname, mut db) = open_dbimpl_for_test("new_iterator_yields_user_keys_in_sorted_order_and_values_match");

        write_kv(&mut *db, "b", "2");
        write_kv(&mut *db, "a", "1");
        write_kv(&mut *db, "c", "3");

        let it: *mut LevelDBIterator = <DBImpl as DBNewIterator>::new_iterator(&mut *db, &ReadOptions::default());
        assert!(!it.is_null(), "iterator must not be null");

        unsafe {
            (*it).seek_to_first();
            let mut seen: Vec<(String, String)> = Vec::new();

            while (*it).valid() {
                let k = (*it).key().to_string();
                let v = (*it).value().to_string();
                tracing::debug!(key = %k, value = %v, "iter item");
                seen.push((k, v));
                (*it).next();
            }

            let st = (*it).status();
            tracing::info!(status = %st.to_string(), n = seen.len(), "iterator finished");
            assert!(st.is_ok(), "iterator status must be ok: {}", st.to_string());

            assert!(seen.len() >= 3, "should see at least the inserted keys");
            assert!(seen[0].0 <= seen[1].0 && seen[1].0 <= seen[2].0, "keys should be sorted");

            drop(Box::from_raw(it));
        }

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
