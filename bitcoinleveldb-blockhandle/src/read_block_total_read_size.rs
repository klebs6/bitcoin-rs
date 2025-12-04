// ---------------- [ File: bitcoinleveldb-blockhandle/src/read_block_total_read_size.rs ]
crate::ix!();

pub fn read_block_total_read_size(block_size: usize) -> usize {
    let total = block_size + BLOCK_TRAILER_SIZE;

    trace!(
        "read_block_total_read_size: block_size={}, trailer_size={}, total={}",
        block_size,
        BLOCK_TRAILER_SIZE,
        total
    );

    total
}

#[cfg(test)]
mod read_block_total_read_size_unit_tests {
    use super::*;

    #[traced_test]
    fn total_read_size_adds_trailer_to_block_size() {
        let block_size = 10usize;
        let total =
            read_block_total_read_size(block_size);

        trace!(
            "total_read_size_adds_trailer_to_block_size: block_size={}, total={}",
            block_size,
            total
        );

        assert_eq!(
            total,
            block_size + BLOCK_TRAILER_SIZE
        );
    }

    #[traced_test]
    fn total_read_size_for_zero_block_is_trailer_size() {
        let total = read_block_total_read_size(0);

        trace!(
            "total_read_size_for_zero_block_is_trailer_size: total={}",
            total
        );

        assert_eq!(total, BLOCK_TRAILER_SIZE);
    }
}
