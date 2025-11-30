// ---------------- [ File: bitcoinleveldb-dumpfile/src/write_batch_item_printer.rs ]
crate::ix!();

/**
  | Called on every item found in a WriteBatch.
  |
  */
pub struct WriteBatchItemPrinter {
    dst:  *mut dyn WritableFile,
}

impl WriteBatchHandler for WriteBatchItemPrinter {

}

impl WriteBatchPut for WriteBatchItemPrinter {

    fn put(&mut self, 
        k:   &Slice,
        value: &Slice)  {
        
        todo!();
        /*
            std::string r = "  put '";
        AppendEscapedStringTo(&r, k);
        r += "' '";
        AppendEscapedStringTo(&r, value);
        r += "'\n";
        dst_->Append(r);
        */
    }
}

impl WriteBatchDelete for WriteBatchItemPrinter {

    fn delete(&mut self, k: &Slice)  {
        
        todo!();
        /*
            std::string r = "  del '";
        AppendEscapedStringTo(&r, k);
        r += "'\n";
        dst_->Append(r);
        */
    }
}
