crate::ix!();

impl DBImpl {
    
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn background_compaction(&mut self)  {
        
        todo!();
        /*
            mutex_.AssertHeld();

      if (imm_ != nullptr) {
        CompactMemTable();
        return;
      }

      Compaction* c;
      bool is_manual = (manual_compaction_ != nullptr);
      InternalKey manual_end;
      if (is_manual) {
        ManualCompaction* m = manual_compaction_;
        c = versions_->CompactRange(m->level, m->begin, m->end);
        m->done = (c == nullptr);
        if (c != nullptr) {
          manual_end = c->input(0, c->num_input_files(0) - 1)->largest;
        }
        Log(options_.info_log,
            "Manual compaction at level-%d from %s .. %s; will stop at %s\n",
            m->level, (m->begin ? m->begin->DebugString().c_str() : "(begin)"),
            (m->end ? m->end->DebugString().c_str() : "(end)"),
            (m->done ? "(end)" : manual_end.DebugString().c_str()));
      } else {
        c = versions_->PickCompaction();
      }

      Status status;
      if (c == nullptr) {
        // Nothing to do
      } else if (!is_manual && c->IsTrivialMove()) {
        // Move file to next level
        assert(c->num_input_files(0) == 1);
        FileMetaData* f = c->input(0, 0);
        c->edit()->DeleteFile(c->level(), f->number);
        c->edit()->AddFile(c->level() + 1, f->number, f->file_size, f->smallest,
                           f->largest);
        status = versions_->LogAndApply(c->edit(), &mutex_);
        if (!status.ok()) {
          RecordBackgroundError(status);
        }
        VersionSet::LevelSummaryStorage tmp;
        Log(options_.info_log, "Moved #%lld to level-%d %lld bytes %s: %s\n",
            static_cast<unsigned long long>(f->number), c->level() + 1,
            static_cast<unsigned long long>(f->file_size),
            status.ToString().c_str(), versions_->LevelSummary(&tmp));
      } else {
        CompactionState* compact = new CompactionState(c);
        status = DoCompactionWork(compact);
        if (!status.ok()) {
          RecordBackgroundError(status);
        }
        CleanupCompaction(compact);
        c->ReleaseInputs();
        DeleteObsoleteFiles();
      }
      delete c;

      if (status.ok()) {
        // Done
      } else if (shutting_down_.load(std::memory_order_acquire)) {
        // Ignore compaction errors found during shutting down
      } else {
        Log(options_.info_log, "Compaction error: %s", status.ToString().c_str());
      }

      if (is_manual) {
        ManualCompaction* m = manual_compaction_;
        if (!status.ok()) {
          m->done = true;
        }
        if (!m->done) {
          // We only compacted part of the requested range.  Update *m
          // to the range that is left to be compacted.
          m->tmp_storage = manual_end;
          m->begin = &m->tmp_storage;
        }
        manual_compaction_ = nullptr;
      }
        */
    }
}
