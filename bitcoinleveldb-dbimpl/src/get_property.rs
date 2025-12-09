// ---------------- [ File: bitcoinleveldb-dbimpl/src/get_property.rs ]
crate::ix!();

impl GetProperty for DBImpl {
    
    fn get_property(&mut self, 
        property: &str,
        value:    *mut String) -> bool {
        
        todo!();
        /*
            value->clear();

      MutexLock l(&mutex_);
      Slice in = property;
      Slice prefix("leveldb.");
      if (!in.starts_with(prefix)) return false;
      in.remove_prefix(prefix.size());

      if (in.starts_with("num-files-at-level")) {
        in.remove_prefix(strlen("num-files-at-level"));
        uint64_t level;
        bool ok = ConsumeDecimalNumber(&in, &level) && in.empty();
        if (!ok || level >= config::kNumLevels) {
          return false;
        } else {
          char buf[100];
          snprintf(buf, sizeof(buf), "%d",
                   versions_->NumLevelFiles(static_cast<int>(level)));
          *value = buf;
          return true;
        }
      } else if (in == "stats") {
        char buf[200];
        snprintf(buf, sizeof(buf),
                 "                               Compactions\n"
                 "Level  Files Size(MB) Time(sec) Read(MB) Write(MB)\n"
                 "--------------------------------------------------\n");
        value->append(buf);
        for (int level = 0; level < config::kNumLevels; level++) {
          int files = versions_->NumLevelFiles(level);
          if (stats_[level].micros > 0 || files > 0) {
            snprintf(buf, sizeof(buf), "%3d %8d %8.0f %9.0f %8.0f %9.0f\n", level,
                     files, versions_->NumLevelBytes(level) / 1048576.0,
                     stats_[level].micros / 1e6,
                     stats_[level].bytes_read / 1048576.0,
                     stats_[level].bytes_written / 1048576.0);
            value->append(buf);
          }
        }
        return true;
      } else if (in == "sstables") {
        *value = versions_->current()->DebugString();
        return true;
      } else if (in == "approximate-memory-usage") {
        size_t total_usage = options_.block_cache->TotalCharge();
        if (mem_) {
          total_usage += mem_->ApproximateMemoryUsage();
        }
        if (imm_) {
          total_usage += imm_->ApproximateMemoryUsage();
        }
        char buf[50];
        snprintf(buf, sizeof(buf), "%llu",
                 static_cast<unsigned long long>(total_usage));
        value->append(buf);
        return true;
      }

      return false;
        */
    }
}
