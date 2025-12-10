// ---------------- [ File: bitcoinleveldb-versionsetutil/src/total_file_size.rs ]
crate::ix!();

pub fn total_file_size(files: &Vec<*mut FileMetaData>) -> i64 {

    let mut sum: i64 = 0;

    for (i, &f_ptr) in files.iter().enumerate() {
        unsafe {
            let f = &*f_ptr;
            let sz_u64 = *f.file_size();
            let sz_i64 = sz_u64 as i64;
            sum += sz_i64;
            debug!(
                index = i,
                file_number = *f.number(),
                file_size = sz_u64,
                running_total = sum,
                "total_file_size: accumulating"
            );
        }
    }

    trace!(total = sum, file_count = files.len(), "total_file_size: done");
    sum
}

#[cfg(test)]
mod total_file_size_spec {
    use super::*;

    #[traced_test]
    fn verify_total_file_size_empty_list_is_zero() {
        let files: Vec<*mut FileMetaData> = Vec::new();
        let sum = total_file_size(&files);
        assert_eq!(0, sum, "An empty file list must have total size zero");
    }

    #[traced_test]
    fn verify_total_file_size_accumulates_sizes() {
        let mut f1 = FileMetaData::default();
        f1.set_number(1);
        f1.set_file_size(10);

        let mut f2 = FileMetaData::default();
        f2.set_number(2);
        f2.set_file_size(20);

        let mut f3 = FileMetaData::default();
        f3.set_number(3);
        f3.set_file_size(30);

        let mut b1 = Box::new(f1);
        let mut b2 = Box::new(f2);
        let mut b3 = Box::new(f3);

        let p1: *mut FileMetaData = &mut *b1;
        let p2: *mut FileMetaData = &mut *b2;
        let p3: *mut FileMetaData = &mut *b3;

        let files = vec![p1, p2, p3];

        let sum = total_file_size(&files);

        debug!(
            sum = sum,
            expected = 60,
            "verify_total_file_size_accumulates_sizes"
        );

        assert_eq!(60, sum, "File sizes 10 + 20 + 30 must sum to 60");
    }
}
