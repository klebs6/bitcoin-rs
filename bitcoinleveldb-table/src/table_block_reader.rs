// ---------------- [ File: bitcoinleveldb-table/src/table_block_reader.rs ]
crate::ix!();

impl Table {

    /**
      | Convert an index iterator value (i.e.,
      | an encoded BlockHandle) into an iterator
      | over the contents of the corresponding
      | block.
      |
      */
    pub fn block_reader(
        arg:         *mut c_void,
        options:     &ReadOptions,
        index_value: &Slice,
    ) -> *mut LevelDBIterator {
        unsafe {
            assert!(
                !arg.is_null(),
                "Table::block_reader: arg pointer is null"
            );

            let table = &mut *(arg as *mut Table);

            let rep_ptr = table.rep_mut_ptr();
            assert!(
                !rep_ptr.is_null(),
                "Table::block_reader: table.rep pointer is null"
            );

            let rep = &mut *rep_ptr;
            let block_cache: *mut Cache = rep.options().block_cache();

            let mut block: *mut Block = core::ptr::null_mut();
            let mut cache_handle: *mut CacheHandle = core::ptr::null_mut();

            let mut handle = BlockHandle::default();
            let mut input = *index_value;

            trace!(
                "Table::block_reader: decoding BlockHandle from index_value (size={})",
                index_value.size()
            );

            let mut status = handle.decode_from(&mut input as *mut Slice);
            // We intentionally allow extra stuff in index_value so that we
            // can add more features in the future.

            if status.is_ok() {
                let mut contents = BlockContents {
                    data:           Slice::default(),
                    cachable:       false,
                    heap_allocated: false,
                };

                if !block_cache.is_null() {
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
                    let cache_ref = &mut *block_cache;

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
                                    delete_cached_block as CacheDeleterFn,
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

            let iter: *mut LevelDBIterator;

            if !block.is_null() {
                let cmp_ptr = rep.options().comparator();
                assert!(
                    !cmp_ptr.is_null(),
                    "Table::block_reader: comparator pointer is null"
                );

                let cmp = &*cmp_ptr;
                let block_ref = &mut *block;

                trace!(
                    "Table::block_reader: creating data-block iterator for block @ {:?}",
                    block
                );

                iter = block_ref.new_iterator(cmp);

                if cache_handle.is_null() {
                    trace!(
                        "Table::block_reader: registering DeleteBlock cleanup for uncached block"
                    );
                    (*iter).register_cleanup(
                        delete_block as LevelDBIteratorCleanupFunction,
                        block as *mut c_void,
                        core::ptr::null_mut(),
                    );
                } else {
                    trace!(
                        "Table::block_reader: registering ReleaseBlock cleanup for cached block (cache_handle={:?})",
                        cache_handle
                    );
                    (*iter).register_cleanup(
                        release_block as LevelDBIteratorCleanupFunction,
                        block_cache as *mut c_void,
                        cache_handle as *mut c_void,
                    );
                }
            } else {
                trace!(
                    "Table::block_reader: no block available; returning error iterator (status_ok={})",
                    status.is_ok()
                );
                iter = bitcoinleveldb_erroriterator::new_error_iterator(&status);
            }

            iter
        }
    }
}
