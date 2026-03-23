// ---------------- [ File: bitcoinleveldbt-util/src/test_type.rs ]
crate::ix!();

#[derive(Debug,Copy,Clone,PartialEq,Hash,Eq)]
pub enum TestType { 
    TABLE_TEST, 
    BLOCK_TEST, 
    MEMTABLE_TEST, 
    DB_TEST 
}

pub struct TestArgs {
    pub ty:               TestType,
    pub reverse_compare:  bool,
    pub restart_interval: i32,
}

lazy_static!{
    /*
    static const TestArgs TestArgList[] = {
        {TABLE_TEST, false, 16},
        {TABLE_TEST, false, 1},
        {TABLE_TEST, false, 1024},
        {TABLE_TEST, true, 16},
        {TABLE_TEST, true, 1},
        {TABLE_TEST, true, 1024},

        {BLOCK_TEST, false, 16},
        {BLOCK_TEST, false, 1},
        {BLOCK_TEST, false, 1024},
        {BLOCK_TEST, true, 16},
        {BLOCK_TEST, true, 1},
        {BLOCK_TEST, true, 1024},

        // Restart interval does not matter for memtables
        {MEMTABLE_TEST, false, 16},
        {MEMTABLE_TEST, true, 16},

        // Do not bother with restart interval variations for DB
        {DB_TEST, false, 16},
        {DB_TEST, true, 16},
    };

    const NUM_TEST_ARGS: i32 = size_of_val(&TEST_ARG_LIST) / size_of_val(&TEST_ARG_LIST[0]);
    */
}
