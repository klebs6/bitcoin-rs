// ---------------- [ File: bitcoinleveldb-repair/src/repair.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/repair.cc]

/**
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
    tables:           Vec<RepairerTableInfo>,
    next_file_number: u64,
}
