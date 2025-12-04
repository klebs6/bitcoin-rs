// ---------------- [ File: bitcoinleveldb-block/src/block_builder.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/block_builder.h]

/**
  | BlockBuilder generates blocks where keys are
  | prefix-compressed:
  |
  | When we store a key, we drop the prefix shared
  | with the previous string.  This helps reduce
  | the space requirement significantly.
  | Furthermore, once every K keys, we do not apply
  | the prefix compression and store the entire
  | key.  We call this a "restart point".  The tail
  | end of the block stores the offsets of all of
  | the restart points, and can be used to do
  | a binary search when looking for a particular
  | key.  Values are stored as-is (without
  | compression) immediately following the
  | corresponding key.
  |
  | An entry for a particular key-value pair has the form:
  |     shared_bytes: varint32
  |     unshared_bytes: varint32
  |     value_length: varint32
  |     key_delta: char[unshared_bytes]
  |     value: char[value_length]
  | shared_bytes == 0 for restart points.
  |
  | The trailer of the block has the form:
  |     restarts: uint32[num_restarts]
  |     num_restarts: uint32
  |
  | restarts[i] contains the offset within the
  | block of the ith restart point.
  */
#[derive(Setters,Getters,MutGetters)]
#[getset(set="pub",get="pub",get_mut="pub")]
pub struct BlockBuilder {

    options:  *const Options,

    /**
      | Destination buffer
      |
      */
    buffer:   String,

    /**
      | Restart points
      |
      */
    restarts: Vec<u32>,

    /**
      | Number of entries emitted since restart
      |
      */
    counter:  i32,

    /**
      | Has Finish() been called?
      |
      */
    finished: bool,

    last_key: String,
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/block_builder.cc]
impl BlockBuilder {

    pub fn new(options: *const Options) -> Self {
        unsafe {
            assert!(
                !options.is_null(),
                "BlockBuilder::new: options pointer is null"
            );

            let interval: i32 = *(*options).block_restart_interval();

            assert!(
                interval >= 1,
                "BlockBuilder::new: block_restart_interval {} < 1",
                interval
            );

            trace!(
                "BlockBuilder::new: block_restart_interval={}",
                interval
            );
        }

        let mut restarts = Vec::<u32>::new();
        restarts.push(0);

        trace!(
            "BlockBuilder::new: initialized with first restart at offset 0"
        );

        BlockBuilder {
            options,
            buffer:    String::new(),
            restarts,
            counter:   0,
            finished: false,
            last_key: String::new(),
        }
    }

    /**
      | Return true iff no entries have been
      | added since the last Reset()
      |
      */
    pub fn empty(&self) -> bool {
        trace!(
            "BlockBuilder::empty called; buffer_len={}",
            self.buffer.len()
        );
        self.buffer.is_empty()
    }

    #[inline]
    pub fn options_ptr(&self) -> *const Options {
        self.options
    }

    #[inline]
    pub fn buffer_len(&self) -> usize {
        self.buffer.len()
    }

    #[inline]
    pub fn buffer_is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    #[inline]
    pub fn buffer_clear(&mut self) {
        self.buffer.clear();
    }

    #[inline]
    pub fn buffer_as_mut_vec(&mut self) -> &mut Vec<u8> {
        unsafe { self.buffer.as_mut_vec() }
    }

    #[inline]
    pub fn buffer_string(&self) -> &String {
        &self.buffer
    }

    #[inline]
    pub fn restarts_len(&self) -> usize {
        self.restarts.len()
    }

    #[inline]
    pub fn restarts_clear(&mut self) {
        self.restarts.clear();
    }

    #[inline]
    pub fn restarts_push(&mut self, offset: u32) {
        self.restarts.push(offset);
    }

    #[inline]
    pub fn restarts_slice(&self) -> &[u32] {
        &self.restarts
    }

    #[inline]
    pub fn inc_counter(&mut self) {
        self.counter += 1;
    }

    #[inline]
    pub fn is_finished(&self) -> bool {
        self.finished
    }
}

#[cfg(test)]
mod block_builder_constructor_and_helpers_tests {
    use super::*;
    use tracing::{debug, trace};

    fn new_boxed_options_for_block_builder() -> Box<Options> {
        trace!("allocating boxed Options for BlockBuilder tests");
        let opts = Box::new(Options::default());
        debug!(
            "created Options for BlockBuilder tests; block_restart_interval={}",
            *opts.block_restart_interval()
        );
        opts
    }

    #[traced_test]
    fn block_builder_new_creates_empty_builder_with_initial_restart_point() {
        let options = new_boxed_options_for_block_builder();
        let options_ptr: *const Options = &*options;

        let builder = BlockBuilder::new(options_ptr);

        trace!("verifying freshly constructed BlockBuilder state");
        assert!(builder.empty());
        assert_eq!(builder.buffer_len(), 0);
        assert_eq!(builder.restarts_len(), 1);
        assert_eq!(builder.restarts_slice(), &[0u32]);
        assert_eq!(*builder.counter(), 0);
        assert!(!builder.is_finished());
    }

    #[traced_test]
    fn block_builder_empty_tracks_buffer_emptiness() {
        let options = new_boxed_options_for_block_builder();
        let options_ptr: *const Options = &*options;

        let mut builder = BlockBuilder::new(options_ptr);
        assert!(builder.empty());

        let key   = Slice::from("a".as_bytes());
        let value = Slice::from("v1".as_bytes());
        trace!("adding single entry to BlockBuilder");
        builder.add(&key, &value);

        assert!(!builder.empty());
        assert!(builder.buffer_len() > 0);
    }

    #[traced_test]
    fn block_builder_buffer_accessors_operate_consistently() {
        let options = new_boxed_options_for_block_builder();
        let options_ptr: *const Options = &*options;

        let mut builder = BlockBuilder::new(options_ptr);
        assert_eq!(builder.buffer_len(), 0);

        {
            let buf: &mut Vec<u8> = builder.buffer_as_mut_vec();
            buf.extend_from_slice(b"xyz");
        }

        trace!(
            "after mutating buffer via buffer_as_mut_vec, len={}",
            builder.buffer_len()
        );
        assert_eq!(builder.buffer_len(), 3);
        assert_eq!(builder.buffer_string().as_bytes(), b"xyz");
    }
}
