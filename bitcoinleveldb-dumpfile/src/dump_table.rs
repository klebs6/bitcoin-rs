// ---------------- [ File: bitcoinleveldb-dumpfile/src/dump_table.rs ]
crate::ix!();

pub fn dump_table(
        env:   Rc<RefCell<dyn crate::Env>>,
        fname: &String,
        dst:   *mut dyn WritableFile) -> crate::Status {
    
    todo!();
        /*
            uint64_t file_size;
      RandomAccessFile* file = nullptr;
      Table* table = nullptr;
      Status s = env->GetFileSize(fname, &file_size);
      if (s.ok()) {
        s = env->NewRandomAccessFile(fname, &file);
      }
      if (s.ok()) {
        // We use the default comparator, which may or may not match the
        // comparator used in this database. However this should not cause
        // problems since we only use Table operations that do not require
        // any comparisons.  In particular, we do not call Seek or Prev.
        s = Table::Open(Options(), file, file_size, &table);
      }
      if (!s.ok()) {
        delete table;
        delete file;
        return s;
      }

      ReadOptions ro;
      ro.fill_cache = false;
      Iterator* iter = table->NewIterator(ro);
      std::string r;
      for (iter->SeekToFirst(); iter->Valid(); iter->Next()) {
        r.clear();
        ParsedInternalKey key;
        if (!ParseInternalKey(iter->key(), &key)) {
          r = "badkey '";
          AppendEscapedStringTo(&r, iter->key());
          r += "' => '";
          AppendEscapedStringTo(&r, iter->value());
          r += "'\n";
          dst->Append(r);
        } else {
          r = "'";
          AppendEscapedStringTo(&r, key.user_key);
          r += "' @ ";
          AppendNumberTo(&r, key.sequence);
          r += " : ";
          if (key.type == kTypeDeletion) {
            r += "del";
          } else if (key.type == kTypeValue) {
            r += "val";
          } else {
            AppendNumberTo(&r, key.type);
          }
          r += " => '";
          AppendEscapedStringTo(&r, iter->value());
          r += "'\n";
          dst->Append(r);
        }
      }
      s = iter->status();
      if (!s.ok()) {
        dst->Append("iterator error: " + s.ToString() + "\n");
      }

      delete iter;
      delete table;
      delete file;
      return Status::OK();
        */
}
