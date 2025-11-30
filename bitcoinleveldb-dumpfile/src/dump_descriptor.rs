// ---------------- [ File: bitcoinleveldb-dumpfile/src/dump_descriptor.rs ]
crate::ix!();

pub fn dump_descriptor(
        env:   Rc<RefCell<dyn crate::Env>>,
        fname: &String,
        dst:   *mut dyn WritableFile) -> crate::Status {
    
    todo!();
        /*
            return PrintLogContents(env, fname, VersionEditPrinter, dst);
        */
}
