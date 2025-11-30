// ---------------- [ File: bitcoinleveldb-dumpfile/src/dump_log.rs ]
crate::ix!();

pub fn dump_log(
        env:   Rc<RefCell<dyn crate::Env>>,
        fname: &String,
        dst:   Rc<RefCell<dyn WritableFile>>) -> crate::Status {
    
    todo!();
        /*
            return PrintLogContents(env, fname, WriteBatchPrinter, dst);
        */
}
