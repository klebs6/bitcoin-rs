// ---------------- [ File: bitcoinleveldb-dumpfile/src/corruption_reporter.rs ]
crate::ix!();

/**
  | Notified when log reader encounters
  | corruption.
  |
  */
pub struct CorruptionReporter {
    dst:  *mut dyn WritableFile,
}

impl LogReaderReporter for CorruptionReporter {

    fn corruption(&mut self, 
        bytes:  usize,
        status: &Status)  {
        
        todo!();
        /*
            std::string r = "corruption: ";
        AppendNumberTo(&r, bytes);
        r += " bytes; ";
        r += status.ToString();
        r.push_back('\n');
        dst_->Append(r);
        */
    }
}
