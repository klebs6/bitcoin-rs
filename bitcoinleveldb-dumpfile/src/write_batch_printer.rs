// ---------------- [ File: bitcoinleveldb-dumpfile/src/write_batch_printer.rs ]
crate::ix!();

/**
  | Called on every log record (each one
  | of which is a WriteBatch) found in a kLogFile.
  |
  */
pub fn write_batch_printer(
        pos:    u64,
        record: Slice,
        dst:    Rc<RefCell<dyn WritableFile>>)  {
    
    todo!();
        /*
            std::string r = "--- offset ";
      AppendNumberTo(&r, pos);
      r += "; ";
      if (record.size() < 12) {
        r += "log record length ";
        AppendNumberTo(&r, record.size());
        r += " is too small\n";
        dst->Append(r);
        return;
      }
      WriteBatch batch;
      WriteBatchInternal::SetContents(&batch, record);
      r += "sequence ";
      AppendNumberTo(&r, WriteBatchInternal::Sequence(&batch));
      r.push_back('\n');
      dst->Append(r);
      WriteBatchItemPrinter batch_item_printer;
      batch_item_printer.dst_ = dst;
      Status s = batch.Iterate(&batch_item_printer);
      if (!s.ok()) {
        dst->Append("  error: " + s.ToString() + "\n");
      }
        */
}
