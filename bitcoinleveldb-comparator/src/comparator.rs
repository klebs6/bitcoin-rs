// ---------------- [ File: bitcoinleveldb-comparator/src/comparator.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/comparator.h]

pub trait Compare {

    /**
      | Three-way comparison.  Returns value:
      |
      |   < 0 iff "a" < "b",
      |
      |   == 0 iff "a" == "b",
      |
      |   > 0 iff "a" > "b"
      */
    fn compare(&self, 
            a: &Slice,
            b: &Slice) -> i32;
}

/*
  | Advanced functions: these are used
  | to reduce the space requirements for
  | internal data structures like index
  | blocks.
  |
  */
pub trait FindShortestSeparator {

    /**
      | If *start < limit, changes *start to a short
      | string in [start,limit).
      |
      | Simple comparator implementations may return
      | with *start unchanged, i.e., an
      | implementation of this method that does
      | nothing is correct.
      */
    fn find_shortest_separator(&self, 
            start: *mut String,
            limit: &Slice);
}

pub trait FindShortSuccessor {

    /**
      | Changes *key to a short string >= *key.
      |
      | Simple comparator implementations may return
      | with *key unchanged,
      |
      | i.e., an implementation of this method that
      | does nothing is correct.
      */
    fn find_short_successor(&self, key_: *mut String);
}

/**
  | A Comparator object provides a total order
  | across slices that are used as keys in an
  | sstable or a database.  A Comparator
  | implementation must be thread-safe since
  | leveldb may invoke its methods concurrently
  | from multiple threads.
  */
pub trait SliceComparator: 
    Compare 

    /*
      | The name of the comparator. Used to check
      | for comparator mismatches (i.e., a
      | DB created with one comparator is accessed
      | using a different comparator.
      | 
      | The client of this package should switch
      | to a new name whenever the comparator
      | implementation changes in a way that
      | will cause the relative ordering of
      | any two keys to change.
      | 
      | Names starting with "leveldb." are
      | reserved and should not be used by any
      | clients of this package.
      |
      */
    + Name 
    + FindShortestSeparator 
    + FindShortSuccessor 
{
    /**
      | Return a builtin comparator that uses
      | lexicographic byte-wise ordering.  The result
      | remains the property of this module and must
      | not be deleted.
      */
    fn bytewise_comparator(&mut self) -> *const dyn SliceComparator {
        
        todo!();
        /*
        
        */
    }
}

///-------------------------------
#[derive(Default)]
pub struct BytewiseComparatorImpl {

}

impl SliceComparator for BytewiseComparatorImpl {

}
    
impl FindShortestSeparator for BytewiseComparatorImpl {

    fn find_shortest_separator(&self, 
        start: *mut String,
        limit: &Slice)  {
        
        todo!();
        /*
            // Find length of common prefix
        size_t min_length = std::min(start->size(), limit.size());
        size_t diff_index = 0;
        while ((diff_index < min_length) &&
               ((*start)[diff_index] == limit[diff_index])) {
          diff_index++;
        }

        if (diff_index >= min_length) {
          // Do not shorten if one string is a prefix of the other
        } else {
          uint8_t diff_byte = static_cast<uint8_t>((*start)[diff_index]);
          if (diff_byte < static_cast<uint8_t>(0xff) &&
              diff_byte + 1 < static_cast<uint8_t>(limit[diff_index])) {
            (*start)[diff_index]++;
            start->resize(diff_index + 1);
            assert(Compare(*start, limit) < 0);
          }
        }
        */
    }
}
    
impl FindShortSuccessor for BytewiseComparatorImpl {

    fn find_short_successor(&self, key_: *mut String)  {
        
        todo!();
        /*
            // Find first character that can be incremented
        size_t n = key->size();
        for (size_t i = 0; i < n; i++) {
          const uint8_t byte = (*key)[i];
          if (byte != static_cast<uint8_t>(0xff)) {
            (*key)[i] = byte + 1;
            key->resize(i + 1);
            return;
          }
        }
        // *key is a run of 0xffs.  Leave it alone.
        */
    }
}

impl Name for BytewiseComparatorImpl {
    
    fn name(&self) -> *const u8 {
        
        todo!();
        /*
            return "leveldb.BytewiseComparator";
        */
    }
}
    
impl Compare for BytewiseComparatorImpl {

    fn compare(&self, 
        a: &Slice,
        b: &Slice) -> i32 {
        
        todo!();
        /*
            return a.compare(b);
        */
    }
}

pub fn bytewise_comparator() -> *const dyn SliceComparator {

    todo!();
    /*
    static NoDestructor<BytewiseComparatorImpl> singleton;
    return singleton.get();
    */
}
