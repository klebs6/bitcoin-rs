// ---------------- [ File: bitcoinleveldb-repair/src/repair.rs ]
/*!
  | We recover the contents of the descriptor from
  | the other files we find.
  |
  | (1) Any log files are first converted to tables
  |
  | (2) We scan every table to compute
  |     (a) smallest/largest for the table
  |     (b) largest sequence number in the table
  |
  | (3) We generate descriptor contents:
  |      - log number is set to zero
  |
  |      - next-file-number is set to 1 + largest
  |        file number we found
  |
  |      - last-sequence-number is set to largest
  |        sequence# found across all tables (see
  |        2c)
  |
  |      - compaction pointers are cleared
  |
  |      - every table file is added at level 0
  |
  | Possible optimization 1:
  |
  |   (a) Compute total size and use to pick
  |       appropriate max-level M
  |
  |   (b) Sort tables by largest sequence# in the
  |       table
  |
  |   (c) For each table: if it overlaps earlier
  |       table, place in level-0, else place in
  |       level-M.
  |
  | Possible optimization 2:
  |
  |   Store per-table metadata (smallest, largest,
  |   largest-seq#, ...) in the table's meta
  |   section to speed up ScanTable.
  */

crate::ix!();

pub fn repairdb(
        dbname:  &String,
        options: &Options) -> crate::Status {
    
    todo!();

    /*
    let repairer: Repairer = Repairer::new(dbname, options);

    repairer.run()
    */
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/repair.cc]

pub struct Repairer {
    dbname:           String,
    env:              Box<dyn Env>,
    icmp:             InternalKeyComparator,
    ipolicy:          InternalFilterPolicy,
    options:          Options,
    owns_info_log:    bool,
    owns_cache:       bool,
    table_cache:      *mut TableCache,
    edit:             VersionEdit,
    manifests:        Vec<String>,
    table_numbers:    Vec<u64>,
    logs:             Vec<u64>,
    tables:           Vec<repairer::TableInfo>,
    next_file_number: u64,
}

pub mod repairer {
    use super::*;

    pub struct TableInfo {
        meta:         FileMetaData,
        max_sequence: SequenceNumber,
    }
}

impl Drop for Repairer {
    fn drop(&mut self) {
        todo!();
        /*
            delete table_cache_;
        if (owns_info_log_) {
          delete options_.info_log;
        }
        if (owns_cache_) {
          delete options_.block_cache;
        }
        */
    }
}

impl Repairer {

    pub fn new(
        dbname:  &String,
        options: &Options) -> Self {
    
        todo!();
        /*


            : dbname_(dbname),
            env_(options.env),
            icmp_(options.comparator),
            ipolicy_(options.filter_policy),
            options_(SanitizeOptions(dbname, &icmp_, &ipolicy_, options)),
            owns_info_log_(options_.info_log != options.info_log),
            owns_cache_(options_.block_cache != options.block_cache),
            next_file_number_(1) 

        // TableCache can be small since we expect each table to be opened once.
        table_cache_ = new TableCache(dbname_, options_, 10);
        */
    }
    
    pub fn run(&mut self) -> crate::Status {

        todo!();
        /*
        let mut status: Status = find_files();

        if status.ok() {

            convert_log_files_to_tables();

            extract_meta_data();

            status = write_descriptor();
        }
        */
        
        todo!();
        /*
        if (status.ok()) {
          unsigned long long bytes = 0;
          for (size_t i = 0; i < tables_.size(); i++) {
            bytes += tables_[i].meta.file_size;
          }
          Log(options_.info_log,
              "**** Repaired leveldb %s; "
              "recovered %d files; %llu bytes. "
              "Some data may have been lost. "
              "****",
              dbname_.c_str(), static_cast<int>(tables_.size()), bytes);
        }
        return status;
        */
    }
    
    pub fn find_files(&mut self) -> crate::Status {
        
        todo!();
        /*
            std::vector<std::string> filenames;
        Status status = env_->GetChildren(dbname_, &filenames);
        if (!status.ok()) {
          return status;
        }
        if (filenames.empty()) {
          return Status::IOError(dbname_, "repair found no files");
        }

        uint64_t number;
        FileType type;
        for (size_t i = 0; i < filenames.size(); i++) {
          if (ParseFileName(filenames[i], &number, &type)) {
            if (type == kDescriptorFile) {
              manifests_.push_back(filenames[i]);
            } else {
              if (number + 1 > next_file_number_) {
                next_file_number_ = number + 1;
              }
              if (type == kLogFile) {
                logs_.push_back(number);
              } else if (type == kTableFile) {
                table_numbers_.push_back(number);
              } else {
                // Ignore other files
              }
            }
          }
        }
        return status;
        */
    }
    
    pub fn convert_log_files_to_tables(&mut self)  {
        
        todo!();
        /*
            for (size_t i = 0; i < logs_.size(); i++) {
          std::string logname = LogFileName(dbname_, logs_[i]);
          Status status = ConvertLogToTable(logs_[i]);
          if (!status.ok()) {
            Log(options_.info_log, "Log #%llu: ignoring conversion error: %s",
                (unsigned long long)logs_[i], status.ToString().c_str());
          }
          ArchiveFile(logname);
        }
        */
    }
    
    pub fn convert_log_to_table(&mut self, log: u64) -> crate::Status {
        
        todo!();
        /*
            struct LogReporter : public LogReader::Reporter {
          Env* env;
          Logger* info_log;
          uint64_t lognum;
          c_void Corruption(size_t bytes, const Status& s) override {
            // We print error messages for corruption, but continue repairing.
            Log(info_log, "Log #%llu: dropping %d bytes; %s",
                (unsigned long long)lognum, static_cast<int>(bytes),
                s.ToString().c_str());
          }
        };

        // Open the log file
        std::string logname = LogFileName(dbname_, log);
        SequentialFile* lfile;
        Status status = env_->NewSequentialFile(logname, &lfile);
        if (!status.ok()) {
          return status;
        }

        // Create the log reader.
        LogReporter reporter;
        reporter.env = env_;
        reporter.info_log = options_.info_log;
        reporter.lognum = log;
        // We intentionally make LogReader do checksumming so that
        // corruptions cause entire commits to be skipped instead of
        // propagating bad information (like overly large sequence
        // numbers).
        LogReader reader(lfile, &reporter, false /*do not checksum*/,
                           0 /*initial_offset*/);

        // Read all the records and add to a memtable
        std::string scratch;
        Slice record;
        WriteBatch batch;
        MemTable* mem = new MemTable(icmp_);
        mem->Ref();
        int counter = 0;
        while (reader.ReadRecord(&record, &scratch)) {
          if (record.size() < 12) {
            reporter.Corruption(record.size(),
                                Status::Corruption("log record too small", logname));
            continue;
          }
          WriteBatchInternal::SetContents(&batch, record);
          status = WriteBatchInternal::InsertInto(&batch, mem);
          if (status.ok()) {
            counter += WriteBatchInternal::Count(&batch);
          } else {
            Log(options_.info_log, "Log #%llu: ignoring %s",
                (unsigned long long)log, status.ToString().c_str());
            status = Status::OK();  // Keep going with rest of file
          }
        }
        delete lfile;

        // Do not record a version edit for this conversion to a Table
        // since ExtractMetaData() will also generate edits.
        FileMetaData meta;
        meta.number = next_file_number_++;
        Iterator* iter = mem->NewIterator();
        status = BuildTable(dbname_, env_, options_, table_cache_, iter, &meta);
        delete iter;
        mem->Unref();
        mem = nullptr;
        if (status.ok()) {
          if (meta.file_size > 0) {
            table_numbers_.push_back(meta.number);
          }
        }
        Log(options_.info_log, "Log #%llu: %d ops saved to Table #%llu %s",
            (unsigned long long)log, counter, (unsigned long long)meta.number,
            status.ToString().c_str());
        return status;
        */
    }
    
    pub fn extract_meta_data(&mut self)  {
        
        todo!();
        /*
            for (size_t i = 0; i < table_numbers_.size(); i++) {
          ScanTable(table_numbers_[i]);
        }
        */
    }
    
    pub fn new_table_iterator(&mut self, meta: &FileMetaData) -> *mut LevelDBIterator {
        
        todo!();
        /*
            // Same as compaction iterators: if paranoid_checks are on, turn
        // on checksum verification.
        ReadOptions r;
        r.verify_checksums = options_.paranoid_checks;
        return table_cache_->NewIterator(r, meta.number, meta.file_size);
        */
    }
    
    pub fn scan_table(&mut self, number: u64)  {
        
        todo!();
        /*
            TableInfo t;
        t.meta.number = number;
        std::string fname = TableFileName(dbname_, number);
        Status status = env_->GetFileSize(fname, &t.meta.file_size);
        if (!status.ok()) {
          // Try alternate file name.
          fname = SSTTableFileName(dbname_, number);
          Status s2 = env_->GetFileSize(fname, &t.meta.file_size);
          if (s2.ok()) {
            status = Status::OK();
          }
        }
        if (!status.ok()) {
          ArchiveFile(TableFileName(dbname_, number));
          ArchiveFile(SSTTableFileName(dbname_, number));
          Log(options_.info_log, "Table #%llu: dropped: %s",
              (unsigned long long)t.meta.number, status.ToString().c_str());
          return;
        }

        // Extract metadata by scanning through table.
        int counter = 0;
        Iterator* iter = NewTableIterator(t.meta);
        bool empty = true;
        ParsedInternalKey parsed;
        t.max_sequence = 0;
        for (iter->SeekToFirst(); iter->Valid(); iter->Next()) {
          Slice key = iter->key();
          if (!ParseInternalKey(key, &parsed)) {
            Log(options_.info_log, "Table #%llu: unparsable key %s",
                (unsigned long long)t.meta.number, EscapeString(key).c_str());
            continue;
          }

          counter++;
          if (empty) {
            empty = false;
            t.meta.smallest.DecodeFrom(key);
          }
          t.meta.largest.DecodeFrom(key);
          if (parsed.sequence > t.max_sequence) {
            t.max_sequence = parsed.sequence;
          }
        }
        if (!iter->status().ok()) {
          status = iter->status();
        }
        delete iter;
        Log(options_.info_log, "Table #%llu: %d entries %s",
            (unsigned long long)t.meta.number, counter, status.ToString().c_str());

        if (status.ok()) {
          tables_.push_back(t);
        } else {
          RepairTable(fname, t);  // RepairTable archives input file.
        }
        */
    }
    
    pub fn repair_table(&mut self, 
        src: &String,
        t:   repairer::TableInfo)  {
        
        todo!();
        /*
            // We will copy src contents to a new table and then rename the
        // new table over the source.

        // Create builder.
        std::string copy = TableFileName(dbname_, next_file_number_++);
        WritableFile* file;
        Status s = env_->NewWritableFile(copy, &file);
        if (!s.ok()) {
          return;
        }
        TableBuilder* builder = new TableBuilder(options_, file);

        // Copy data.
        Iterator* iter = NewTableIterator(t.meta);
        int counter = 0;
        for (iter->SeekToFirst(); iter->Valid(); iter->Next()) {
          builder->Add(iter->key(), iter->value());
          counter++;
        }
        delete iter;

        ArchiveFile(src);
        if (counter == 0) {
          builder->Abandon();  // Nothing to save
        } else {
          s = builder->Finish();
          if (s.ok()) {
            t.meta.file_size = builder->FileSize();
          }
        }
        delete builder;
        builder = nullptr;

        if (s.ok()) {
          s = file->Close();
        }
        delete file;
        file = nullptr;

        if (counter > 0 && s.ok()) {
          std::string orig = TableFileName(dbname_, t.meta.number);
          s = env_->RenameFile(copy, orig);
          if (s.ok()) {
            Log(options_.info_log, "Table #%llu: %d entries repaired",
                (unsigned long long)t.meta.number, counter);
            tables_.push_back(t);
          }
        }
        if (!s.ok()) {
          env_->DeleteFile(copy);
        }
        */
    }
    
    pub fn write_descriptor(&mut self) -> crate::Status {
        
        todo!();
        /*
            std::string tmp = TempFileName(dbname_, 1);
        WritableFile* file;
        Status status = env_->NewWritableFile(tmp, &file);
        if (!status.ok()) {
          return status;
        }

        SequenceNumber max_sequence = 0;
        for (size_t i = 0; i < tables_.size(); i++) {
          if (max_sequence < tables_[i].max_sequence) {
            max_sequence = tables_[i].max_sequence;
          }
        }

        edit_.SetComparatorName(icmp_.user_comparator()->Name());
        edit_.SetLogNumber(0);
        edit_.SetNextFile(next_file_number_);
        edit_.SetLastSequence(max_sequence);

        for (size_t i = 0; i < tables_.size(); i++) {
          // TODO(opt): separate out into multiple levels
          const TableInfo& t = tables_[i];
          edit_.AddFile(0, t.meta.number, t.meta.file_size, t.meta.smallest,
                        t.meta.largest);
        }

        // fprintf(stderr, "NewDescriptor:\n%s\n", edit_.DebugString().c_str());
        {
          LogWriter log(file);
          std::string record;
          edit_.EncodeTo(&record);
          status = log.AddRecord(record);
        }
        if (status.ok()) {
          status = file->Close();
        }
        delete file;
        file = nullptr;

        if (!status.ok()) {
          env_->DeleteFile(tmp);
        } else {
          // Discard older manifests
          for (size_t i = 0; i < manifests_.size(); i++) {
            ArchiveFile(dbname_ + "/" + manifests_[i]);
          }

          // Install new manifest
          status = env_->RenameFile(tmp, DescriptorFileName(dbname_, 1));
          if (status.ok()) {
            status = SetCurrentFile(env_, dbname_, 1);
          } else {
            env_->DeleteFile(tmp);
          }
        }
        return status;
        */
    }
    
    pub fn archive_file(&mut self, fname: &String)  {
        
        todo!();
        /*
            // Move into another directory.  E.g., for
        //    dir/foo
        // rename to
        //    dir/lost/foo
        const char* slash = strrchr(fname.c_str(), '/');
        std::string new_dir;
        if (slash != nullptr) {
          new_dir.assign(fname.data(), slash - fname.data());
        }
        new_dir.append("/lost");
        env_->CreateDir(new_dir);  // Ignore error
        std::string new_file = new_dir;
        new_file.append("/");
        new_file.append((slash == nullptr) ? fname.c_str() : slash + 1);
        Status s = env_->RenameFile(fname, new_file);
        Log(options_.info_log, "Archiving %s: %s\n", fname.c_str(),
            s.ToString().c_str());
        */
    }
}
