// ---------------- [ File: bitcoinleveldb-blockhandle/src/global_state.rs ]
crate::ix!();

/**
  | Invariant: a given shared RandomAccessFile
  | identity key is mapped to exactly one
  | process-local serialization lock for the
  | lifetime of the process.
  |
  */
pub type BitcoinLevelDbBlockHandleRandomAccessFileAccessRegistry =
    StdMutex<HashMap<usize, Arc<StdMutex<()>>>>;

/**
  | Invariant: the returned registry remains
  | process-global and stable for the entire
  | process lifetime.
  |
  */
pub fn bitcoinleveldb_blockhandle_random_access_file_access_registry(
) -> &'static BitcoinLevelDbBlockHandleRandomAccessFileAccessRegistry {
    static BITCOIN_LEVELDB_BLOCKHANDLE_RANDOM_ACCESS_FILE_ACCESS_REGISTRY:
        Lazy<BitcoinLevelDbBlockHandleRandomAccessFileAccessRegistry> =
            Lazy::new(|| {
                trace!(
                    target: "bitcoinleveldb_blockhandle::global_state",
                    label = "blockhandle_random_access_file_access_registry.initialize",
                );
                StdMutex::new(HashMap::new())
            });

    &BITCOIN_LEVELDB_BLOCKHANDLE_RANDOM_ACCESS_FILE_ACCESS_REGISTRY
}

/**
  | Invariant: the identity key depends only on
  | the stable allocation address of the shared
  | RandomAccessFile wrapper and therefore does
  | not change while the wrapper remains alive.
  |
  */
pub fn bitcoinleveldb_blockhandle_random_access_file_identity_key(
    file: &Rc<RefCell<dyn RandomAccessFile>>,
) -> usize {
    let file_ptr: *const RefCell<dyn RandomAccessFile> = Rc::as_ptr(file);
    let identity_key: usize = file_ptr as *const () as usize;

    trace!(
        target: "bitcoinleveldb_blockhandle::global_state",
        label = "blockhandle_random_access_file_identity_key",
        identity_key = identity_key,
    );

    identity_key
}

/**
  | Invariant: callers that request the lock for
  | the same shared RandomAccessFile identity key
  | receive the same lock instance.
  |
  */
pub fn bitcoinleveldb_blockhandle_random_access_file_access_lock(
    file: &Rc<RefCell<dyn RandomAccessFile>>,
) -> Arc<StdMutex<()>> {
    let identity_key =
        bitcoinleveldb_blockhandle_random_access_file_identity_key(file);

    trace!(
        target: "bitcoinleveldb_blockhandle::global_state",
        label = "blockhandle_random_access_file_access_lock.entry",
        identity_key = identity_key,
    );

    let registry =
        bitcoinleveldb_blockhandle_random_access_file_access_registry();

    let mut registry_guard = match registry.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            warn!(
                target: "bitcoinleveldb_blockhandle::global_state",
                label = "blockhandle_random_access_file_access_lock.registry_poisoned",
                identity_key = identity_key,
            );
            poisoned.into_inner()
        }
    };

    let existing_lock_opt =
        registry_guard.get(&identity_key).cloned();

    let access_lock = match existing_lock_opt {
        Some(existing_lock) => {
            trace!(
                target: "bitcoinleveldb_blockhandle::global_state",
                label = "blockhandle_random_access_file_access_lock.reuse_existing",
                identity_key = identity_key,
            );
            existing_lock
        }
        None => {
            let new_lock: Arc<StdMutex<()>> =
                Arc::new(StdMutex::new(()));

            registry_guard.insert(
                identity_key,
                new_lock.clone(),
            );

            trace!(
                target: "bitcoinleveldb_blockhandle::global_state",
                label = "blockhandle_random_access_file_access_lock.install_new",
                identity_key = identity_key,
            );

            new_lock
        }
    };

    trace!(
        target: "bitcoinleveldb_blockhandle::global_state",
        label = "blockhandle_random_access_file_access_lock.exit",
        identity_key = identity_key,
    );

    access_lock
}

/**
  | Invariant: the returned String is an owned
  | snapshot of the file name taken while file
  | access for that identity is serialized.
  |
  */
pub fn bitcoinleveldb_blockhandle_random_access_file_name(
    file: &Rc<RefCell<dyn RandomAccessFile>>,
) -> String {
    let identity_key =
        bitcoinleveldb_blockhandle_random_access_file_identity_key(file);

    let access_lock =
        bitcoinleveldb_blockhandle_random_access_file_access_lock(file);

    let _access_guard = match access_lock.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            warn!(
                target: "bitcoinleveldb_blockhandle::global_state",
                label = "blockhandle_random_access_file_name.lock_poisoned",
                identity_key = identity_key,
            );
            poisoned.into_inner()
        }
    };

    trace!(
        target: "bitcoinleveldb_blockhandle::global_state",
        label = "blockhandle_random_access_file_name.borrow_begin",
        identity_key = identity_key,
    );

    let file_ref = file.borrow();

    let file_name = match file_ref.name() {
        Cow::Borrowed(borrowed_name) => borrowed_name.to_owned(),
        Cow::Owned(owned_name) => owned_name,
    };

    trace!(
        target: "bitcoinleveldb_blockhandle::global_state",
        label = "blockhandle_random_access_file_name.borrow_end",
        identity_key = identity_key,
        file_name = %file_name,
    );

    file_name
}

/**
  | Invariant: this function performs exactly one
  | serialized RandomAccessFile::read operation
  | for the supplied shared file identity and
  | does not keep the RefCell borrow alive after
  | the read returns.
  |
  */
pub fn bitcoinleveldb_blockhandle_read_random_access_file(
    file:    &Rc<RefCell<dyn RandomAccessFile>>,
    offset:  u64,
    n:       usize,
    result:  *mut Slice,
    scratch: *mut u8,
) -> Status {
    let identity_key =
        bitcoinleveldb_blockhandle_random_access_file_identity_key(file);

    let access_lock =
        bitcoinleveldb_blockhandle_random_access_file_access_lock(file);

    let _access_guard = match access_lock.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            warn!(
                target: "bitcoinleveldb_blockhandle::global_state",
                label = "blockhandle_read_random_access_file.lock_poisoned",
                identity_key = identity_key,
                offset = offset,
                n = n,
            );
            poisoned.into_inner()
        }
    };

    trace!(
        target: "bitcoinleveldb_blockhandle::global_state",
        label = "blockhandle_read_random_access_file.entry",
        identity_key = identity_key,
        offset = offset,
        n = n,
    );

    let file_ref = file.borrow();

    let status = RandomAccessFileRead::read(
        &*file_ref,
        offset,
        n,
        result,
        scratch,
    );

    let status_ok = status.is_ok();

    trace!(
        target: "bitcoinleveldb_blockhandle::global_state",
        label = "blockhandle_read_random_access_file.exit",
        identity_key = identity_key,
        offset = offset,
        n = n,
        status_ok = status_ok,
    );

    if !status_ok {
        error!(
            target: "bitcoinleveldb_blockhandle::global_state",
            label = "blockhandle_read_random_access_file.read_failed",
            identity_key = identity_key,
            offset = offset,
            n = n,
        );
    }

    status
}

