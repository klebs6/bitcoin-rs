// ---------------- [ File: bitcoinleveldb-memtable/src/memtable_constructor.rs ]
crate::ix!();

#[repr(C)]
pub struct MemTableConstructor  {
    base:                Constructor,
    user_comparator:     std::sync::Arc<Box<dyn SliceComparator>>,
    internal_comparator: InternalKeyComparator,
    memtable:            *mut MemTable,
}

impl Drop for MemTableConstructor {
    fn drop(&mut self) {
        trace!(
            "MemTableConstructor::drop: memtable_ptr={:?}",
            self.memtable
        );
        unsafe {
            if !self.memtable.is_null() {
                trace!(
                    "MemTableConstructor::drop: calling MemTable::unref()"
                );
                (*self.memtable).unref();
                self.memtable = core::ptr::null_mut();
            }
        }
    }
}

impl MemTableConstructor {
    pub fn new(
        cmp: Box<dyn SliceComparator>,
    ) -> Self {
        trace!(
            "MemTableConstructor::new: cmp_ptr={:p}",
            &*cmp as *const dyn SliceComparator
        );

        let user_comparator = std::sync::Arc::new(cmp);
        let base = Constructor::with_comparator_owner(user_comparator.clone());

        // Hold the user comparator so that the pointer
        // embedded in InternalKeyComparator remains valid.

        let user_cmp_ptr: *const dyn SliceComparator =
            &**user_comparator;

        let internal_comparator =
            InternalKeyComparator::new(
                user_cmp_ptr,
            );

        // Construct first MemTable instance.
        let memtable_val =
            MemTable::new(&internal_comparator);
        let mut memtable_box =
            Box::new(memtable_val);
        memtable_box.ref_();
        let memtable_ptr =
            Box::into_raw(memtable_box);

        MemTableConstructor {
            base,
            user_comparator,
            internal_comparator,
            memtable: memtable_ptr,
        }
    }

    pub fn finish_impl(&mut self, options: &Options, data: &KVMap) -> Status {
        self.finish_impl_bytes(options, &constructor_bytes_from_text(data))
    }
    pub fn finish_impl_bytes(&mut self, options: &Options, data: &ConstructorByteMap) -> Status {
        if self.user_comparator.name() != options.comparator().name() {
            return Status::invalid_argument(&Slice::from("memtable constructor comparator mismatch"), None);
        }
        let entries = match constructor_ordered_byte_entries(data, &**self.user_comparator) {
            Ok(entries) => entries, Err(status) => return status,
        };
        trace!(
            "MemTableConstructor::finish_impl: entries={}, memtable_ptr={:?}",
            data.len(),
            self.memtable
        );

        unsafe {
            if !self.memtable.is_null() {
                trace!(
                    "MemTableConstructor::finish_impl: Unref() old memtable @ {:?}",
                    self.memtable
                );
                (*self.memtable).unref();
            }
        }

        let internal =
            &self.internal_comparator;

        let mem_val =
            MemTable::new(internal);
        let mut mem_box =
            Box::new(mem_val);
        mem_box.ref_();
        let mem_ptr = Box::into_raw(mem_box);
        self.memtable = mem_ptr;

        // Fill the new memtable.
        let mut seq: SequenceNumber = 1;
        for (k, v) in entries {
            let key_bytes = k.as_slice();
            let val_bytes = v.as_slice();

            let key_slice =
                Slice::from(key_bytes);
            let val_slice =
                Slice::from(val_bytes);

            trace!(
                "MemTableConstructor::finish_impl: inserting seq={} key bytes={:?}",
                seq,
                k
            );

            unsafe {
                (*self.memtable).add(
                    seq,
                    ValueType::TypeValue,
                    &key_slice,
                    &val_slice,
                );
            }

            seq += 1;
        }

        Status::ok()
    }

    pub fn new_iterator(&self) -> *mut LevelDBIterator {
        trace!(
            "MemTableConstructor::new_iterator: memtable_ptr={:?}",
            self.memtable
        );

        unsafe {
            assert!(
                !self.memtable.is_null(),
                "MemTableConstructor::new_iterator: memtable pointer is null"
            );

            let internal_raw: *mut LevelDBIterator =
                (*self.memtable).new_iterator();

            trace!(
                "MemTableConstructor::new_iterator: internal iterator @ {:?}",
                internal_raw
            );

            let key_conv =
                KeyConvertingIterator::new(
                    internal_raw,
                );
            let iface: Box<
                dyn LevelDBIteratorInterface,
            > = Box::new(key_conv);

            let wrapper =
                LevelDBIterator::new(Some(iface));
            let boxed_wrapper =
                Box::new(wrapper);
            let raw_wrapper =
                Box::into_raw(boxed_wrapper);

            trace!(
                "MemTableConstructor::new_iterator: returning iterator @ {:?}",
                raw_wrapper
            );

            raw_wrapper
        }
    }
}
