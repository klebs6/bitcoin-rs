// ---------------- [ File: bitcoinleveldb-harness/src/test.rs ]
crate::ix!();

impl Harness {

    pub fn test(&mut self, rnd: *mut Random) {
        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.harness.test.entry",
        );

        const TAG_MASK: usize = BITCOINLEVELDB_HARNESS_CONSTRUCTOR_PTR_TAG_MASK;

        let tagged: usize = self.constructor as usize;
        let tag: usize = tagged & TAG_MASK;
        let raw: *mut Constructor = (tagged & !TAG_MASK) as *mut Constructor;

        assert!(!raw.is_null());

        let mut keys: Vec<String> = Vec::new();
        let mut data: KVMap = KVMap::default();

        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.harness.test.finish.begin",
            constructor_tag = tag,
            constructor_tag_label = bitcoinleveldb_harness_constructor_tag_machine_label(tag),
        );

        unsafe {
            (&mut *raw).finish(
                &self.options,
                &mut keys as *mut Vec<String>,
                &mut data as *mut KVMap,
            );
        }

        let finish_impl_status: Status = unsafe {
            match tag {
                0 => (&mut *(raw as *mut TableConstructor)).finish_impl(&self.options, &data),
                1 => (&mut *(raw as *mut BlockConstructor)).finish_impl(&self.options, &data),
                2 => (&mut *(raw as *mut MemTableConstructor)).finish_impl(&self.options, &data),
                3 => (&mut *(raw as *mut DBConstructor)).finish_impl(&self.options, &data),
                _ => {
                    panic!();
                }
            }
        };

        let finish_impl_ok: bool = finish_impl_status.is_ok();

        assert!(finish_impl_ok);

        let comparator: &Arc<dyn SliceComparator> = self.options.comparator();
        keys.sort_by(|a, b| {
            let a_slice: Slice = Slice::from(a.as_bytes());
            let b_slice: Slice = Slice::from(b.as_bytes());
            let cmp: i32 = comparator.compare(&a_slice, &b_slice);
            cmp.cmp(&0)
        });

        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.harness.test.finish.end",
            constructor_tag = tag,
            constructor_tag_label = bitcoinleveldb_harness_constructor_tag_machine_label(tag),
            num_keys = keys.len(),
            finish_impl_ok = finish_impl_ok,
        );

        self.test_forward_scan(&keys, &data);
        self.test_backward_scan(&keys, &data);
        self.test_random_access(rnd, &keys, &data);

        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.harness.test.exit",
            num_keys = keys.len(),
        );
    }
}
