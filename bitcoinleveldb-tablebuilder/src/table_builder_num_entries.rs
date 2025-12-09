// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder_num_entries.rs ]
crate::ix!();

impl TableBuilder {

    /// Number of calls to Add() so far.
    /// 
    pub fn num_entries(&self) -> u64 {
        unsafe {
            let rep_ptr = self.rep_ptr();
            assert!(
                !rep_ptr.is_null(),
                "TableBuilder::num_entries: rep pointer is null"
            );
            let r: &TableBuilderRep = &*rep_ptr;
            let n = *r.num_entries();
            if n < 0 {
                0
            } else {
                n as u64
            }
        }
    }
}

#[cfg(test)]
mod table_builder_num_entries_tests {
    use super::*;

    #[traced_test]
    fn num_entries_on_new_builder_is_zero() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("num_entries_on_new_builder_is_zero");

        trace!("num_entries_on_new_builder_is_zero: checking num_entries");
        assert_eq!(
            builder.num_entries(),
            0,
            "new builder must report num_entries == 0"
        );

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }

    #[traced_test]
    fn num_entries_tracks_add_calls() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("num_entries_tracks_add_calls");

        let key1   = Slice::from(b"k1".as_ref());
        let value1 = Slice::from(b"v1".as_ref());

        let key2   = Slice::from(b"k2".as_ref());
        let value2 = Slice::from(b"v2".as_ref());

        let key3   = Slice::from(b"k3".as_ref());
        let value3 = Slice::from(b"v3".as_ref());

        trace!("num_entries_tracks_add_calls: adding three entries");
        builder.add(&key1, &value1);
        builder.add(&key2, &value2);
        builder.add(&key3, &value3);

        assert_eq!(
            builder.num_entries(),
            3,
            "num_entries must equal number of Add calls"
        );

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }

    #[traced_test]
    fn num_entries_never_returns_negative_value() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("num_entries_never_returns_negative_value");

        unsafe {
            let rep_ptr = builder.rep_ptr() as *mut TableBuilderRep;
            let rep: &mut TableBuilderRep = &mut *rep_ptr;
            trace!("forcing negative num_entries inside rep");
            rep.set_num_entries(-5);
        }

        let reported = builder.num_entries();
        assert_eq!(
            reported, 0,
            "num_entries must clamp negative values to zero"
        );

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw as *mut InMemoryWritableFile);
        }
    }
}
