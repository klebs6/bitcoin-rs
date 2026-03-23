// ---------------- [ File: bitcoinleveldbt-faultinjection/src/truncate.rs ]
crate::ix!();

/**
  | A basic file truncation function suitable
  | for this test.
  |
  */
pub fn faultinjection_test_truncate(
    filename: &String,
    length:   u64,
) -> crate::Status {
    trace!(
        target: "bitcoinleveldbt_faultinjection::fault_injection_test",
        event = "truncate_entry",
        filename = %filename,
        length = length
    );

    let env = posix_default_env();

    let mut orig_file: *mut Box<dyn SequentialFile> = core::ptr::null_mut();
    let mut s = env.borrow_mut().new_sequential_file(
        filename,
        (&mut orig_file) as *mut *mut Box<dyn SequentialFile>,
    );

    if !s.is_ok() {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "truncate_exit",
            filename = %filename,
            ok = s.is_ok()
        );

        return s;
    }

    let mut scratch: Vec<u8> = vec![0u8; length as usize];
    let mut result: Slice = Slice::default();

    {
        let mut orig_file_holder: Box<Box<dyn SequentialFile>> = unsafe {
            Box::from_raw(orig_file)
        };
        let orig_file_ref: &mut Box<dyn SequentialFile> = orig_file_holder.as_mut();

        s = orig_file_ref.read(
            length as usize,
            (&mut result) as *mut Slice,
            scratch.as_mut_ptr(),
        );
    }

    if s.is_ok() {
        scratch.truncate(*result.size());

        let tmp_name = format!("{}/truncate.tmp", get_dir_name(filename));

        let mut tmp_file: *mut Box<dyn WritableFile> = core::ptr::null_mut();
        s = env.borrow_mut().new_writable_file(
            &tmp_name,
            (&mut tmp_file) as *mut *mut Box<dyn WritableFile>,
        );

        if s.is_ok() {
            let mut tmp_file_holder: Box<Box<dyn WritableFile>> = unsafe {
                Box::from_raw(tmp_file)
            };
            let tmp_file_ref: &mut Box<dyn WritableFile> = tmp_file_holder.as_mut();

            let append_slice = Slice::from(scratch.as_slice());
            s = tmp_file_ref.append(&append_slice);

            if s.is_ok() {
                s = tmp_file_ref.close();
            }

            drop(tmp_file_holder);

            if s.is_ok() {
                s = env.borrow_mut().rename_file(&tmp_name, filename);
            } else {
                let _ = env.borrow_mut().delete_file(&tmp_name);
            }
        }
    }

    trace!(
        target: "bitcoinleveldbt_faultinjection::fault_injection_test",
        event = "truncate_exit",
        filename = %filename,
        ok = s.is_ok()
    );

    s
}
