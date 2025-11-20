// ---------------- [ File: bitcoinleveldb-env/src/handle_dump_command.rs ]
crate::ix!();

pub fn handle_dump_command(
        env:   Rc<RefCell<dyn Env>>,
        files: *mut *mut u8,
        num:   i32) -> bool {
    
    todo!();
        /*
            StdoutPrinter printer;
      bool ok = true;
      for (int i = 0; i < num; i++) {
        Status s = DumpFile(env, files[i], &printer);
        if (!s.ok()) {
          fprintf(stderr, "%s\n", s.ToString().c_str());
          ok = false;
        }
      }
      return ok;
        */
}

pub fn dbleveldbutil_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            leveldb::Env* env = leveldb::Env::Default();
      bool ok = true;
      if (argc < 2) {
        Usage();
        ok = false;
      } else {
        std::string command = argv[1];
        if (command == "dump") {
          ok = leveldb::HandleDumpCommand(env, argv + 2, argc - 2);
        } else {
          Usage();
          ok = false;
        }
      }
      return (ok ? 0 : 1);
        */
}
