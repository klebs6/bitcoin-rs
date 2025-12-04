// ---------------- [ File: bitcoinleveldb-memenv/src/file_state_size.rs ]
crate::ix!();

impl FileState {
   
    pub fn size(&self) -> u64 {
        trace!("FileState::size: querying current file size");
        let blocks_ref = self.blocks_mutex().borrow();
        let guard = blocks_ref.lock();

        let size = *guard.size();
        debug!("FileState::size: returning size={}", size);
        size
    }
}

#[cfg(test)]
mod file_state_size_tests {
    use super::*;

    #[traced_test]
    fn size_tracks_total_appended_bytes() {
        crate::ix!();

        let mut file = FileState::default();
        assert_eq!(file.size(), 0);

        let part1 = b"abc";
        let part2 = b"defghijk";

        let s1 = Slice::from(&part1[..]);
        let s2 = Slice::from(&part2[..]);

        let status1 = file.append(&s1);
        assert!(status1.is_ok());
        assert_eq!(file.size(), part1.len() as u64);

        let status2 = file.append(&s2);
        assert!(status2.is_ok());
        assert_eq!(file.size(), (part1.len() + part2.len()) as u64);
    }
}
