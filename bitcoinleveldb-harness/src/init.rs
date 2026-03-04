// ---------------- [ File: bitcoinleveldb-harness/src/init.rs ]
crate::ix!();

impl Harness {

    pub fn init(&mut self, args: &TestArgs) {
        /*
            delete constructor_;
        constructor_ = nullptr;
        options_ = Options();

        options_.block_restart_interval = args.restart_interval;
        // Use shorter block size for tests to exercise block boundary
        // conditions more.
        options_.block_size = 256;
        if (args.reverse_compare) {
          options_.comparator = &reverse_key_comparator;
        }
        switch (args.type) {
          case TABLE_TEST:
            constructor_ = new TableConstructor(options_.comparator);
            break;
          case BLOCK_TEST:
            constructor_ = new BlockConstructor(options_.comparator);
            break;
          case MEMTABLE_TEST:
            constructor_ = new MemTableConstructor(options_.comparator);
            break;
          case DB_TEST:
            constructor_ = new DBConstructor(options_.comparator);
            break;
        }
        */
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
            let reverse_cmp: Arc<dyn Comparator> = bitcoinleveldb_harness_reverse_bytewise_comparator();
            self.options.set_comparator(reverse_cmp);
            debug!(
                target: "bitcoinleveldb_harness",
                label = "bitcoinleveldb_harness.harness.init.reverse_comparator.enabled",
                comparator_name = self.options.comparator().name(),
            );
        } else {
            debug!(
                target: "bitcoinleveldb_harness",
                label = "bitcoinleveldb_harness.harness.init.reverse_comparator.disabled",
                comparator_name = self.options.comparator().name(),
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
        );
    }
}
