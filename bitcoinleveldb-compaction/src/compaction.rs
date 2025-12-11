// ---------------- [ File: bitcoinleveldb-compaction/src/compaction.rs ]
crate::ix!();

/// A Compaction encapsulates information about a compaction.
/// 
#[derive(Getters,MutGetters,Setters,Builder)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
#[builder(pattern = "owned")]
pub struct Compaction {
    #[getset(skip)]
    level:                i32,

    #[getset(skip)]
    max_output_file_size: u64,

    input_version:        *mut Version,

    #[getset(skip)]
    edit:                 VersionEdit,

    // Each compaction reads inputs from "level_" and "level_+1"

    /// The two sets of inputs
    ///
    inputs: [Vec<*mut FileMetaData>; 2],

    /// State used to check for number of overlapping grandparent files 
    /// (parent == level_ + 1, grandparent == level_ + 2)
    /// 
    grandparents:      Vec<*mut FileMetaData>,

    /// Index in grandparent_starts_
    grandparent_index: usize,

    /// Some output key has been seen
    /// 
    seen_key_:         bool,

    /// Bytes of overlap between current output
    /// and grandparent files
    /// 
    overlapped_bytes:  i64,

    /* State for implementing IsBaseLevelForKey */

    /// level_ptrs_ holds indices into input_version_->levels_: our state is
    /// that we are positioned at one of the file ranges for each higher level
    /// than the ones involved in this compaction (i.e. for all L >= level_
    /// + 2).
    level_ptrs: [usize; NUM_LEVELS],
}

impl Compaction {

    /// Return the level that is being compacted.
    ///
    /// Inputs from "level" and "level+1" will be merged to produce a set of
    /// "level+1" files.
    ///
    pub fn level(&self) -> i32 {
        trace!(
            "Compaction::level: returning level {}",
            self.level
        );
        self.level
    }

    /// Return the object that holds the edits to the descriptor done by this
    /// compaction.
    ///
    pub fn edit(&mut self) -> *mut VersionEdit {
        let ptr: *mut VersionEdit = &mut self.edit;
        trace!(
            "Compaction::edit: returning edit pointer {:p}",
            ptr
        );
        ptr
    }

    /// "which" must be either 0 or 1
    ///
    pub fn num_input_files(&self, which: i32) -> i32 {
        let index = which as usize;
        let count = self.inputs[index].len() as i32;
        trace!(
            "Compaction::num_input_files: which={} index={} count={}",
            which,
            index,
            count
        );
        count
    }

    /// Return the ith input file at "level()+which" ("which" must be 0 or 1).
    ///
    pub fn input(&self, which: i32, i: i32) -> *mut FileMetaData {
        let which_index = which as usize;
        let file_index  = i as usize;
        let ptr = self.inputs[which_index][file_index];
        trace!(
            "Compaction::input: which={} index={} ptr={:p}",
            which,
            i,
            ptr
        );
        ptr
    }

    /// Maximum size of files to build during this compaction.
    ///
    pub fn max_output_file_size(&self) -> u64 {
        trace!(
            "Compaction::max_output_file_size: {} bytes",
            self.max_output_file_size
        );
        self.max_output_file_size
    }
}

#[cfg(test)]
mod compaction_basic_api_tests {
    use super::*;

    fn make_compaction(level: i32, max_file_size: usize) -> Compaction {
        let mut opts = Options::default();
        opts.set_max_file_size(max_file_size);
        Compaction::new(&opts as *const Options, level)
    }

    #[traced_test]
    fn compaction_level_and_max_output_file_size_match_options() {
        let level = 3;
        let max_file_size = 4 * 1024 * 1024;
        let c = make_compaction(level, max_file_size);

        assert_eq!(c.level(), level);
        assert_eq!(c.max_output_file_size(), max_file_size as u64);
    }

    #[traced_test]
    fn compaction_num_input_files_empty_for_new_compaction() {
        let opts = Options::default();
        let c = Compaction::new(&opts as *const Options, 0);

        assert_eq!(c.num_input_files(0), 0);
        assert_eq!(c.num_input_files(1), 0);
    }

    #[traced_test]
    fn compaction_num_input_files_and_input_access_are_consistent() {
        let opts = Options::default();
        let mut c = Compaction::new(&opts as *const Options, 0);

        let f1 = Box::new(FileMetaData::default());
        let f2 = Box::new(FileMetaData::default());

        let p1: *mut FileMetaData = Box::into_raw(f1);
        let p2: *mut FileMetaData = Box::into_raw(f2);

        c.inputs_mut()[0].push(p1);
        c.inputs_mut()[0].push(p2);

        assert_eq!(c.num_input_files(0), 2);
        assert_eq!(c.num_input_files(1), 0);

        let got1 = c.input(0, 0);
        let got2 = c.input(0, 1);

        assert_eq!(got1, p1);
        assert_eq!(got2, p2);

        drop(c);

        unsafe {
            drop(Box::from_raw(p1));
            drop(Box::from_raw(p2));
        }
    }

    #[traced_test]
    fn compaction_inputs_for_both_levels_are_tracked_independently() {
        let opts = Options::default();
        let mut c = Compaction::new(&opts as *const Options, 0);

        let f0 = Box::new(FileMetaData::default());
        let f1a = Box::new(FileMetaData::default());
        let f1b = Box::new(FileMetaData::default());

        let p0: *mut FileMetaData = Box::into_raw(f0);
        let p1a: *mut FileMetaData = Box::into_raw(f1a);
        let p1b: *mut FileMetaData = Box::into_raw(f1b);

        c.inputs_mut()[0].push(p0);
        c.inputs_mut()[1].push(p1a);
        c.inputs_mut()[1].push(p1b);

        assert_eq!(c.num_input_files(0), 1);
        assert_eq!(c.num_input_files(1), 2);

        let got0 = c.input(0, 0);
        let got1a = c.input(1, 0);
        let got1b = c.input(1, 1);

        assert_eq!(got0, p0);
        assert_eq!(got1a, p1a);
        assert_eq!(got1b, p1b);

        drop(c);

        unsafe {
            drop(Box::from_raw(p0));
            drop(Box::from_raw(p1a));
            drop(Box::from_raw(p1b));
        }
    }

    #[traced_test]
    fn compaction_edit_returns_mutable_pointer_into_struct() {
        let opts = Options::default();
        let mut c = Compaction::new(&opts as *const Options, 1);

        let edit_ptr = c.edit();
        assert!(!edit_ptr.is_null());

        unsafe {
            (*edit_ptr).set_log_number(42);
            assert_eq!(*(*c.edit()).log_number(), 42);
        }
    }

    #[traced_test]
    fn compaction_edit_pointer_is_stable_across_multiple_calls() {
        let opts = Options::default();
        let mut c = Compaction::new(&opts as *const Options, 2);

        let edit_ptr1 = c.edit();
        let edit_ptr2 = c.edit();

        assert_eq!(edit_ptr1, edit_ptr2);
        assert!(!edit_ptr1.is_null());

        unsafe {
            (*edit_ptr1).set_log_number(128);
            assert_eq!(*(*c.edit()).log_number(), 128);
        }
    }
}
