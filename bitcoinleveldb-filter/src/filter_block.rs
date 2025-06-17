// ---------------- [ File: bitcoinleveldb-filter/src/filter_block.rs ]
/*!
  | A filter block is stored near the end of
  | a Table file.  It contains filters (e.g., bloom
  | filters) for all data blocks in the table
  | combined into a single filter block.
  |
  | See doc/table_format.md for an explanation of
  | the filter block format.
  */

crate::ix!();


/// We generate a new filter for every 2KB of data (C++: kFilterBaseLg=11 => 1<<11=2048).
pub const FILTER_BASE_LG: usize = 11;
pub const FILTER_BASE: usize    = 1 << FILTER_BASE_LG;

/// Appends a 32-bit little-endian integer to `dst`.
pub fn put_fixed32(dst: &mut Vec<u8>, value: u32) {
    trace!("put_fixed32 -> value={}", value);
    let bytes = value.to_le_bytes();
    dst.extend_from_slice(&bytes);
}

/// Reads a 32-bit little-endian integer from the first 4 bytes of `src`.
/// If `src` has fewer than 4 bytes, behavior is undefined (the caller ensures correctness).
pub fn decode_fixed32(src: &[u8]) -> u32 {
    let arr: [u8; 4] = src[0..4].try_into().unwrap();
    u32::from_le_bytes(arr)
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/filter_block.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/filter_block.cc]

/// A `FilterBlockBuilder` constructs all the filters for a table.
/// We remove `#[derive(Debug, Builder)]` to avoid compile errors.
#[derive(Getters, Setters)]
pub struct FilterBlockBuilder {
    policy: Box<dyn FilterPolicy>,

    /// Flattened key contents
    keys: Vec<u8>,

    /// For each key in `keys`, the offset where it starts
    start: Vec<usize>,

    /// Filter data (concatenated), plus offset array at the end
    result: Vec<u8>,

    /// Reused in `generate_filter()`
    tmp_keys: Vec<Slice>,

    filter_offsets: Vec<u32>,
}

impl FilterBlockBuilder {
    pub fn new(policy: Box<dyn FilterPolicy>) -> Self {
        info!("FilterBlockBuilder::new invoked");
        Self {
            policy,
            keys: Vec::new(),
            start: Vec::new(),
            result: Vec::new(),
            tmp_keys: Vec::new(),
            filter_offsets: Vec::new(),
        }
    }

    /// Start a filter region for the block at `block_offset`.
    pub fn start_block(&mut self, block_offset: u64) {
        trace!("start_block at offset={}", block_offset);
        let filter_index = (block_offset / FILTER_BASE as u64) as usize;
        assert!(
            filter_index >= self.filter_offsets.len(),
            "Block offset must not go backwards"
        );
        // Generate empty filters for any blocks we skipped
        while filter_index > self.filter_offsets.len() {
            self.generate_filter();
        }
    }

    /// Add a key to the current filter batch
    pub fn add_key(&mut self, key_: &Slice) {
        trace!("add_key with length={}", *key_.size());
        self.start.push(self.keys.len());
        let key_data = unsafe {
            // Must cast to *const u8, length = *key_.size()
            std::slice::from_raw_parts(*key_.data() as *const u8, *key_.size())
        };
        self.keys.extend_from_slice(key_data);
    }

    /// Finish building all filters. Returns a `Slice` over our result buffer.
    pub fn finish(&mut self) -> Slice {
        trace!("FilterBlockBuilder::finish invoked");
        // If there are pending keys, flush them
        if !self.start.is_empty() {
            self.generate_filter();
        }
        // Append array of per-filter offsets
        let array_offset = self.result.len();
        for &off in &self.filter_offsets {
            put_fixed32(&mut self.result, off);
        }
        // Append the start-of-offset-array, then the base_lg
        put_fixed32(&mut self.result, array_offset as u32);
        self.result.push(FILTER_BASE_LG as u8);

        info!("Finished building filter block, size={}", self.result.len());
        Slice::from_ptr_len(self.result.as_ptr(), self.result.len())
    }

    /// Flush any accumulated keys into a new filter block
    fn generate_filter(&mut self) {
        trace!("generate_filter invoked");
        let num_keys = self.start.len();
        if num_keys == 0 {
            // No keys => offset is current end-of-result
            self.filter_offsets.push(self.result.len() as u32);
            return;
        }
        // Add a sentinel so we can compute length of last key
        self.start.push(self.keys.len());
        self.tmp_keys.clear();
        // Prepare self.tmp_keys
        for i in 0..num_keys {
            let base = self.start[i];
            let length = self.start[i + 1] - base;
            self.tmp_keys.push(Slice::from_ptr_len(
                unsafe { self.keys.as_ptr().add(base) },
                length,
            ));
        }

        // The offset for this filter is the current length of result
        self.filter_offsets.push(self.result.len() as u32);

        // Build the filter
        self.policy.create_filter(
            self.tmp_keys.as_ptr(),
            num_keys as i32,
            &mut self.result
        );

        // Reset
        self.tmp_keys.clear();
        self.keys.clear();
        self.start.clear();
    }
}
