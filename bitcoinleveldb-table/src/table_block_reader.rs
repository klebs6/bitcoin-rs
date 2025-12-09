// ---------------- [ File: bitcoinleveldb-table/src/table_block_reader.rs ]
crate::ix!();

impl Table {

    /// Convert an index iterator value (i.e., an encoded BlockHandle) into an
    /// iterator over the contents of the corresponding block.
    ///
    /// This follows the original LevelDB C++ Table::BlockReader semantics
    /// exactly, including cache interactions and cleanup registration on the
    /// returned iterator.
    ///
    pub fn block_reader(
        arg:         *mut c_void,
        options:     &ReadOptions,
        index_value: &Slice,
    ) -> Option<Box<dyn LevelDBIteratorInterface>> {
        unsafe {
            assert!(
                !arg.is_null(),
                "Table::block_reader: arg pointer is null"
            );

            let table: &mut Table = &mut *(arg as *mut Table);

            let rep_ptr = table.rep_mut_ptr();
            assert!(
                !rep_ptr.is_null(),
                "Table::block_reader: table.rep pointer is null"
            );

            let rep: &mut TableRep = &mut *rep_ptr;

            // Cache pointer from options
            let block_cache_ref = rep.options().block_cache();
            let block_cache: *mut Cache = *block_cache_ref;

            let mut block: *mut Block = core::ptr::null_mut();
            let mut cache_handle: *mut CacheHandle = core::ptr::null_mut();

            let mut handle = BlockHandle::default();

            // Make a by-value copy of the Slice header (pointer + length) so we
            // can mutate it for decode_from without touching index_value itself.
            let input_data_ptr = *index_value.data();
            let input_data_len = *index_value.size();
            let mut input = Slice::from_ptr_len(input_data_ptr, input_data_len);

            trace!(
                "Table::block_reader: decoding BlockHandle from index_value (size={})",
                index_value.size()
            );

            let mut status = handle.decode_from(&mut input as *mut Slice);
            // We intentionally allow extra stuff in index_value so that we
            // can add more features in the future.

            if status.is_ok() {
                // Use the public constructor / default rather than
                // reaching into private fields.
                let mut contents = BlockContents::default();

                if !block_cache.is_null() {
                    let cache_ref: &mut Cache = &mut *block_cache;

                    trace!(
                        "Table::block_reader: block cache present; cache_id={}, handle_offset={}, handle_size={}",
                        rep.cache_id(),
                        handle.offset(),
                        handle.size()
                    );

                    // Cache key: 8 bytes of cache_id, followed by 8 bytes of block offset.
                    let mut cache_key_buf = [0u8; 16];

                    bitcoinleveldb_coding::encode_fixed64(
                        cache_key_buf.as_mut_ptr(),
                        *rep.cache_id(),
                    );
                    bitcoinleveldb_coding::encode_fixed64(
                        cache_key_buf.as_mut_ptr().add(8),
                        handle.offset(),
                    );

                    let key = Slice::from(&cache_key_buf[..]);

                    cache_handle = cache_ref.lookup(&key);

                    if !cache_handle.is_null() {
                        trace!(
                            "Table::block_reader: cache hit for offset={}",
                            handle.offset()
                        );

                        let value_ptr = cache_ref.value(cache_handle);
                        assert!(
                            !value_ptr.is_null(),
                            "Table::block_reader: cache value is null on cache hit"
                        );
                        block = value_ptr as *mut Block;
                    } else {
                        trace!(
                            "Table::block_reader: cache miss; reading block from file (offset={}, size={})",
                            handle.offset(),
                            handle.size()
                        );

                        status = read_block(
                            rep.file().clone(),
                            options,
                            &handle,
                            &mut contents as *mut BlockContents,
                        );

                        if status.is_ok() {
                            let block_box = Box::new(Block::new(&contents));
                            block = Box::into_raw(block_box);

                            if *contents.cachable() && *options.fill_cache() {
                                let block_size_bytes =
                                    (&*block).size() as usize;

                                trace!(
                                    "Table::block_reader: inserting block into cache; size={}",
                                    block_size_bytes
                                );

                                cache_handle = cache_ref.insert(
                                    &key,
                                    block as *mut c_void,
                                    block_size_bytes,
                                    delete_cached_block,
                                );
                            } else {
                                trace!(
                                    "Table::block_reader: block not cached (cachable={}, fill_cache={})",
                                    contents.cachable(),
                                    options.fill_cache()
                                );
                            }
                        } else {
                            error!(
                                "Table::block_reader: ReadBlock returned non-OK status (with cache)"
                            );
                        }
                    }
                } else {
                    trace!(
                        "Table::block_reader: block cache disabled; reading directly from file (offset={}, size={})",
                        handle.offset(),
                        handle.size()
                    );

                    status = read_block(
                        rep.file().clone(),
                        options,
                        &handle,
                        &mut contents as *mut BlockContents,
                    );

                    if status.is_ok() {
                        let block_box = Box::new(Block::new(&contents));
                        block = Box::into_raw(block_box);
                    } else {
                        error!(
                            "Table::block_reader: ReadBlock returned non-OK status (no cache)"
                        );
                    }
                }
            } else {
                debug!(
                    "Table::block_reader: failed to decode BlockHandle from index_value"
                );
            }

            if !block.is_null() {
                let cmp_arc: &Arc<dyn SliceComparator> =
                    rep.options().comparator();
                let cmp_ptr: *const dyn SliceComparator = &**cmp_arc;

                assert!(
                    !cmp_ptr.is_null(),
                    "Table::block_reader: comparator pointer is null"
                );

                let block_ref: &mut Block = &mut *block;

                trace!(
                    "Table::block_reader: creating data-block iterator for block @ {:?}",
                    block
                );

                let data_iter_raw: *mut LevelDBIterator =
                    block_ref.new_iterator(cmp_ptr);

                if cache_handle.is_null() {
                    trace!(
                        "Table::block_reader: registering DeleteBlock cleanup for uncached block"
                    );
                    (*data_iter_raw).register_cleanup(
                        delete_block,
                        block as *mut c_void,
                        core::ptr::null_mut(),
                    );
                } else {
                    trace!(
                        "Table::block_reader: registering ReleaseBlock cleanup for cached block (cache_handle={:?})",
                        cache_handle
                    );

                    (*data_iter_raw).register_cleanup(
                        release_block,
                        block_cache as *mut c_void,
                        cache_handle as *mut c_void,
                    );
                }

                let data_iter_box: Box<LevelDBIterator> =
                    Box::from_raw(data_iter_raw);
                let boxed_interface: Box<dyn LevelDBIteratorInterface> =
                    data_iter_box;

                Some(boxed_interface)
            } else {
                trace!(
                    "Table::block_reader: no block available; returning error iterator (status_ok={})",
                    status.is_ok()
                );
                let err_raw: *mut LevelDBIterator =
                    bitcoinleveldb_erroriterator::new_error_iterator(&status);
                let err_box: Box<LevelDBIterator> = Box::from_raw(err_raw);
                let boxed_interface: Box<dyn LevelDBIteratorInterface> =
                    err_box;
                Some(boxed_interface)
            }
        }
    }
}

#[cfg(test)]
mod table_block_reader_invariants {
    use super::*;

    #[test]
    #[should_panic(expected = "Table::block_reader: arg pointer is null")]
    fn block_reader_panics_when_arg_pointer_is_null() {
        let options = ReadOptions::default();
        let index_value = Slice::default();

        trace!(
            "block_reader_panics_when_arg_pointer_is_null: calling with arg=null"
        );
        unsafe {
            let _ = Table::block_reader(std::ptr::null_mut(), &options, &index_value);
        }
    }

    #[test]
    #[should_panic(expected = "Table::block_reader: table.rep pointer is null")]
    fn block_reader_panics_when_table_has_null_rep_pointer() {
        let mut table = Table::new(std::ptr::null_mut());
        let options = ReadOptions::default();
        let index_value = Slice::default();

        let table_ptr: *mut Table = &mut table;

        trace!(
            "block_reader_panics_when_table_has_null_rep_pointer: calling with table_ptr={:?}",
            table_ptr
        );

        unsafe {
            let _ = Table::block_reader(table_ptr as *mut c_void, &options, &index_value);
        }
    }
}

