// ---------------- [ File: bitcoinleveldb-dbimpl/src/do_compaction_work.rs ]
crate::ix!();

impl DBImpl {
    
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn do_compaction_work(&mut self, compact: *mut CompactionState) -> crate::Status {
        
        todo!();
        /*
            const uint64_t start_micros = env_->NowMicros();
      int64_t imm_micros = 0;  // Micros spent doing imm_ compactions

      Log(options_.info_log, "Compacting %d@%d + %d@%d files",
          compact->compaction->num_input_files(0), compact->compaction->level(),
          compact->compaction->num_input_files(1),
          compact->compaction->level() + 1);

      assert(versions_->NumLevelFiles(compact->compaction->level()) > 0);
      assert(compact->builder == nullptr);
      assert(compact->outfile == nullptr);
      if (snapshots_.empty()) {
        compact->smallest_snapshot = versions_->LastSequence();
      } else {
        compact->smallest_snapshot = snapshots_.oldest()->sequence_number();
      }

      Iterator* input = versions_->MakeInputIterator(compact->compaction);

      // Release mutex while we're actually doing the compaction work
      mutex_.Unlock();

      input->SeekToFirst();
      Status status;
      ParsedInternalKey ikey;
      std::string current_user_key;
      bool has_current_user_key = false;
      SequenceNumber last_sequence_for_key = kMaxSequenceNumber;
      while (input->Valid() && !shutting_down_.load(std::memory_order_acquire)) {
        // Prioritize immutable compaction work
        if (has_imm_.load(std::memory_order_relaxed)) {
          const uint64_t imm_start = env_->NowMicros();
          mutex_.Lock();
          if (imm_ != nullptr) {
            CompactMemTable();
            // Wake up MakeRoomForWrite() if necessary.
            background_work_finished_signal_.SignalAll();
          }
          mutex_.Unlock();
          imm_micros += (env_->NowMicros() - imm_start);
        }

        Slice key = input->key();
        if (compact->compaction->ShouldStopBefore(key) &&
            compact->builder != nullptr) {
          status = FinishCompactionOutputFile(compact, input);
          if (!status.ok()) {
            break;
          }
        }

        // Handle key/value, add to state, etc.
        bool drop = false;
        if (!ParseInternalKey(key, &ikey)) {
          // Do not hide error keys
          current_user_key.clear();
          has_current_user_key = false;
          last_sequence_for_key = kMaxSequenceNumber;
        } else {
          if (!has_current_user_key ||
              user_comparator()->Compare(ikey.user_key, Slice(current_user_key)) !=
                  0) {
            // First occurrence of this user key
            current_user_key.assign(ikey.user_key.data(), ikey.user_key.size());
            has_current_user_key = true;
            last_sequence_for_key = kMaxSequenceNumber;
          }

          if (last_sequence_for_key <= compact->smallest_snapshot) {
            // Hidden by an newer entry for same user key
            drop = true;  // (A)
          } else if (ikey.type == kTypeDeletion &&
                     ikey.sequence <= compact->smallest_snapshot &&
                     compact->compaction->IsBaseLevelForKey(ikey.user_key)) {
            // For this user key_:
            // (1) there is no data in higher levels
            // (2) data in lower levels will have larger sequence numbers
            // (3) data in layers that are being compacted here and have
            //     smaller sequence numbers will be dropped in the next
            //     few iterations of this loop (by rule (A) above).
            // Therefore this deletion marker is obsolete and can be dropped.
            drop = true;
          }

          last_sequence_for_key = ikey.sequence;
        }
    #if 0
        Log(options_.info_log,
            "  Compact: %s, seq %d, type: %d %d, drop: %d, is_base: %d, "
            "%d smallest_snapshot: %d",
            ikey.user_key.ToString().c_str(),
            (int)ikey.sequence, ikey.type, kTypeValue, drop,
            compact->compaction->IsBaseLevelForKey(ikey.user_key),
            (int)last_sequence_for_key, (int)compact->smallest_snapshot);
    #endif

        if (!drop) {
          // Open output file if necessary
          if (compact->builder == nullptr) {
            status = OpenCompactionOutputFile(compact);
            if (!status.ok()) {
              break;
            }
          }
          if (compact->builder->NumEntries() == 0) {
            compact->current_output()->smallest.DecodeFrom(key);
          }
          compact->current_output()->largest.DecodeFrom(key);
          compact->builder->Add(key, input->value());

          // Close output file if it is big enough
          if (compact->builder->FileSize() >=
              compact->compaction->MaxOutputFileSize()) {
            status = FinishCompactionOutputFile(compact, input);
            if (!status.ok()) {
              break;
            }
          }
        }

        input->Next();
      }

      if (status.ok() && shutting_down_.load(std::memory_order_acquire)) {
        status = Status::IOError("Deleting DB during compaction");
      }
      if (status.ok() && compact->builder != nullptr) {
        status = FinishCompactionOutputFile(compact, input);
      }
      if (status.ok()) {
        status = input->status();
      }
      delete input;
      input = nullptr;

      CompactionStats stats;
      stats.micros = env_->NowMicros() - start_micros - imm_micros;
      for (int which = 0; which < 2; which++) {
        for (int i = 0; i < compact->compaction->num_input_files(which); i++) {
          stats.bytes_read += compact->compaction->input(which, i)->file_size;
        }
      }
      for (size_t i = 0; i < compact->outputs.size(); i++) {
        stats.bytes_written += compact->outputs[i].file_size;
      }

      mutex_.Lock();
      stats_[compact->compaction->level() + 1].Add(stats);

      if (status.ok()) {
        status = InstallCompactionResults(compact);
      }
      if (!status.ok()) {
        RecordBackgroundError(status);
      }
      VersionSet::LevelSummaryStorage tmp;
      Log(options_.info_log, "compacted to: %s", versions_->LevelSummary(&tmp));
      return status;
        */
    }
}
