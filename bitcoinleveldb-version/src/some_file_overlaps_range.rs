// ---------------- [ File: bitcoinleveldb-version/src/some_file_overlaps_range.rs ]
crate::ix!();

pub fn some_file_overlaps_range(
    icmp:                 &InternalKeyComparator,
    disjoint_sorted:      bool,
    files:                &Vec<*mut FileMetaData>,
    smallest_user_key:    Option<&Slice>,
    largest_user_key:     Option<&Slice>,
) -> bool {
    if !disjoint_sorted {
        // Need to check against all files
        for &fptr in files.iter() {
            if fptr.is_null() {
                continue;
            }
            unsafe {
                let f = &*fptr;
                if after_file(smallest_user_key, f) || before_file(largest_user_key, f) {
                    // No overlap with this file
                } else {
                    return true; // Overlap
                }
            }
        }
        return false;
    }

    // disjoint_sorted == true: files sorted by smallest key, non‑overlapping.
    let index: usize = if let Some(smallest) = smallest_user_key {
        let ikey = InternalKey::new(smallest, MAX_SEQUENCE_NUMBER, VALUE_TYPE_FOR_SEEK);
        let encoded = ikey.encode();
        let i = find_file(icmp, files, &encoded);
        if i < 0 {
            0
        } else {
            i as usize
        }
    } else {
        0
    };

    if index >= files.len() {
        // beginning of range is after all files, so no overlap.
        return false;
    }

    unsafe {
        let f = &*files[index];
        !before_file(largest_user_key, f)
    }
}
