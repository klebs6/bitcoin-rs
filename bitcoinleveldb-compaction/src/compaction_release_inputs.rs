// ---------------- [ File: bitcoinleveldb-compaction/src/compaction_release_inputs.rs ]
crate::ix!();

impl Compaction {

    /// Release the input version for the compaction, once the compaction is
    /// successful.
    /// 
    pub fn release_inputs(&mut self) {
        let raw_input_version_ptr: *mut Version = *self.input_version();

        trace!(
            "Compaction::release_inputs: enter; input_version_ptr={:p}",
            raw_input_version_ptr
        );

        unsafe {
            if !raw_input_version_ptr.is_null() {
                let v: &mut Version = &mut *raw_input_version_ptr;
                trace!(
                    "Compaction::release_inputs: calling Version::unref on {:p}",
                    raw_input_version_ptr
                );
                v.unref();
                self.set_input_version(core::ptr::null_mut());
                trace!(
                    "Compaction::release_inputs: input_version_ cleared"
                );
            } else {
                trace!(
                    "Compaction::release_inputs: input_version_ already null; nothing to do"
                );
            }
        }
    }
}

#[cfg(test)]
mod compaction_release_inputs_safe_noop_tests {
    use super::*;

    #[traced_test]
    fn release_inputs_on_new_compaction_is_safe_noop() {
        let opts = Options::default();
        let mut c = Compaction::new(&opts as *const Options, 0);
        c.release_inputs();
    }

    #[traced_test]
    fn release_inputs_is_idempotent_when_input_version_is_null() {
        let opts = Options::default();
        let mut c = Compaction::new(&opts as *const Options, 1);

        assert!((*c.input_version()).is_null());

        c.release_inputs();
        assert!((*c.input_version()).is_null());

        c.release_inputs();
        assert!((*c.input_version()).is_null());
    }
}
