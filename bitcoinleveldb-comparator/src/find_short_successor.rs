crate::ix!();

impl FindShortSuccessor for BytewiseComparatorImpl {

    fn find_short_successor(&self, key: &mut Vec<u8>) {
        trace!(
            "BytewiseComparatorImpl::find_short_successor called with key={:?}",
            key
        );

        // 1. Find first character that can be incremented
        for i in 0..key.len() {
            if key[i] != 0xFF {
                key[i] = key[i] + 1; // increment it
                key.truncate(i + 1); // truncate the rest
                debug!("Short successor -> {:?}", key);
                return;
            }
        }
        // If entire key is 0xFF, do nothing
        trace!("Key is a run of 0xFF bytes; leaving unchanged");
    }
}
