// ---------------- [ File: bitcoinleveldb-dbtest/src/tests.rs ]
crate::ix!();

#[test] fn db_test_empty() {
    todo!();
    /*
    
      do {
        ASSERT_TRUE(db_ != nullptr);
        ASSERT_EQ("NOT_FOUND", Get("foo"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_empty_key() {
    todo!();
    /*
    
      do {
        ASSERT_OK(Put("", "v1"));
        ASSERT_EQ("v1", Get(""));
        ASSERT_OK(Put("", "v2"));
        ASSERT_EQ("v2", Get(""));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_empty_value() {
    todo!();
    /*
    
      do {
        ASSERT_OK(Put("key", "v1"));
        ASSERT_EQ("v1", Get("key"));
        ASSERT_OK(Put("key", ""));
        ASSERT_EQ("", Get("key"));
        ASSERT_OK(Put("key", "v2"));
        ASSERT_EQ("v2", Get("key"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_read_write() {
    todo!();
    /*
    
      do {
        ASSERT_OK(Put("foo", "v1"));
        ASSERT_EQ("v1", Get("foo"));
        ASSERT_OK(Put("bar", "v2"));
        ASSERT_OK(Put("foo", "v3"));
        ASSERT_EQ("v3", Get("foo"));
        ASSERT_EQ("v2", Get("bar"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_put_delete_get() {
    todo!();
    /*
    
      do {
        ASSERT_OK(db_->Put(WriteOptions(), "foo", "v1"));
        ASSERT_EQ("v1", Get("foo"));
        ASSERT_OK(db_->Put(WriteOptions(), "foo", "v2"));
        ASSERT_EQ("v2", Get("foo"));
        ASSERT_OK(db_->Delete(WriteOptions(), "foo"));
        ASSERT_EQ("NOT_FOUND", Get("foo"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_get_from_immutable_layer() {
    todo!();
    /*
    
      do {
        Options options = CurrentOptions();
        options.env = env_;
        options.write_buffer_size = 100000;  // Small write buffer
        Reopen(&options);

        ASSERT_OK(Put("foo", "v1"));
        ASSERT_EQ("v1", Get("foo"));

        // Block sync calls.
        env_->delay_data_sync_.store(true, std::memory_order_release);
        Put("k1", std::string(100000, 'x'));  // Fill memtable.
        Put("k2", std::string(100000, 'y'));  // Trigger compaction.
        ASSERT_EQ("v1", Get("foo"));
        // Release sync calls.
        env_->delay_data_sync_.store(false, std::memory_order_release);
      } while (ChangeOptions());

    */
}

#[test] fn db_test_get_from_versions() {
    todo!();
    /*
    
      do {
        ASSERT_OK(Put("foo", "v1"));
        dbfull()->TEST_CompactMemTable();
        ASSERT_EQ("v1", Get("foo"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_get_mem_usage() {
    todo!();
    /*
    
      do {
        ASSERT_OK(Put("foo", "v1"));
        std::string val;
        ASSERT_TRUE(db_->GetProperty("leveldb.approximate-memory-usage", &val));
        int mem_usage = std::stoi(val);
        ASSERT_GT(mem_usage, 0);
        ASSERT_LT(mem_usage, 5 * 1024 * 1024);
      } while (ChangeOptions());

    */
}

#[test] fn db_test_get_snapshot() {
    todo!();
    /*
    
      do {
        // Try with both a short key and a long key
        for (int i = 0; i < 2; i++) {
          std::string key = (i == 0) ? std::string("foo") : std::string(200, 'x');
          ASSERT_OK(Put(key, "v1"));
          const Snapshot* s1 = db_->GetSnapshot();
          ASSERT_OK(Put(key, "v2"));
          ASSERT_EQ("v2", Get(key));
          ASSERT_EQ("v1", Get(key, s1));
          dbfull()->TEST_CompactMemTable();
          ASSERT_EQ("v2", Get(key));
          ASSERT_EQ("v1", Get(key, s1));
          db_->ReleaseSnapshot(s1);
        }
      } while (ChangeOptions());

    */
}

#[test] fn db_test_get_identical_snapshots() {
    todo!();
    /*
    
      do {
        // Try with both a short key and a long key
        for (int i = 0; i < 2; i++) {
          std::string key = (i == 0) ? std::string("foo") : std::string(200, 'x');
          ASSERT_OK(Put(key, "v1"));
          const Snapshot* s1 = db_->GetSnapshot();
          const Snapshot* s2 = db_->GetSnapshot();
          const Snapshot* s3 = db_->GetSnapshot();
          ASSERT_OK(Put(key, "v2"));
          ASSERT_EQ("v2", Get(key));
          ASSERT_EQ("v1", Get(key, s1));
          ASSERT_EQ("v1", Get(key, s2));
          ASSERT_EQ("v1", Get(key, s3));
          db_->ReleaseSnapshot(s1);
          dbfull()->TEST_CompactMemTable();
          ASSERT_EQ("v2", Get(key));
          ASSERT_EQ("v1", Get(key, s2));
          db_->ReleaseSnapshot(s2);
          ASSERT_EQ("v1", Get(key, s3));
          db_->ReleaseSnapshot(s3);
        }
      } while (ChangeOptions());

    */
}

#[test] fn db_test_iterate_over_empty_snapshot() {
    todo!();
    /*
    
      do {
        const Snapshot* snapshot = db_->GetSnapshot();
        ReadOptions read_options;
        read_options.snapshot = snapshot;
        ASSERT_OK(Put("foo", "v1"));
        ASSERT_OK(Put("foo", "v2"));

        Iterator* iterator1 = db_->NewIterator(read_options);
        iterator1->SeekToFirst();
        ASSERT_TRUE(!iterator1->Valid());
        delete iterator1;

        dbfull()->TEST_CompactMemTable();

        Iterator* iterator2 = db_->NewIterator(read_options);
        iterator2->SeekToFirst();
        ASSERT_TRUE(!iterator2->Valid());
        delete iterator2;

        db_->ReleaseSnapshot(snapshot);
      } while (ChangeOptions());

    */
}

#[test] fn db_test_get_level_0ordering() {
    todo!();
    /*
    
      do {
        // Check that we process level-0 files in correct order.  The code
        // below generates two level-0 files where the earlier one comes
        // before the later one in the level-0 file list since the earlier
        // one has a smaller "smallest" key.
        ASSERT_OK(Put("bar", "b"));
        ASSERT_OK(Put("foo", "v1"));
        dbfull()->TEST_CompactMemTable();
        ASSERT_OK(Put("foo", "v2"));
        dbfull()->TEST_CompactMemTable();
        ASSERT_EQ("v2", Get("foo"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_get_ordered_by_levels() {
    todo!();
    /*
    
      do {
        ASSERT_OK(Put("foo", "v1"));
        Compact("a", "z");
        ASSERT_EQ("v1", Get("foo"));
        ASSERT_OK(Put("foo", "v2"));
        ASSERT_EQ("v2", Get("foo"));
        dbfull()->TEST_CompactMemTable();
        ASSERT_EQ("v2", Get("foo"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_get_picks_correct_file() {
    todo!();
    /*
    
      do {
        // Arrange to have multiple files in a non-level-0 level.
        ASSERT_OK(Put("a", "va"));
        Compact("a", "b");
        ASSERT_OK(Put("x", "vx"));
        Compact("x", "y");
        ASSERT_OK(Put("f", "vf"));
        Compact("f", "g");
        ASSERT_EQ("va", Get("a"));
        ASSERT_EQ("vf", Get("f"));
        ASSERT_EQ("vx", Get("x"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_get_encounters_empty_level() {
    todo!();
    /*
    
      do {
        // Arrange for the following to happen:
        //   * sstable A in level 0
        //   * nothing in level 1
        //   * sstable B in level 2
        // Then do enough Get() calls to arrange for an automatic compaction
        // of sstable A.  A bug would cause the compaction to be marked as
        // occurring at level 1 (instead of the correct level 0).

        // Step 1: First place sstables in levels 0 and 2
        int compaction_count = 0;
        while (NumTableFilesAtLevel(0) == 0 || NumTableFilesAtLevel(2) == 0) {
          ASSERT_LE(compaction_count, 100) << "could not fill levels 0 and 2";
          compaction_count++;
          Put("a", "begin");
          Put("z", "end");
          dbfull()->TEST_CompactMemTable();
        }

        // Step 2: clear level 1 if necessary.
        dbfull()->TEST_CompactRange(1, nullptr, nullptr);
        ASSERT_EQ(NumTableFilesAtLevel(0), 1);
        ASSERT_EQ(NumTableFilesAtLevel(1), 0);
        ASSERT_EQ(NumTableFilesAtLevel(2), 1);

        // Step 3: read a bunch of times
        for (int i = 0; i < 1000; i++) {
          ASSERT_EQ("NOT_FOUND", Get("missing"));
        }

        // Step 4: Wait for compaction to finish
        DelayMilliseconds(1000);

        ASSERT_EQ(NumTableFilesAtLevel(0), 0);
      } while (ChangeOptions());

    */
}

#[test] fn db_test_iter_empty() {
    todo!();
    /*
    
      Iterator* iter = db_->NewIterator(ReadOptions());

      iter->SeekToFirst();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      iter->SeekToLast();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      iter->Seek("foo");
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      delete iter;

    */
}

#[test] fn db_test_iter_single() {
    todo!();
    /*
    
      ASSERT_OK(Put("a", "va"));
      Iterator* iter = db_->NewIterator(ReadOptions());

      iter->SeekToFirst();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "(invalid)");
      iter->SeekToFirst();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      iter->SeekToLast();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "(invalid)");
      iter->SeekToLast();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      iter->Seek("");
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      iter->Seek("a");
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      iter->Seek("b");
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      delete iter;

    */
}

#[test] fn db_test_iter_multi() {
    todo!();
    /*
    
      ASSERT_OK(Put("a", "va"));
      ASSERT_OK(Put("b", "vb"));
      ASSERT_OK(Put("c", "vc"));
      Iterator* iter = db_->NewIterator(ReadOptions());

      iter->SeekToFirst();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "b->vb");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "c->vc");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "(invalid)");
      iter->SeekToFirst();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      iter->SeekToLast();
      ASSERT_EQ(IterStatus(iter), "c->vc");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "b->vb");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "(invalid)");
      iter->SeekToLast();
      ASSERT_EQ(IterStatus(iter), "c->vc");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      iter->Seek("");
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Seek("a");
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Seek("ax");
      ASSERT_EQ(IterStatus(iter), "b->vb");
      iter->Seek("b");
      ASSERT_EQ(IterStatus(iter), "b->vb");
      iter->Seek("z");
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      // Switch from reverse to forward
      iter->SeekToLast();
      iter->Prev();
      iter->Prev();
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "b->vb");

      // Switch from forward to reverse
      iter->SeekToFirst();
      iter->Next();
      iter->Next();
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "b->vb");

      // Make sure iter stays at snapshot
      ASSERT_OK(Put("a", "va2"));
      ASSERT_OK(Put("a2", "va3"));
      ASSERT_OK(Put("b", "vb2"));
      ASSERT_OK(Put("c", "vc2"));
      ASSERT_OK(Delete("b"));
      iter->SeekToFirst();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "b->vb");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "c->vc");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "(invalid)");
      iter->SeekToLast();
      ASSERT_EQ(IterStatus(iter), "c->vc");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "b->vb");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      delete iter;

    */
}

#[test] fn db_test_iter_small_and_large_mix() {
    todo!();
    /*
    
      ASSERT_OK(Put("a", "va"));
      ASSERT_OK(Put("b", std::string(100000, 'b')));
      ASSERT_OK(Put("c", "vc"));
      ASSERT_OK(Put("d", std::string(100000, 'd')));
      ASSERT_OK(Put("e", std::string(100000, 'e')));

      Iterator* iter = db_->NewIterator(ReadOptions());

      iter->SeekToFirst();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "b->" + std::string(100000, 'b'));
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "c->vc");
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "d->" + std::string(100000, 'd'));
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "e->" + std::string(100000, 'e'));
      iter->Next();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      iter->SeekToLast();
      ASSERT_EQ(IterStatus(iter), "e->" + std::string(100000, 'e'));
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "d->" + std::string(100000, 'd'));
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "c->vc");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "b->" + std::string(100000, 'b'));
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "a->va");
      iter->Prev();
      ASSERT_EQ(IterStatus(iter), "(invalid)");

      delete iter;

    */
}

#[test] fn db_test_iter_multi_with_delete() {
    todo!();
    /*
    
      do {
        ASSERT_OK(Put("a", "va"));
        ASSERT_OK(Put("b", "vb"));
        ASSERT_OK(Put("c", "vc"));
        ASSERT_OK(Delete("b"));
        ASSERT_EQ("NOT_FOUND", Get("b"));

        Iterator* iter = db_->NewIterator(ReadOptions());
        iter->Seek("c");
        ASSERT_EQ(IterStatus(iter), "c->vc");
        iter->Prev();
        ASSERT_EQ(IterStatus(iter), "a->va");
        delete iter;
      } while (ChangeOptions());

    */
}

#[test] fn db_test_recover() {
    todo!();
    /*
    
      do {
        ASSERT_OK(Put("foo", "v1"));
        ASSERT_OK(Put("baz", "v5"));

        Reopen();
        ASSERT_EQ("v1", Get("foo"));

        ASSERT_EQ("v1", Get("foo"));
        ASSERT_EQ("v5", Get("baz"));
        ASSERT_OK(Put("bar", "v2"));
        ASSERT_OK(Put("foo", "v3"));

        Reopen();
        ASSERT_EQ("v3", Get("foo"));
        ASSERT_OK(Put("foo", "v4"));
        ASSERT_EQ("v4", Get("foo"));
        ASSERT_EQ("v2", Get("bar"));
        ASSERT_EQ("v5", Get("baz"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_recovery_with_empty_log() {
    todo!();
    /*
    
      do {
        ASSERT_OK(Put("foo", "v1"));
        ASSERT_OK(Put("foo", "v2"));
        Reopen();
        Reopen();
        ASSERT_OK(Put("foo", "v3"));
        Reopen();
        ASSERT_EQ("v3", Get("foo"));
      } while (ChangeOptions());

    */
}

/**
  | Check that writes done during a memtable
  | compaction are recovered if the database
  | is shutdown during the memtable compaction.
  |
  */
#[test] fn db_test_recover_during_memtable_compaction() {
    todo!();
    /*
    
      do {
        Options options = CurrentOptions();
        options.env = env_;
        options.write_buffer_size = 1000000;
        Reopen(&options);

        // Trigger a long memtable compaction and reopen the database during it
        ASSERT_OK(Put("foo", "v1"));                         // Goes to 1st log file
        ASSERT_OK(Put("big1", std::string(10000000, 'x')));  // Fills memtable
        ASSERT_OK(Put("big2", std::string(1000, 'y')));      // Triggers compaction
        ASSERT_OK(Put("bar", "v2"));                         // Goes to new log file

        Reopen(&options);
        ASSERT_EQ("v1", Get("foo"));
        ASSERT_EQ("v2", Get("bar"));
        ASSERT_EQ(std::string(10000000, 'x'), Get("big1"));
        ASSERT_EQ(std::string(1000, 'y'), Get("big2"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_minor_compactions_happen() {
    todo!();
    /*
    
      Options options = CurrentOptions();
      options.write_buffer_size = 10000;
      Reopen(&options);

      const int N = 500;

      int starting_num_tables = TotalTableFiles();
      for (int i = 0; i < N; i++) {
        ASSERT_OK(Put(Key(i), Key(i) + std::string(1000, 'v')));
      }
      int ending_num_tables = TotalTableFiles();
      ASSERT_GT(ending_num_tables, starting_num_tables);

      for (int i = 0; i < N; i++) {
        ASSERT_EQ(Key(i) + std::string(1000, 'v'), Get(Key(i)));
      }

      Reopen();

      for (int i = 0; i < N; i++) {
        ASSERT_EQ(Key(i) + std::string(1000, 'v'), Get(Key(i)));
      }

    */
}

#[test] fn db_test_recover_with_large_log() {
    todo!();
    /*
    
      {
        Options options = CurrentOptions();
        Reopen(&options);
        ASSERT_OK(Put("big1", std::string(200000, '1')));
        ASSERT_OK(Put("big2", std::string(200000, '2')));
        ASSERT_OK(Put("small3", std::string(10, '3')));
        ASSERT_OK(Put("small4", std::string(10, '4')));
        ASSERT_EQ(NumTableFilesAtLevel(0), 0);
      }

      // Make sure that if we re-open with a small write buffer size that
      // we flush table files in the middle of a large log file.
      Options options = CurrentOptions();
      options.write_buffer_size = 100000;
      Reopen(&options);
      ASSERT_EQ(NumTableFilesAtLevel(0), 3);
      ASSERT_EQ(std::string(200000, '1'), Get("big1"));
      ASSERT_EQ(std::string(200000, '2'), Get("big2"));
      ASSERT_EQ(std::string(10, '3'), Get("small3"));
      ASSERT_EQ(std::string(10, '4'), Get("small4"));
      ASSERT_GT(NumTableFilesAtLevel(0), 1);

    */
}

#[test] fn db_test_compactions_generate_multiple_files() {
    todo!();
    /*
    
      Options options = CurrentOptions();
      options.write_buffer_size = 100000000;  // Large write buffer
      Reopen(&options);

      Random rnd(301);

      // Write 8MB (80 values, each 100K)
      ASSERT_EQ(NumTableFilesAtLevel(0), 0);
      std::vector<std::string> values;
      for (int i = 0; i < 80; i++) {
        values.push_back(RandomString(&rnd, 100000));
        ASSERT_OK(Put(Key(i), values[i]));
      }

      // Reopening moves updates to level-0
      Reopen(&options);
      dbfull()->TEST_CompactRange(0, nullptr, nullptr);

      ASSERT_EQ(NumTableFilesAtLevel(0), 0);
      ASSERT_GT(NumTableFilesAtLevel(1), 1);
      for (int i = 0; i < 80; i++) {
        ASSERT_EQ(Get(Key(i)), values[i]);
      }

    */
}

#[test] fn db_test_repeated_writes_to_same_key() {
    todo!();
    /*
    
      Options options = CurrentOptions();
      options.env = env_;
      options.write_buffer_size = 100000;  // Small write buffer
      Reopen(&options);

      // We must have at most one file per level except for level-0,
      // which may have up to kL0_StopWritesTrigger files.
      const int kMaxFiles = config::kNumLevels + config::kL0_StopWritesTrigger;

      Random rnd(301);
      std::string value = RandomString(&rnd, 2 * options.write_buffer_size);
      for (int i = 0; i < 5 * kMaxFiles; i++) {
        Put("key", value);
        ASSERT_LE(TotalTableFiles(), kMaxFiles);
        fprintf(stderr, "after %d: %d files\n", i + 1, TotalTableFiles());
      }

    */
}

#[test] fn db_test_sparse_merge() {
    todo!();
    /*
    
      Options options = CurrentOptions();
      options.compression = kNoCompression;
      Reopen(&options);

      FillLevels("A", "Z");

      // Suppose there is:
      //    small amount of data with prefix A
      //    large amount of data with prefix B
      //    small amount of data with prefix C
      // and that recent updates have made small changes to all three prefixes.
      // Check that we do not do a compaction that merges all of B in one shot.
      const std::string value(1000, 'x');
      Put("A", "va");
      // Write approximately 100MB of "B" values
      for (int i = 0; i < 100000; i++) {
        char key[100];
        snprintf(key, sizeof(key), "B%010d", i);
        Put(key, value);
      }
      Put("C", "vc");
      dbfull()->TEST_CompactMemTable();
      dbfull()->TEST_CompactRange(0, nullptr, nullptr);

      // Make sparse update
      Put("A", "va2");
      Put("B100", "bvalue2");
      Put("C", "vc2");
      dbfull()->TEST_CompactMemTable();

      // Compactions should not cause us to create a situation where
      // a file overlaps too much data at the next level.
      ASSERT_LE(dbfull()->TEST_MaxNextLevelOverlappingBytes(), 20 * 1048576);
      dbfull()->TEST_CompactRange(0, nullptr, nullptr);
      ASSERT_LE(dbfull()->TEST_MaxNextLevelOverlappingBytes(), 20 * 1048576);
      dbfull()->TEST_CompactRange(1, nullptr, nullptr);
      ASSERT_LE(dbfull()->TEST_MaxNextLevelOverlappingBytes(), 20 * 1048576);

    */
}

fn between(
        val:  u64,
        low:  u64,
        high: u64) -> bool {
    
    todo!();
        /*
            bool result = (val >= low) && (val <= high);
      if (!result) {
        fprintf(stderr, "Value %llu is not in range [%llu, %llu]\n",
                (unsigned long long)(val), (unsigned long long)(low),
                (unsigned long long)(high));
      }
      return result;
        */
}

#[test] fn db_test_approximate_sizes() {
    todo!();
    /*
    
      do {
        Options options = CurrentOptions();
        options.write_buffer_size = 100000000;  // Large write buffer
        options.compression = kNoCompression;
        DestroyAndReopen();

        ASSERT_TRUE(Between(Size("", "xyz"), 0, 0));
        Reopen(&options);
        ASSERT_TRUE(Between(Size("", "xyz"), 0, 0));

        // Write 8MB (80 values, each 100K)
        ASSERT_EQ(NumTableFilesAtLevel(0), 0);
        const int N = 80;
        static const int S1 = 100000;
        static const int S2 = 105000;  // Allow some expansion from metadata
        Random rnd(301);
        for (int i = 0; i < N; i++) {
          ASSERT_OK(Put(Key(i), RandomString(&rnd, S1)));
        }

        // 0 because GetApproximateSizes() does not account for memtable space
        ASSERT_TRUE(Between(Size("", Key(50)), 0, 0));

        if (options.reuse_logs) {
          // Recovery will reuse memtable, and GetApproximateSizes() does not
          // account for memtable usage;
          Reopen(&options);
          ASSERT_TRUE(Between(Size("", Key(50)), 0, 0));
          continue;
        }

        // Check sizes across recovery by reopening a few times
        for (int run = 0; run < 3; run++) {
          Reopen(&options);

          for (int compact_start = 0; compact_start < N; compact_start += 10) {
            for (int i = 0; i < N; i += 10) {
              ASSERT_TRUE(Between(Size("", Key(i)), S1 * i, S2 * i));
              ASSERT_TRUE(Between(Size("", Key(i) + ".suffix"), S1 * (i + 1),
                                  S2 * (i + 1)));
              ASSERT_TRUE(Between(Size(Key(i), Key(i + 10)), S1 * 10, S2 * 10));
            }
            ASSERT_TRUE(Between(Size("", Key(50)), S1 * 50, S2 * 50));
            ASSERT_TRUE(Between(Size("", Key(50) + ".suffix"), S1 * 50, S2 * 50));

            std::string cstart_str = Key(compact_start);
            std::string cend_str = Key(compact_start + 9);
            Slice cstart = cstart_str;
            Slice cend = cend_str;
            dbfull()->TEST_CompactRange(0, &cstart, &cend);
          }

          ASSERT_EQ(NumTableFilesAtLevel(0), 0);
          ASSERT_GT(NumTableFilesAtLevel(1), 0);
        }
      } while (ChangeOptions());

    */
}

#[test] fn db_test_approximate_sizes_mix_of_small_and_large() {
    todo!();
    /*
    
      do {
        Options options = CurrentOptions();
        options.compression = kNoCompression;
        Reopen();

        Random rnd(301);
        std::string big1 = RandomString(&rnd, 100000);
        ASSERT_OK(Put(Key(0), RandomString(&rnd, 10000)));
        ASSERT_OK(Put(Key(1), RandomString(&rnd, 10000)));
        ASSERT_OK(Put(Key(2), big1));
        ASSERT_OK(Put(Key(3), RandomString(&rnd, 10000)));
        ASSERT_OK(Put(Key(4), big1));
        ASSERT_OK(Put(Key(5), RandomString(&rnd, 10000)));
        ASSERT_OK(Put(Key(6), RandomString(&rnd, 300000)));
        ASSERT_OK(Put(Key(7), RandomString(&rnd, 10000)));

        if (options.reuse_logs) {
          // Need to force a memtable compaction since recovery does not do so.
          ASSERT_OK(dbfull()->TEST_CompactMemTable());
        }

        // Check sizes across recovery by reopening a few times
        for (int run = 0; run < 3; run++) {
          Reopen(&options);

          ASSERT_TRUE(Between(Size("", Key(0)), 0, 0));
          ASSERT_TRUE(Between(Size("", Key(1)), 10000, 11000));
          ASSERT_TRUE(Between(Size("", Key(2)), 20000, 21000));
          ASSERT_TRUE(Between(Size("", Key(3)), 120000, 121000));
          ASSERT_TRUE(Between(Size("", Key(4)), 130000, 131000));
          ASSERT_TRUE(Between(Size("", Key(5)), 230000, 231000));
          ASSERT_TRUE(Between(Size("", Key(6)), 240000, 241000));
          ASSERT_TRUE(Between(Size("", Key(7)), 540000, 541000));
          ASSERT_TRUE(Between(Size("", Key(8)), 550000, 560000));

          ASSERT_TRUE(Between(Size(Key(3), Key(5)), 110000, 111000));

          dbfull()->TEST_CompactRange(0, nullptr, nullptr);
        }
      } while (ChangeOptions());

    */
}

#[test] fn db_test_iterator_pins_ref() {
    todo!();
    /*
    
      Put("foo", "hello");

      // Get iterator that will yield the current contents of the DB.
      Iterator* iter = db_->NewIterator(ReadOptions());

      // Write to force compactions
      Put("foo", "newvalue1");
      for (int i = 0; i < 100; i++) {
        ASSERT_OK(Put(Key(i), Key(i) + std::string(100000, 'v')));  // 100K values
      }
      Put("foo", "newvalue2");

      iter->SeekToFirst();
      ASSERT_TRUE(iter->Valid());
      ASSERT_EQ("foo", iter->key().ToString());
      ASSERT_EQ("hello", iter->value().ToString());
      iter->Next();
      ASSERT_TRUE(!iter->Valid());
      delete iter;

    */
}

#[test] fn db_test_snapshot() {
    todo!();
    /*
    
      do {
        Put("foo", "v1");
        const Snapshot* s1 = db_->GetSnapshot();
        Put("foo", "v2");
        const Snapshot* s2 = db_->GetSnapshot();
        Put("foo", "v3");
        const Snapshot* s3 = db_->GetSnapshot();

        Put("foo", "v4");
        ASSERT_EQ("v1", Get("foo", s1));
        ASSERT_EQ("v2", Get("foo", s2));
        ASSERT_EQ("v3", Get("foo", s3));
        ASSERT_EQ("v4", Get("foo"));

        db_->ReleaseSnapshot(s3);
        ASSERT_EQ("v1", Get("foo", s1));
        ASSERT_EQ("v2", Get("foo", s2));
        ASSERT_EQ("v4", Get("foo"));

        db_->ReleaseSnapshot(s1);
        ASSERT_EQ("v2", Get("foo", s2));
        ASSERT_EQ("v4", Get("foo"));

        db_->ReleaseSnapshot(s2);
        ASSERT_EQ("v4", Get("foo"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_hidden_values_are_removed() {
    todo!();
    /*
    
      do {
        Random rnd(301);
        FillLevels("a", "z");

        std::string big = RandomString(&rnd, 50000);
        Put("foo", big);
        Put("pastfoo", "v");
        const Snapshot* snapshot = db_->GetSnapshot();
        Put("foo", "tiny");
        Put("pastfoo2", "v2");  // Advance sequence number one more

        ASSERT_OK(dbfull()->TEST_CompactMemTable());
        ASSERT_GT(NumTableFilesAtLevel(0), 0);

        ASSERT_EQ(big, Get("foo", snapshot));
        ASSERT_TRUE(Between(Size("", "pastfoo"), 50000, 60000));
        db_->ReleaseSnapshot(snapshot);
        ASSERT_EQ(AllEntriesFor("foo"), "[ tiny, " + big + " ]");
        Slice x("x");
        dbfull()->TEST_CompactRange(0, nullptr, &x);
        ASSERT_EQ(AllEntriesFor("foo"), "[ tiny ]");
        ASSERT_EQ(NumTableFilesAtLevel(0), 0);
        ASSERT_GE(NumTableFilesAtLevel(1), 1);
        dbfull()->TEST_CompactRange(1, nullptr, &x);
        ASSERT_EQ(AllEntriesFor("foo"), "[ tiny ]");

        ASSERT_TRUE(Between(Size("", "pastfoo"), 0, 1000));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_deletion_markers1() {
    todo!();
    /*
    
      Put("foo", "v1");
      ASSERT_OK(dbfull()->TEST_CompactMemTable());
      const int last = config::kMaxMemCompactLevel;
      ASSERT_EQ(NumTableFilesAtLevel(last), 1);  // foo => v1 is now in last level

      // Place a table at level last-1 to prevent merging with preceding mutation
      Put("a", "begin");
      Put("z", "end");
      dbfull()->TEST_CompactMemTable();
      ASSERT_EQ(NumTableFilesAtLevel(last), 1);
      ASSERT_EQ(NumTableFilesAtLevel(last - 1), 1);

      Delete("foo");
      Put("foo", "v2");
      ASSERT_EQ(AllEntriesFor("foo"), "[ v2, DEL, v1 ]");
      ASSERT_OK(dbfull()->TEST_CompactMemTable());  // Moves to level last-2
      ASSERT_EQ(AllEntriesFor("foo"), "[ v2, DEL, v1 ]");
      Slice z("z");
      dbfull()->TEST_CompactRange(last - 2, nullptr, &z);
      // DEL eliminated, but v1 remains because we aren't compacting that level
      // (DEL can be eliminated because v2 hides v1).
      ASSERT_EQ(AllEntriesFor("foo"), "[ v2, v1 ]");
      dbfull()->TEST_CompactRange(last - 1, nullptr, nullptr);
      // Merging last-1 w/ last, so we are the base level for "foo", so
      // DEL is removed.  (as is v1).
      ASSERT_EQ(AllEntriesFor("foo"), "[ v2 ]");

    */
}

#[test] fn db_test_deletion_markers2() {
    todo!();
    /*
    
      Put("foo", "v1");
      ASSERT_OK(dbfull()->TEST_CompactMemTable());
      const int last = config::kMaxMemCompactLevel;
      ASSERT_EQ(NumTableFilesAtLevel(last), 1);  // foo => v1 is now in last level

      // Place a table at level last-1 to prevent merging with preceding mutation
      Put("a", "begin");
      Put("z", "end");
      dbfull()->TEST_CompactMemTable();
      ASSERT_EQ(NumTableFilesAtLevel(last), 1);
      ASSERT_EQ(NumTableFilesAtLevel(last - 1), 1);

      Delete("foo");
      ASSERT_EQ(AllEntriesFor("foo"), "[ DEL, v1 ]");
      ASSERT_OK(dbfull()->TEST_CompactMemTable());  // Moves to level last-2
      ASSERT_EQ(AllEntriesFor("foo"), "[ DEL, v1 ]");
      dbfull()->TEST_CompactRange(last - 2, nullptr, nullptr);
      // DEL kept: "last" file overlaps
      ASSERT_EQ(AllEntriesFor("foo"), "[ DEL, v1 ]");
      dbfull()->TEST_CompactRange(last - 1, nullptr, nullptr);
      // Merging last-1 w/ last, so we are the base level for "foo", so
      // DEL is removed.  (as is v1).
      ASSERT_EQ(AllEntriesFor("foo"), "[ ]");

    */
}

#[test] fn db_test_overlap_in_level0() {
    todo!();
    /*
    
      do {
        ASSERT_EQ(config::kMaxMemCompactLevel, 2) << "Fix test to match config";

        // Fill levels 1 and 2 to disable the pushing of new memtables to levels >
        // 0.
        ASSERT_OK(Put("100", "v100"));
        ASSERT_OK(Put("999", "v999"));
        dbfull()->TEST_CompactMemTable();
        ASSERT_OK(Delete("100"));
        ASSERT_OK(Delete("999"));
        dbfull()->TEST_CompactMemTable();
        ASSERT_EQ("0,1,1", FilesPerLevel());

        // Make files spanning the following ranges in level-0:
        //  files[0]  200 .. 900
        //  files[1]  300 .. 500
        // Note that files are sorted by smallest key.
        ASSERT_OK(Put("300", "v300"));
        ASSERT_OK(Put("500", "v500"));
        dbfull()->TEST_CompactMemTable();
        ASSERT_OK(Put("200", "v200"));
        ASSERT_OK(Put("600", "v600"));
        ASSERT_OK(Put("900", "v900"));
        dbfull()->TEST_CompactMemTable();
        ASSERT_EQ("2,1,1", FilesPerLevel());

        // Compact away the placeholder files we created initially
        dbfull()->TEST_CompactRange(1, nullptr, nullptr);
        dbfull()->TEST_CompactRange(2, nullptr, nullptr);
        ASSERT_EQ("2", FilesPerLevel());

        // Do a memtable compaction.  Before bug-fix, the compaction would
        // not detect the overlap with level-0 files and would incorrectly place
        // the deletion in a deeper level.
        ASSERT_OK(Delete("600"));
        dbfull()->TEST_CompactMemTable();
        ASSERT_EQ("3", FilesPerLevel());
        ASSERT_EQ("NOT_FOUND", Get("600"));
      } while (ChangeOptions());

    */
}

#[test] fn db_test_l0_compaction_bug_issue44_a() {
    todo!();
    /*
    
      Reopen();
      ASSERT_OK(Put("b", "v"));
      Reopen();
      ASSERT_OK(Delete("b"));
      ASSERT_OK(Delete("a"));
      Reopen();
      ASSERT_OK(Delete("a"));
      Reopen();
      ASSERT_OK(Put("a", "v"));
      Reopen();
      Reopen();
      ASSERT_EQ("(a->v)", Contents());
      DelayMilliseconds(1000);  // Wait for compaction to finish
      ASSERT_EQ("(a->v)", Contents());

    */
}

#[test] fn db_test_l0_compaction_bug_issue44_b() {
    todo!();
    /*
    
      Reopen();
      Put("", "");
      Reopen();
      Delete("e");
      Put("", "");
      Reopen();
      Put("c", "cv");
      Reopen();
      Put("", "");
      Reopen();
      Put("", "");
      DelayMilliseconds(1000);  // Wait for compaction to finish
      Reopen();
      Put("d", "dv");
      Reopen();
      Put("", "");
      Reopen();
      Delete("d");
      Delete("b");
      Reopen();
      ASSERT_EQ("(->)(c->cv)", Contents());
      DelayMilliseconds(1000);  // Wait for compaction to finish
      ASSERT_EQ("(->)(c->cv)", Contents());

    */
}

#[test] fn db_test_fflush_issue474() {
    todo!();
    /*
    
      static const int kNum = 100000;
      Random rnd(test::RandomSeed());
      for (int i = 0; i < kNum; i++) {
        fflush(nullptr);
        ASSERT_OK(Put(RandomKey(&rnd), RandomString(&rnd, 100)));
      }

    */
}

#[test] fn db_test_comparator_check() {
    todo!();
    /*
    
      class NewComparator : public Comparator {
       
        const char* Name() const override { return "leveldb.NewComparator"; }
        int Compare(const Slice& a, const Slice& b) const override {
          return BytewiseComparator()->Compare(a, b);
        }
        c_void FindShortestSeparator(std::string* s, const Slice& l) const override {
          BytewiseComparator()->FindShortestSeparator(s, l);
        }
        c_void FindShortSuccessor(std::string* key) const override {
          BytewiseComparator()->FindShortSuccessor(key);
        }
      };
      NewComparator cmp;
      Options new_options = CurrentOptions();
      new_options.comparator = &cmp;
      Status s = TryReopen(&new_options);
      ASSERT_TRUE(!s.ok());
      ASSERT_TRUE(s.ToString().find("comparator") != std::string::npos)
          << s.ToString();

    */
}

#[test] fn db_test_custom_comparator() {
    todo!();
    /*
    
      class NumberComparator : public Comparator {
       
        const char* Name() const override { return "test.NumberComparator"; }
        int Compare(const Slice& a, const Slice& b) const override {
          return ToNumber(a) - ToNumber(b);
        }
        c_void FindShortestSeparator(std::string* s, const Slice& l) const override {
          ToNumber(*s);  // Check format
          ToNumber(l);   // Check format
        }
        c_void FindShortSuccessor(std::string* key) const override {
          ToNumber(*key);  // Check format
        }

       
        static int ToNumber(const Slice& x) {
          // Check that there are no extra characters.
          ASSERT_TRUE(x.size() >= 2 && x[0] == '[' && x[x.size() - 1] == ']')
              << EscapeString(x);
          int val;
          char ignored;
          ASSERT_TRUE(sscanf(x.ToString().c_str(), "[%i]%c", &val, &ignored) == 1)
              << EscapeString(x);
          return val;
        }
      };
      NumberComparator cmp;
      Options new_options = CurrentOptions();
      new_options.create_if_missing = true;
      new_options.comparator = &cmp;
      new_options.filter_policy = nullptr;   // Cannot use bloom filters
      new_options.write_buffer_size = 1000;  // Compact more often
      DestroyAndReopen(&new_options);
      ASSERT_OK(Put("[10]", "ten"));
      ASSERT_OK(Put("[0x14]", "twenty"));
      for (int i = 0; i < 2; i++) {
        ASSERT_EQ("ten", Get("[10]"));
        ASSERT_EQ("ten", Get("[0xa]"));
        ASSERT_EQ("twenty", Get("[20]"));
        ASSERT_EQ("twenty", Get("[0x14]"));
        ASSERT_EQ("NOT_FOUND", Get("[15]"));
        ASSERT_EQ("NOT_FOUND", Get("[0xf]"));
        Compact("[0]", "[9999]");
      }

      for (int run = 0; run < 2; run++) {
        for (int i = 0; i < 1000; i++) {
          char buf[100];
          snprintf(buf, sizeof(buf), "[%d]", i * 10);
          ASSERT_OK(Put(buf, buf));
        }
        Compact("[0]", "[1000000]");
      }

    */
}

#[test] fn db_test_manual_compaction() {
    todo!();
    /*
    
      ASSERT_EQ(config::kMaxMemCompactLevel, 2)
          << "Need to update this test to match kMaxMemCompactLevel";

      MakeTables(3, "p", "q");
      ASSERT_EQ("1,1,1", FilesPerLevel());

      // Compaction range falls before files
      Compact("", "c");
      ASSERT_EQ("1,1,1", FilesPerLevel());

      // Compaction range falls after files
      Compact("r", "z");
      ASSERT_EQ("1,1,1", FilesPerLevel());

      // Compaction range overlaps files
      Compact("p1", "p9");
      ASSERT_EQ("0,0,1", FilesPerLevel());

      // Populate a different range
      MakeTables(3, "c", "e");
      ASSERT_EQ("1,1,2", FilesPerLevel());

      // Compact just the new range
      Compact("b", "f");
      ASSERT_EQ("0,0,2", FilesPerLevel());

      // Compact all
      MakeTables(1, "a", "z");
      ASSERT_EQ("0,1,2", FilesPerLevel());
      db_->CompactRange(nullptr, nullptr);
      ASSERT_EQ("0,0,1", FilesPerLevel());

    */
}

#[test] fn db_test_open_options() {
    todo!();
    /*
    
      std::string dbname = test::TmpDir() + "/db_options_test";
      DestroyDB(dbname, Options());

      // Does not exist, and create_if_missing == false: error
      DB* db = nullptr;
      Options opts;
      opts.create_if_missing = false;
      Status s = DB::Open(opts, dbname, &db);
      ASSERT_TRUE(strstr(s.ToString().c_str(), "does not exist") != nullptr);
      ASSERT_TRUE(db == nullptr);

      // Does not exist, and create_if_missing == true: OK
      opts.create_if_missing = true;
      s = DB::Open(opts, dbname, &db);
      ASSERT_OK(s);
      ASSERT_TRUE(db != nullptr);

      delete db;
      db = nullptr;

      // Does exist, and error_if_exists == true: error
      opts.create_if_missing = false;
      opts.error_if_exists = true;
      s = DB::Open(opts, dbname, &db);
      ASSERT_TRUE(strstr(s.ToString().c_str(), "exists") != nullptr);
      ASSERT_TRUE(db == nullptr);

      // Does exist, and error_if_exists == false: OK
      opts.create_if_missing = true;
      opts.error_if_exists = false;
      s = DB::Open(opts, dbname, &db);
      ASSERT_OK(s);
      ASSERT_TRUE(db != nullptr);

      delete db;
      db = nullptr;

    */
}

#[test] fn db_test_destroy_empty_dir() {
    todo!();
    /*
    
      std::string dbname = test::TmpDir() + "/db_empty_dir";
      TestEnv env(Env::Default());
      env.DeleteDir(dbname);
      ASSERT_TRUE(!env.FileExists(dbname));

      Options opts;
      opts.env = &env;

      ASSERT_OK(env.CreateDir(dbname));
      ASSERT_TRUE(env.FileExists(dbname));
      std::vector<std::string> children;
      ASSERT_OK(env.GetChildren(dbname, &children));
      // The stock Env's do not filter out '.' and '..' special files.
      ASSERT_EQ(2, children.size());
      ASSERT_OK(DestroyDB(dbname, opts));
      ASSERT_TRUE(!env.FileExists(dbname));

      // Should also be destroyed if Env is filtering out dot files.
      env.SetIgnoreDotFiles(true);
      ASSERT_OK(env.CreateDir(dbname));
      ASSERT_TRUE(env.FileExists(dbname));
      ASSERT_OK(env.GetChildren(dbname, &children));
      ASSERT_EQ(0, children.size());
      ASSERT_OK(DestroyDB(dbname, opts));
      ASSERT_TRUE(!env.FileExists(dbname));

    */
}

#[test] fn db_test_destroy_opendb() {
    todo!();
    /*
    
      std::string dbname = test::TmpDir() + "/open_db_dir";
      env_->DeleteDir(dbname);
      ASSERT_TRUE(!env_->FileExists(dbname));

      Options opts;
      opts.create_if_missing = true;
      DB* db = nullptr;
      ASSERT_OK(DB::Open(opts, dbname, &db));
      ASSERT_TRUE(db != nullptr);

      // Must fail to destroy an open db.
      ASSERT_TRUE(env_->FileExists(dbname));
      ASSERT_TRUE(!DestroyDB(dbname, Options()).ok());
      ASSERT_TRUE(env_->FileExists(dbname));

      delete db;
      db = nullptr;

      // Should succeed destroying a closed db.
      ASSERT_OK(DestroyDB(dbname, Options()));
      ASSERT_TRUE(!env_->FileExists(dbname));

    */
}

#[test] fn db_test_locking() {
    todo!();
    /*
    
      DB* db2 = nullptr;
      Status s = DB::Open(CurrentOptions(), dbname_, &db2);
      ASSERT_TRUE(!s.ok()) << "Locking did not prevent re-opening db";

    */
}

/**
  | Check that number of files does not grow
  | when we are out of space
  |
  */
#[test] fn db_test_no_space() {
    todo!();
    /*
    
      Options options = CurrentOptions();
      options.env = env_;
      Reopen(&options);

      ASSERT_OK(Put("foo", "v1"));
      ASSERT_EQ("v1", Get("foo"));
      Compact("a", "z");
      const int num_files = CountFiles();
      // Force out-of-space errors.
      env_->no_space_.store(true, std::memory_order_release);
      for (int i = 0; i < 10; i++) {
        for (int level = 0; level < config::kNumLevels - 1; level++) {
          dbfull()->TEST_CompactRange(level, nullptr, nullptr);
        }
      }
      env_->no_space_.store(false, std::memory_order_release);
      ASSERT_LT(CountFiles(), num_files + 3);

    */
}

#[test] fn db_test_non_writable_file_system() {
    todo!();
    /*
    
      Options options = CurrentOptions();
      options.write_buffer_size = 1000;
      options.env = env_;
      Reopen(&options);
      ASSERT_OK(Put("foo", "v1"));
      // Force errors for new files.
      env_->non_writable_.store(true, std::memory_order_release);
      std::string big(100000, 'x');
      int errors = 0;
      for (int i = 0; i < 20; i++) {
        fprintf(stderr, "iter %d; errors %d\n", i, errors);
        if (!Put("foo", big).ok()) {
          errors++;
          DelayMilliseconds(100);
        }
      }
      ASSERT_GT(errors, 0);
      env_->non_writable_.store(false, std::memory_order_release);

    */
}

#[test] fn db_test_write_sync_error() {
    todo!();
    /*
    
      // Check that log sync errors cause the DB to disallow future writes.

      // (a) Cause log sync calls to fail
      Options options = CurrentOptions();
      options.env = env_;
      Reopen(&options);
      env_->data_sync_error_.store(true, std::memory_order_release);

      // (b) Normal write should succeed
      WriteOptions w;
      ASSERT_OK(db_->Put(w, "k1", "v1"));
      ASSERT_EQ("v1", Get("k1"));

      // (c) Do a sync write; should fail
      w.sync = true;
      ASSERT_TRUE(!db_->Put(w, "k2", "v2").ok());
      ASSERT_EQ("v1", Get("k1"));
      ASSERT_EQ("NOT_FOUND", Get("k2"));

      // (d) make sync behave normally
      env_->data_sync_error_.store(false, std::memory_order_release);

      // (e) Do a non-sync write; should fail
      w.sync = false;
      ASSERT_TRUE(!db_->Put(w, "k3", "v3").ok());
      ASSERT_EQ("v1", Get("k1"));
      ASSERT_EQ("NOT_FOUND", Get("k2"));
      ASSERT_EQ("NOT_FOUND", Get("k3"));

    */
}

#[test] fn db_test_manifest_write_error() {
    todo!();
    /*
    
      // Test for the following problem:
      // (a) Compaction produces file F
      // (b) Log record containing F is written to MANIFEST file, but Sync() fails
      // (c) GC deletes F
      // (d) After reopening DB, reads fail since deleted F is named in log record

      // We iterate twice.  In the second iteration, everything is the
      // same except the log record never makes it to the MANIFEST file.
      for (int iter = 0; iter < 2; iter++) {
        std::atomic<bool>* error_type = (iter == 0) ? &env_->manifest_sync_error_
                                                    : &env_->manifest_write_error_;

        // Insert foo=>bar mapping
        Options options = CurrentOptions();
        options.env = env_;
        options.create_if_missing = true;
        options.error_if_exists = false;
        DestroyAndReopen(&options);
        ASSERT_OK(Put("foo", "bar"));
        ASSERT_EQ("bar", Get("foo"));

        // Memtable compaction (will succeed)
        dbfull()->TEST_CompactMemTable();
        ASSERT_EQ("bar", Get("foo"));
        const int last = config::kMaxMemCompactLevel;
        ASSERT_EQ(NumTableFilesAtLevel(last), 1);  // foo=>bar is now in last level

        // Merging compaction (will fail)
        error_type->store(true, std::memory_order_release);
        dbfull()->TEST_CompactRange(last, nullptr, nullptr);  // Should fail
        ASSERT_EQ("bar", Get("foo"));

        // Recovery: should not lose data
        error_type->store(false, std::memory_order_release);
        Reopen(&options);
        ASSERT_EQ("bar", Get("foo"));
      }

    */
}

#[test] fn db_test_missing_sst_file() {
    todo!();
    /*
    
      ASSERT_OK(Put("foo", "bar"));
      ASSERT_EQ("bar", Get("foo"));

      // Dump the memtable to disk.
      dbfull()->TEST_CompactMemTable();
      ASSERT_EQ("bar", Get("foo"));

      Close();
      ASSERT_TRUE(DeleteAnSSTFile());
      Options options = CurrentOptions();
      options.paranoid_checks = true;
      Status s = TryReopen(&options);
      ASSERT_TRUE(!s.ok());
      ASSERT_TRUE(s.ToString().find("issing") != std::string::npos) << s.ToString();

    */
}

#[test] fn db_test_still_readsst() {
    todo!();
    /*
    
      ASSERT_OK(Put("foo", "bar"));
      ASSERT_EQ("bar", Get("foo"));

      // Dump the memtable to disk.
      dbfull()->TEST_CompactMemTable();
      ASSERT_EQ("bar", Get("foo"));
      Close();
      ASSERT_GT(RenameLDBToSST(), 0);
      Options options = CurrentOptions();
      options.paranoid_checks = true;
      Status s = TryReopen(&options);
      ASSERT_TRUE(s.ok());
      ASSERT_EQ("bar", Get("foo"));

    */
}

#[test] fn db_test_files_deleted_after_compaction() {
    todo!();
    /*
    
      ASSERT_OK(Put("foo", "v2"));
      Compact("a", "z");
      const int num_files = CountFiles();
      for (int i = 0; i < 10; i++) {
        ASSERT_OK(Put("foo", "v2"));
        Compact("a", "z");
      }
      ASSERT_EQ(CountFiles(), num_files);

    */
}

#[test] fn db_test_bloom_filter() {
    todo!();
    /*
    
      env_->count_random_reads_ = true;
      Options options = CurrentOptions();
      options.env = env_;
      options.block_cache = NewLRUCache(0);  // Prevent cache hits
      options.filter_policy = NewBloomFilterPolicy(10);
      Reopen(&options);

      // Populate multiple layers
      const int N = 10000;
      for (int i = 0; i < N; i++) {
        ASSERT_OK(Put(Key(i), Key(i)));
      }
      Compact("a", "z");
      for (int i = 0; i < N; i += 100) {
        ASSERT_OK(Put(Key(i), Key(i)));
      }
      dbfull()->TEST_CompactMemTable();

      // Prevent auto compactions triggered by seeks
      env_->delay_data_sync_.store(true, std::memory_order_release);

      // Lookup present keys.  Should rarely read from small sstable.
      env_->random_read_counter_.Reset();
      for (int i = 0; i < N; i++) {
        ASSERT_EQ(Key(i), Get(Key(i)));
      }
      int reads = env_->random_read_counter_.Read();
      fprintf(stderr, "%d present => %d reads\n", N, reads);
      ASSERT_GE(reads, N);
      ASSERT_LE(reads, N + 2 * N / 100);

      // Lookup present keys.  Should rarely read from either sstable.
      env_->random_read_counter_.Reset();
      for (int i = 0; i < N; i++) {
        ASSERT_EQ("NOT_FOUND", Get(Key(i) + ".missing"));
      }
      reads = env_->random_read_counter_.Read();
      fprintf(stderr, "%d missing => %d reads\n", N, reads);
      ASSERT_LE(reads, 3 * N / 100);

      env_->delay_data_sync_.store(false, std::memory_order_release);
      Close();
      delete options.block_cache;
      delete options.filter_policy;

    */
}
