// ---------------- [ File: bitcoinleveldb-harness/src/init.rs ]
crate::ix!();

impl Harness {

    pub fn init(&mut self, args: &TestArgs) {
        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.harness.init.entry",
            ty = bitcoinleveldb_harness_test_type_machine_label(&args.ty),
            reverse_compare = args.reverse_compare,
            restart_interval = args.restart_interval,
        );

        // delete constructor_;
        self.bitcoinleveldb_harness_dispose_constructor_pointer("init");

        // options_ = Options();
        self.options = Options::default();

        // options_.block_restart_interval = args.restart_interval;
        self.options.set_block_restart_interval(args.restart_interval);

        // Use shorter block size for tests to exercise block boundary
        // conditions more.
        // options_.block_size = 256;
        self.options.set_block_size(256);

        // if (args.reverse_compare) { options_.comparator = &reverse_key_comparator; }
        if args.reverse_compare {
            let reverse_cmp: Arc<dyn SliceComparator> =
                bitcoinleveldb_harness_reverse_bytewise_comparator();
            self.options.set_comparator(reverse_cmp);

            let comparator_name: Cow<'_, str> = self.options.comparator().name();
            debug!(
                target: "bitcoinleveldb_harness",
                label = "bitcoinleveldb_harness.harness.init.reverse_comparator.enabled",
                comparator_name = comparator_name.as_ref(),
            );
        } else {
            let comparator_name: Cow<'_, str> = self.options.comparator().name();
            debug!(
                target: "bitcoinleveldb_harness",
                label = "bitcoinleveldb_harness.harness.init.reverse_comparator.disabled",
                comparator_name = comparator_name.as_ref(),
            );
        }

        // switch (args.type) { ... }
        match args.ty {
            TestType::TABLE_TEST => {
                debug!(
                    target: "bitcoinleveldb_harness",
                    label = "bitcoinleveldb_harness.harness.init.constructor.select",
                    constructor_kind = "table",
                );

                let cmp_box: Box<dyn SliceComparator> =
                    bitcoinleveldb_harness_slice_comparator_box_from_options(&self.options);

                let ctor: Box<TableConstructor> = Box::new(TableConstructor::new(cmp_box));
                let raw_ctor: *mut TableConstructor = Box::into_raw(ctor);

                let base_ptr: *mut Constructor = raw_ctor as *mut Constructor;
                self.constructor = base_ptr;
            }

            TestType::BLOCK_TEST => {
                debug!(
                    target: "bitcoinleveldb_harness",
                    label = "bitcoinleveldb_harness.harness.init.constructor.select",
                    constructor_kind = "block",
                );

                let cmp_box: Box<dyn SliceComparator> =
                    bitcoinleveldb_harness_slice_comparator_box_from_options(&self.options);

                let ctor: Box<BlockConstructor> = Box::new(BlockConstructor::new(cmp_box));
                let raw_ctor: *mut BlockConstructor = Box::into_raw(ctor);

                let base_ptr: *mut Constructor = raw_ctor as *mut Constructor;
                self.constructor = ((base_ptr as usize) | 1usize) as *mut Constructor;
            }

            TestType::MEMTABLE_TEST => {
                debug!(
                    target: "bitcoinleveldb_harness",
                    label = "bitcoinleveldb_harness.harness.init.constructor.select",
                    constructor_kind = "memtable",
                );

                let cmp_box: Box<dyn SliceComparator> =
                    bitcoinleveldb_harness_slice_comparator_box_from_options(&self.options);

                let ctor: Box<MemTableConstructor> = Box::new(MemTableConstructor::new(cmp_box));
                let raw_ctor: *mut MemTableConstructor = Box::into_raw(ctor);

                let base_ptr: *mut Constructor = raw_ctor as *mut Constructor;
                self.constructor = ((base_ptr as usize) | 2usize) as *mut Constructor;
            }

            TestType::DB_TEST => {
                debug!(
                    target: "bitcoinleveldb_harness",
                    label = "bitcoinleveldb_harness.harness.init.constructor.select",
                    constructor_kind = "db",
                );

                trace!(
                    target: "bitcoinleveldb_harness",
                    label = "bitcoinleveldb_harness.harness.init.db_test_execution_guard.acquire.begin",
                );

                self.db_test_execution_guard =
                    Some(bitcoinleveldb_harness_acquire_db_test_execution_guard());

                trace!(
                    target: "bitcoinleveldb_harness",
                    label = "bitcoinleveldb_harness.harness.init.db_test_execution_guard.acquire.end",
                );

                let cmp_box: Box<dyn SliceComparator> =
                    bitcoinleveldb_harness_slice_comparator_box_from_options(&self.options);

                let ctor: Box<DBConstructor> = Box::new(DBConstructor::new(cmp_box));
                let raw_ctor: *mut DBConstructor = Box::into_raw(ctor);

                let base_ptr: *mut Constructor = raw_ctor as *mut Constructor;
                self.constructor = ((base_ptr as usize) | 3usize) as *mut Constructor;
            }
        }

        trace!(
            target: "bitcoinleveldb_harness",
            label = "bitcoinleveldb_harness.harness.init.exit",
            ty = bitcoinleveldb_harness_test_type_machine_label(&args.ty),
            constructor_tagged = (self.constructor as usize),
            db_test_execution_guard_held = self.db_test_execution_guard.is_some(),
        );
    }
}
