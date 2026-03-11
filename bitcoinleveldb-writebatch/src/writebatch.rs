// ---------------- [ File: bitcoinleveldb-writebatch/src/writebatch.rs ]
crate::ix!();

#[derive(Default,Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct LevelDBWriteBatch {
    rep: WriteBatch,
}

pub type BitcoinLevelDbWriteBatchPutCallback = fn(
    state: *mut c_void,
    k:     *const u8,
    klen:  usize,
    v:     *const u8,
    vlen:  usize,
);

pub type BitcoinLevelDbWriteBatchDeleteCallback = fn(
    state: *mut c_void,
    k:     *const u8,
    klen:  usize,
);

pub fn leveldb_writebatch_create() -> *mut LevelDBWriteBatch {
    Box::into_raw(Box::new(LevelDBWriteBatch::default()))
}

pub fn leveldb_writebatch_destroy(b: *mut LevelDBWriteBatch) {
    unsafe {
        if !b.is_null() {
            drop(Box::from_raw(b));
        }
    }
}

pub fn leveldb_writebatch_clear(b: *mut LevelDBWriteBatch) {
    unsafe {
        let batch_ref = &mut *b;
        batch_ref.rep_mut().clear();
    }
}

pub fn leveldb_writebatch_put(
    b:    *mut LevelDBWriteBatch,
    key_: *const u8,
    klen: usize,
    val:  *const u8,
    vlen: usize
)
{
    unsafe {
        let batch_ref = &mut *b;
        let key_slice = Slice::from_ptr_len(key_, klen);
        let value_slice = Slice::from_ptr_len(val, vlen);
        batch_ref.rep_mut().put(&key_slice, &value_slice);
    }
}

pub fn leveldb_writebatch_delete(b: *mut LevelDBWriteBatch, key_: *const u8, klen: usize) {
    unsafe {
        let batch_ref = &mut *b;
        let key_slice = Slice::from_ptr_len(key_, klen);
        batch_ref.rep_mut().delete(&key_slice);
    }
}

pub fn leveldb_writebatch_iterate(
    b:       *const LevelDBWriteBatch,
    state:   *mut c_void,
    put:     BitcoinLevelDbWriteBatchPutCallback,
    deleted: BitcoinLevelDbWriteBatchDeleteCallback
)
{
    struct BitcoinLevelDbWriteBatchCallbackHandlerBridge {
        state:   *mut c_void,
        put:     BitcoinLevelDbWriteBatchPutCallback,
        deleted: BitcoinLevelDbWriteBatchDeleteCallback,
    }

    impl WriteBatchHandler for BitcoinLevelDbWriteBatchCallbackHandlerBridge {}

    impl WriteBatchPut for BitcoinLevelDbWriteBatchCallbackHandlerBridge {
        fn put(&mut self, key_: &Slice, value: &Slice) {
            let key_bytes = key_.as_bytes();
            let value_bytes = value.as_bytes();
            (self.put)(
                self.state,
                key_bytes.as_ptr(),
                key_bytes.len(),
                value_bytes.as_ptr(),
                value_bytes.len(),
            );
        }
    }

    impl WriteBatchDelete for BitcoinLevelDbWriteBatchCallbackHandlerBridge {
        fn delete(&mut self, key_: &Slice) {
            let key_bytes = key_.as_bytes();
            (self.deleted)(
                self.state,
                key_bytes.as_ptr(),
                key_bytes.len(),
            );
        }
    }

    unsafe {
        let batch_ref = &*b;
        let mut handler = BitcoinLevelDbWriteBatchCallbackHandlerBridge {
            state,
            put,
            deleted,
        };
        let _ = batch_ref.rep().iterate(&mut handler as *mut dyn WriteBatchHandler);
    }
}

pub fn leveldb_writebatch_append(destination: *mut LevelDBWriteBatch, source: *const LevelDBWriteBatch) {
    unsafe {
        let destination_ref = &mut *destination;
        let source_ref = &*source;
        destination_ref.rep_mut().append(source_ref.rep());
    }
}
