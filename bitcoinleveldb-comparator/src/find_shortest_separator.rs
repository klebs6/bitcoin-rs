crate::ix!();

impl FindShortestSeparator for BytewiseComparatorImpl {
    fn find_shortest_separator(&self, start: &mut Vec<u8>, limit: &[u8]) {
        trace!(
            "BytewiseComparatorImpl::find_shortest_separator called with start={:?}, limit={:?}",
            start,
            limit
        );

        // 1. Find length of common prefix
        let min_length = std::cmp::min(start.len(), limit.len());
        let mut diff_index = 0;
        while diff_index < min_length {
            if start[diff_index] != limit[diff_index] {
                break;
            }
            diff_index += 1;
        }

        if diff_index < min_length {
            let diff_byte = start[diff_index];
            let limit_byte = limit[diff_index];

            // 2. If we can increment diff_byte without crossing limit_byte,
            //    do so and truncate.
            if diff_byte < 0xFF && (diff_byte + 1) < limit_byte {
                start[diff_index] = diff_byte + 1;
                // Truncate after diff_index + 1
                start.truncate(diff_index + 1);
                debug!("Shortened separator -> {:?}", start);
            }
        }
        // else they share the entire prefix up to min_length or
        // one is a prefix of the other, so do nothing
    }
}
