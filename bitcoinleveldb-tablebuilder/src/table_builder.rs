// ---------------- [ File: bitcoinleveldb-tablebuilder/src/table_builder.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/table_builder.h]

/// TableBuilder provides the interface used to build a Table (an immutable and
/// sorted map from keys to values).
/// 
/// Multiple threads can invoke const methods on a TableBuilder without external
/// synchronization, but if any of the threads may call a non-const method, all
/// threads accessing the same TableBuilder must use external synchronization.
///
pub struct TableBuilder {
    rep: *mut TableBuilderRep,
}

impl TableBuilder {

    pub fn invalid() -> Self {
        Self {
            rep: std::ptr::null_mut()
        }
    }

    /// Create a builder that will store the contents of the table it is
    /// building in *file.  Does not close the file.
    ///
    /// It is up to the caller to close the file after calling Finish().
    ///
    pub fn new(options: &Options, file: *mut dyn WritableFile) -> Self {
        unsafe {
            assert!(
                !file.is_null(),
                "TableBuilder::new: file pointer is null"
            );

            trace!(
                "TableBuilder::new: constructing TableBuilderRep for file={:?}",
                file
            );

            let rep_box = Box::new(TableBuilderRep::new(options, file));
            let rep_ptr: *mut TableBuilderRep = Box::into_raw(rep_box);

            let rep_ref: &mut TableBuilderRep = &mut *rep_ptr;
            let filter_block_ptr: *mut FilterBlockBuilder = rep_ref.filter_block();

            if !filter_block_ptr.is_null() {
                trace!(
                    "TableBuilder::new: starting first filter block at offset 0 (filter_block={:?})",
                    filter_block_ptr
                );
                let fb: &mut FilterBlockBuilder = &mut *filter_block_ptr;
                fb.start_block(0);
            } else {
                trace!(
                    "TableBuilder::new: filter_block is null; filters disabled for this table"
                );
            }

            TableBuilder { rep: rep_ptr }
        }
    }

    #[inline]
    pub(crate) fn rep_ptr(&self) -> *mut TableBuilderRep {
        self.rep
    }

    #[inline]
    pub(crate) fn rep_ptr_mut(&mut self) -> *mut TableBuilderRep {
        self.rep
    }

    #[inline]
    pub(crate) fn set_rep_ptr(&mut self, rep: *mut TableBuilderRep) {
        self.rep = rep;
    }
}

#[cfg(test)]
pub(crate) mod table_builder_construction_tests {
    use super::*;

    #[traced_test]
    fn table_builder_new_initial_state_is_consistent() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("table_builder_new_initial_state_is_consistent");

        trace!(
            "table_builder_new_initial_state_is_consistent: validating invariants for freshly constructed TableBuilder"
        );

        unsafe {
            let rep_ptr = builder.rep_ptr();
            assert!(
                !rep_ptr.is_null(),
                "expected non-null TableBuilderRep pointer"
            );

            let rep: &TableBuilderRep = &*rep_ptr;

            assert_eq!(
                *rep.num_entries(),
                0,
                "new builder should start with zero entries"
            );
            assert_eq!(
                *rep.offset(),
                0,
                "new builder should start at offset 0"
            );
            assert!(
                !*rep.closed(),
                "new builder must not be marked closed"
            );
            assert!(
                rep.status().is_ok(),
                "new builder must start with OK status"
            );
            assert!(
                !rep.filter_block().is_null(),
                "new builder should create a filter block by default"
            );
        }

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw);
        }
    }

    #[traced_test]
    fn table_builder_rep_pointer_accessors_match() {
        let (mut builder, _options, file_raw) =
            create_table_builder_for_test("table_builder_rep_pointer_accessors_match");

        trace!(
            "table_builder_rep_pointer_accessors_match: verifying rep_ptr and rep_ptr_mut return the same pointer"
        );

        unsafe {
            let rep_const = builder.rep_ptr();
            let rep_mut   = builder.rep_ptr_mut();

            assert_eq!(
                rep_const, rep_mut,
                "rep_ptr and rep_ptr_mut should return the same pointer"
            );
        }

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(file_raw);
        }
    }

    #[traced_test]
    fn table_builder_set_rep_ptr_overwrites_internal_pointer() {
        let (mut builder, options_ref, file_raw) =
            create_table_builder_for_test("table_builder_set_rep_ptr_overwrites_internal_pointer");

        trace!("table_builder_set_rep_ptr_overwrites_internal_pointer: creating alternate rep");

        let alt_file_box =
            Box::new(InMemoryWritableFile::new_for_test("alt_rep_file"));
        let alt_file_raw: *mut InMemoryWritableFile = Box::into_raw(alt_file_box);
        let alt_file_trait: *mut dyn WritableFile =
            alt_file_raw as *mut dyn WritableFile;

        let alt_rep_box =
            Box::new(TableBuilderRep::new(options_ref, alt_file_trait));
        let alt_rep_raw: *mut TableBuilderRep = Box::into_raw(alt_rep_box);

        unsafe {
            let original_rep_raw = builder.rep_ptr();
            assert!(
                !original_rep_raw.is_null(),
                "original rep pointer must not be null"
            );

            builder.set_rep_ptr(alt_rep_raw);

            assert_eq!(
                builder.rep_ptr(),
                alt_rep_raw,
                "set_rep_ptr must update the internal rep pointer"
            );

            trace!(
                "table_builder_set_rep_ptr_overwrites_internal_pointer: dropping original rep that is no longer referenced"
            );

            let _ = Box::from_raw(original_rep_raw);
        }

        builder.abandon();
        drop(builder);

        unsafe {
            let _ = Box::from_raw(alt_file_raw);
            let _ = Box::from_raw(file_raw);
        }
    }
}
