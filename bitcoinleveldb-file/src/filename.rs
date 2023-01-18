/*!
  | File names used by DB code
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/filename.h]

pub enum FileType {
    LogFile,
    DBLockFile,
    TableFile,
    DescriptorFile,
    CurrentFile,
    TempFile,
    InfoLogFile  // Either the current one, or an old one
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/filename.cc]

pub fn make_file_name(
        dbname: &String,
        number: u64,
        suffix: *const u8) -> String {
    
    todo!();
        /*
            char buf[100];
      snprintf(buf, sizeof(buf), "/%06llu.%s",
               static_cast<unsigned long long>(number), suffix);
      return dbname + buf;
        */
}

/**
  | Return the name of the log file with the
  | specified number in the db named by "dbname".
  | 
  | The result will be prefixed with "dbname".
  |
  */
pub fn log_file_name(
        dbname: &String,
        number: u64) -> String {
    
    todo!();
        /*
            assert(number > 0);
      return MakeFileName(dbname, number, "log");
        */
}

/**
  | Return the name of the sstable with the
  | specified number in the db named by "dbname".
  | 
  | The result will be prefixed with "dbname".
  |
  */
pub fn table_file_name(
        dbname: &String,
        number: u64) -> String {
    
    todo!();
        /*
            assert(number > 0);
      return MakeFileName(dbname, number, "ldb");
        */
}

/**
  | Return the legacy file name for an sstable with
  | the specified number in the db named by
  | "dbname". The result will be prefixed with
  | "dbname".
  */
pub fn sst_table_file_name(
        dbname: &String,
        number: u64) -> String {
    
    todo!();
        /*
            assert(number > 0);
      return MakeFileName(dbname, number, "sst");
        */
}

/**
  | Return the name of the descriptor file for the
  | db named by "dbname" and the specified
  | incarnation number.  The result will be
  | prefixed with "dbname".
  */
pub fn descriptor_file_name(
        dbname: &String,
        number: u64) -> String {
    
    todo!();
        /*
            assert(number > 0);
      char buf[100];
      snprintf(buf, sizeof(buf), "/MANIFEST-%06llu",
               static_cast<unsigned long long>(number));
      return dbname + buf;
        */
}

/**
  | Return the name of the current file.
  | This file contains the name of the current
  | manifest file.
  | 
  | The result will be prefixed with "dbname".
  |
  */
pub fn current_file_name(dbname: &String) -> String {
    
    todo!();
        /*
            return dbname + "/CURRENT";
        */
}

/**
  | Return the name of the lock file for the
  | db named by "dbname". The result will
  | be prefixed with "dbname".
  |
  */
pub fn lock_file_name(dbname: &String) -> String {
    
    todo!();
        /*
            return dbname + "/LOCK";
        */
}

/**
  | Return the name of a temporary file owned by
  | the db named "dbname".
  |
  | The result will be prefixed with "dbname".
  */
pub fn temp_file_name(
        dbname: &String,
        number: u64) -> String {
    
    todo!();
        /*
            assert(number > 0);
      return MakeFileName(dbname, number, "dbtmp");
        */
}

/**
  | Return the name of the info log file for
  | "dbname".
  |
  */
pub fn info_log_file_name(dbname: &String) -> String {
    
    todo!();
        /*
            return dbname + "/LOG";
        */
}

/**
  | Return the name of the old info log file
  | for "dbname".
  |
  */
pub fn old_info_log_file_name(dbname: &String) -> String {
    
    todo!();
        /*
            return dbname + "/LOG.old";
        */
}

/**
  | If filename is a leveldb file, store the type
  | of the file in *type.
  |
  | The number encoded in the filename is stored in
  | *number.  If the filename was successfully
  | parsed, returns true.  Else return false.
  |
  ----------------------
  | Owned filenames have the form:
  |    dbname/CURRENT
  |    dbname/LOCK
  |    dbname/LOG
  |    dbname/LOG.old
  |    dbname/MANIFEST-[0-9]+
  |    dbname/[0-9]+.(log|sst|ldb)
  */
pub fn parse_file_name(
        filename: &String,
        number:   *mut u64,
        ty:       *mut FileType) -> bool {
    
    todo!();
        /*
            Slice rest(filename);
      if (rest == "CURRENT") {
        *number = 0;
        *type = kCurrentFile;
      } else if (rest == "LOCK") {
        *number = 0;
        *type = kDBLockFile;
      } else if (rest == "LOG" || rest == "LOG.old") {
        *number = 0;
        *type = kInfoLogFile;
      } else if (rest.starts_with("MANIFEST-")) {
        rest.remove_prefix(strlen("MANIFEST-"));
        uint64_t num;
        if (!ConsumeDecimalNumber(&rest, &num)) {
          return false;
        }
        if (!rest.empty()) {
          return false;
        }
        *type = kDescriptorFile;
        *number = num;
      } else {
        // Avoid strtoull() to keep filename format independent of the
        // current locale
        uint64_t num;
        if (!ConsumeDecimalNumber(&rest, &num)) {
          return false;
        }
        Slice suffix = rest;
        if (suffix == Slice(".log")) {
          *type = kLogFile;
        } else if (suffix == Slice(".sst") || suffix == Slice(".ldb")) {
          *type = kTableFile;
        } else if (suffix == Slice(".dbtmp")) {
          *type = kTempFile;
        } else {
          return false;
        }
        *number = num;
      }
      return true;
        */
}

