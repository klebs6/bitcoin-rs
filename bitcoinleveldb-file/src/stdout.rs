// ---------------- [ File: bitcoinleveldb-file/src/stdout.rs ]
crate::ix!();

pub struct StdoutPrinter {

}

impl WritableFile for StdoutPrinter {

}

impl WritableFileClose for StdoutPrinter {
    fn close(&mut self) -> crate::Status {
        
        todo!();
        /*
            return Status::OK();
        */
    }
}

impl WritableFileAppend for StdoutPrinter {

    fn append(&mut self, data: &Slice) -> crate::Status {
        
        todo!();
        /*
            fwrite(data.data(), 1, data.size(), stdout);
        return Status::OK();
        */
    }
}

impl WritableFileFlush for StdoutPrinter {

    fn flush(&mut self) -> crate::Status {
        
        todo!();
        /*
            return Status::OK();
        */
    }
}

impl WritableFileSync for StdoutPrinter {

    fn sync(&mut self) -> crate::Status {
        
        todo!();
        /*
            return Status::OK();
        */
    }
}

impl GetName for StdoutPrinter {

    fn get_name(&self) -> &'static str {
        
        todo!();
        /*
            return "[stdout]";
        */
    }
}
