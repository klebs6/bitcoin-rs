crate::ix!();

pub fn compare_iterators(
        step:       i32,
        model:      *mut dyn DB,
        db:         *mut dyn DB,
        model_snap: *const dyn Snapshot,
        db_snap:    *const dyn Snapshot) -> bool {
    
    todo!();
        /*
            ReadOptions options;
      options.snapshot = model_snap;
      Iterator* miter = model->NewIterator(options);
      options.snapshot = db_snap;
      Iterator* dbiter = db->NewIterator(options);
      bool ok = true;
      int count = 0;
      for (miter->SeekToFirst(), dbiter->SeekToFirst();
           ok && miter->Valid() && dbiter->Valid(); miter->Next(), dbiter->Next()) {
        count++;
        if (miter->key().compare(dbiter->key()) != 0) {
          fprintf(stderr, "step %d: Key mismatch: '%s' vs. '%s'\n", step,
                  EscapeString(miter->key()).c_str(),
                  EscapeString(dbiter->key()).c_str());
          ok = false;
          break;
        }

        if (miter->value().compare(dbiter->value()) != 0) {
          fprintf(stderr, "step %d: Value mismatch for key '%s': '%s' vs. '%s'\n",
                  step, EscapeString(miter->key()).c_str(),
                  EscapeString(miter->value()).c_str(),
                  EscapeString(miter->value()).c_str());
          ok = false;
        }
      }

      if (ok) {
        if (miter->Valid() != dbiter->Valid()) {
          fprintf(stderr, "step %d: Mismatch at end of iterators: %d vs. %d\n",
                  step, miter->Valid(), dbiter->Valid());
          ok = false;
        }
      }
      fprintf(stderr, "%d entries compared: ok=%d\n", count, ok);
      delete miter;
      delete dbiter;
      return ok;
        */
}


