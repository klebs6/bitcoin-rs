// ---------------- [ File: bitcoinleveldb-compaction/src/compaction_is_trivial_move.rs ]
crate::ix!();

impl Compaction {

    /// Is this a trivial compaction that can be implemented by just moving
    /// a single input file to the next level (no merging or splitting)
    ///
    pub fn is_trivial_move(&self) -> bool {
        trace!(
            "Compaction::is_trivial_move: checking compaction at level {}",
            self.level()
        );

        let input0 = self.num_input_files(0);
        let input1 = self.num_input_files(1);

        trace!(
            "Compaction::is_trivial_move: num_input_files(0)={} num_input_files(1)={}",
            input0,
            input1
        );

        if input0 != 1 || input1 != 0 {
            debug!(
                "Compaction::is_trivial_move: requires exactly one level-{} file and zero level-{} files; returning false",
                self.level(),
                self.level() + 1
            );
            return false;
        }

        let input_version_ptr: *mut Version = *self.input_version();
        if input_version_ptr.is_null() {
            debug!("Compaction::is_trivial_move: input_version_ is null; returning false");
            return false;
        }

        unsafe {
            let input_version: &mut Version = &mut *input_version_ptr;
            let vset_ptr: *mut dyn VersionSetInterface = input_version.vset();

            assert!(
                !vset_ptr.is_null(),
                "Compaction::is_trivial_move: vset pointer is null"
            );

            let vset: &mut dyn VersionSetInterface = &mut *vset_ptr;
            let options_ptr: *const Options = vset.options();

            let grandparents_bytes:  i64 = total_file_size(self.grandparents());
            let max_allowed_overlap: i64 = max_grand_parent_overlap_bytes(options_ptr);

            let allowed = grandparents_bytes <= max_allowed_overlap;

            debug!(
                "Compaction::is_trivial_move: grandparents_bytes={} max_allowed_overlap={} -> allowed={}",
                grandparents_bytes,
                max_allowed_overlap,
                allowed
            );

            allowed
        }
    }
}

#[cfg(test)]
mod compaction_trivial_move_basic_tests {
    use super::*;

    fn make_empty_compaction(level: i32) -> Compaction {
        let opts = Options::default();
        Compaction::new(&opts as *const Options, level)
    }

    #[traced_test]
    fn trivial_move_false_when_input_file_counts_do_not_match() {
        let mut c = make_empty_compaction(0);

        assert!(!c.is_trivial_move());

        let f1 = Box::new(FileMetaData::default());
        let f2 = Box::new(FileMetaData::default());
        let p1 = Box::into_raw(f1);
        let p2 = Box::into_raw(f2);

        c.inputs_mut()[0].push(p1);
        c.inputs_mut()[0].push(p2);

        assert!(!c.is_trivial_move());

        drop(c);
        unsafe {
            drop(Box::from_raw(p1));
            drop(Box::from_raw(p2));
        }
    }

    #[traced_test]
    fn trivial_move_false_when_input_version_is_null_even_if_counts_match() {
        let mut c = make_empty_compaction(0);

        let f = Box::new(FileMetaData::default());
        let p = Box::into_raw(f);
        c.inputs_mut()[0].push(p);

        assert_eq!(c.num_input_files(0), 1);
        assert_eq!(c.num_input_files(1), 0);

        assert!(!c.is_trivial_move());

        drop(c);
        unsafe {
            drop(Box::from_raw(p));
        }
    }

    #[traced_test]
    fn trivial_move_false_when_only_next_level_has_files() {
        let mut c = make_empty_compaction(0);

        let f = Box::new(FileMetaData::default());
        let p = Box::into_raw(f);

        c.inputs_mut()[1].push(p);

        assert_eq!(c.num_input_files(0), 0);
        assert_eq!(c.num_input_files(1), 1);

        assert!(!c.is_trivial_move());

        drop(c);
        unsafe {
            drop(Box::from_raw(p));
        }
    }

    #[traced_test]
    fn trivial_move_ignores_grandparents_when_input_version_is_null() {
        let mut c = make_empty_compaction(0);

        let f_input = Box::new(FileMetaData::default());
        let f_gp = Box::new(FileMetaData::default());

        let p_input = Box::into_raw(f_input);
        let p_gp = Box::into_raw(f_gp);

        c.inputs_mut()[0].push(p_input);
        c.grandparents_mut().push(p_gp);

        assert_eq!(c.num_input_files(0), 1);
        assert_eq!(c.num_input_files(1), 0);

        assert!(!c.is_trivial_move());

        drop(c);

        unsafe {
            drop(Box::from_raw(p_input));
            drop(Box::from_raw(p_gp));
        }
    }
}
