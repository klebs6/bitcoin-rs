// ---------------- [ File: bitcoinleveldb-filter/src/filter_block_reader.rs ]
crate::ix!();

/// A `FilterBlockReader` parses the filter block stored in `contents` and
/// answers queries via `key_may_match`.
#[derive(Getters, Setters)]
pub struct FilterBlockReader {
    policy: Box<dyn FilterPolicy>,

    /// The entire filter block data
    data: Arc<[u8]>,

    /// The offset of the start of the offset array within `data`
    offset: usize,

    /// Number of filter offsets
    num: usize,

    /// Usually 11 for 2KB
    base_lg: usize,

    valid: bool,
}

impl FilterBlockReader {

    pub fn new(policy: Box<dyn FilterPolicy>, contents: &Slice) -> Self {

        info!("FilterBlockReader::new with size={}", *contents.size());

        let src = unsafe {
            // Must cast to *const u8, length = *contents.size()
            std::slice::from_raw_parts(*contents.data() as *const u8, *contents.size())
        };

        let data: Arc<[u8]> = Arc::from(src);

        let n = data.len();

        if n < 5 {
            warn!("Data too short (<5 bytes). Mark invalid");
            return Self {
                policy,
                data,
                offset: 0,
                num: 0,
                base_lg: 0,
                valid: false,
            };
        }

        let base_lg = data[n - 1] as usize;

        let last_word = decode_fixed32(&data[n - 5..n - 1]) as usize;

        if last_word > n - 5 {
            warn!("Offset array start out of range. Mark invalid");
            return Self {
                policy,
                data,
                offset: 0,
                num: 0,
                base_lg,
                valid: false,
            };
        }

        let offset = last_word;
        let num = (n - 5 - last_word) / 4;

        info!("Parsed FilterBlockReader: offset={}, num={}, base_lg={}", offset, num, base_lg);
        Self {
            policy,
            data,
            offset,
            num,
            base_lg,
            valid: true,
        }
    }

    /// Return true if `key` may be present in the filter for block at `block_offset`.
    pub fn key_may_match(&self, block_offset: u64, key: &Slice) -> bool {
        trace!("key_may_match offset={}, key_len={}", block_offset, *key.size());
        if !self.valid {
            trace!("Invalid filter => treat as possible match");
            return true;
        }

        let index = (block_offset >> self.base_lg) as usize;
        if index < self.num {
            // read offset array to find filter region
            let idx1 = self.offset + index * 4;
            let idx2 = idx1 + 4;
            let start = decode_fixed32(&self.data[idx1..idx2]) as usize;

            let idx3 = idx2;
            let idx4 = idx3 + 4;
            let limit = decode_fixed32(&self.data[idx3..idx4]) as usize;

            // The filter data is [start..limit]
            if start <= limit && limit <= self.offset {
                let filter_data = &self.data[start..limit];
                let filter_slice = Slice::from_ptr_len(
                    filter_data.as_ptr(),
                    filter_data.len(),
                );
                return self.policy.key_may_match(key, &filter_slice);
            } else if start == limit {
                // Empty filter
                return false;
            }
        }
        // If out-of-range, treat as potential match
        true
    }
}
