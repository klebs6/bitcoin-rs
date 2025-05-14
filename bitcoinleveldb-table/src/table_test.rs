// ---------------- [ File: bitcoinleveldb-table/src/table_test.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/table_test.cc]

/**
  | Return reverse of "key".
  | 
  | Used to test non-lexicographic comparators.
  |
  */
fn reverse(key_: &Slice) -> String {
    
    todo!();
        /*
            std::string str(key.ToString());
      std::string rev("");
      for (std::string::reverse_iterator rit = str.rbegin(); rit != str.rend();
           ++rit) {
        rev.push_back(*rit);
      }
      return rev;
        */
}

struct ReverseKeyComparator {

}

impl SliceComparator for ReverseKeyComparator {

}

impl Name for ReverseKeyComparator {

    fn name(&self) -> *const u8 {
        
        todo!();
        /*
            return "leveldb.ReverseBytewiseComparator";
        */
    }
}

impl Compare for ReverseKeyComparator {

    fn compare(&self, 
        a: &Slice,
        b: &Slice) -> i32 {
        
        todo!();
        /*
            return BytewiseComparator()->Compare(Reverse(a), Reverse(b));
        */
    }
}

impl FindShortestSeparator for ReverseKeyComparator {

    fn find_shortest_separator(&self, 
        start: *mut String,
        limit: &Slice)  {
        
        todo!();
        /*
            std::string s = Reverse(*start);
        std::string l = Reverse(limit);
        BytewiseComparator()->FindShortestSeparator(&s, l);
        *start = Reverse(s);
        */
    }
}
    
impl FindShortSuccessor for ReverseKeyComparator {

    fn find_short_successor(&self, key_: *mut String)  {
        
        todo!();
        /*
            std::string s = Reverse(*key);
        BytewiseComparator()->FindShortSuccessor(&s);
        *key = Reverse(s);
        */
    }
}

lazy_static!{
    /*
    static ReverseKeyComparator reverse_key_comparator;
    */
}

fn increment(
        cmp: Box<dyn SliceComparator>,
        key_: *mut String)  {
    
    todo!();
        /*
            if (cmp == BytewiseComparator()) {
        key->push_back('\0');
      } else {
        assert(cmp == &reverse_key_comparator);
        std::string rev = Reverse(*key);
        rev.push_back('\0');
        *key = Reverse(rev);
      }
        */
}

///------------------------
struct StringSink {
    contents: String,
}

impl WritableFile for StringSink { }

impl StringSink {

    pub fn contents(&self) -> &String {
        
        todo!();
        /*
            return contents_;
        */
    }
}

impl WritableFileClose for StringSink {

    fn close(&mut self) -> crate::Status {
        
        todo!();
        /*
            return Status::OK();
        */
    }
}
    
impl WritableFileFlush for StringSink {
    fn flush(&mut self) -> crate::Status {
        
        todo!();
        /*
            return Status::OK();
        */
    }
}
    
impl WritableFileSync for StringSink {
    fn sync(&mut self) -> crate::Status {
        
        todo!();
        /*
            return Status::OK();
        */
    }
}
    
impl WritableFileAppend for StringSink {
    fn append(&mut self, data: &Slice) -> crate::Status {
        
        todo!();
        /*
            contents_.append(data.data(), data.size());
        return Status::OK();
        */
    }
}
    
impl GetName for StringSink {
    fn get_name(&self) -> &'static str {
        
        todo!();
        /*
            return "";
        */
    }
}

///---------------------------
struct StringSource {
    contents: String,
}

impl RandomAccessFile for StringSource {

}

impl RandomAccessFileRead for StringSource {

    fn read(&self, 
        offset:  u64,
        n:       usize,
        result:  *mut Slice,
        scratch: *mut u8) -> crate::Status {
        
        todo!();
        /*
            if (offset >= contents_.size()) {
          return Status::InvalidArgument("invalid Read offset");
        }
        if (offset + n > contents_.size()) {
          n = contents_.size() - offset;
        }
        memcpy(scratch, &contents_[offset], n);
        *result = Slice(scratch, n);
        return Status::OK();
        */
    }
}

impl GetName for StringSource {

    fn get_name(&self) -> &'static str {
        
        todo!();
        /*
            return "";
        */
    }
}

impl StringSource {

    pub fn new(contents: &Slice) -> Self {
    
        todo!();
        /*
        : contents(contents.data(), contents.size()),

        
        */
    }
    
    pub fn size(&self) -> u64 {
        
        todo!();
        /*
            return contents_.size();
        */
    }
}
 
///----------------------------
struct BlockConstructor {
    base:       Constructor,
    comparator: Box<dyn SliceComparator>,
    data:       String,
    block:      *mut Block,
}

impl Drop for BlockConstructor {
    fn drop(&mut self) {
        todo!();
        /*
            delete block_;
        */
    }
}

impl BlockConstructor {

    pub fn new(cmp: Box<dyn SliceComparator>) -> Self {
    
        todo!();
        /*
        : constructor(cmp),
        : comparator(cmp),
        : block(nullptr),

        
        */
    }
    
    pub fn finish_impl(&mut self, 
        options: &Options,
        data:    &KVMap) -> crate::Status {
        
        todo!();
        /*
            delete block_;
        block_ = nullptr;
        BlockBuilder builder(&options);

        for (const auto& kvp : data) {
          builder.Add(kvp.first, kvp.second);
        }
        // Open the block
        data_ = builder.Finish().ToString();
        BlockContents contents;
        contents.data = data_;
        contents.cachable = false;
        contents.heap_allocated = false;
        block_ = new Block(contents);
        return Status::OK();
        */
    }
    
    pub fn new_iterator(&self) -> *mut LevelDBIterator {
        
        todo!();
        /*
            return block_->NewIterator(comparator_);
        */
    }
}

///--------------------
struct TableConstructor {
    base:   Constructor,
    source: *mut StringSource,
    table:  *mut Table,
}

impl Drop for TableConstructor {
    fn drop(&mut self) {
        todo!();
        /*
            Reset();
        */
    }
}

impl TableConstructor {

    pub fn new(cmp: Box<dyn SliceComparator>) -> Self {
    
        todo!();
        /*
        : constructor(cmp),
        : source(nullptr),
        : table(nullptr),

        
        */
    }
    
    pub fn finish_impl(&mut self, 
        options: &Options,
        data:    &KVMap) -> crate::Status {
        
        todo!();
        /*
            Reset();
        StringSink sink;
        TableBuilder builder(options, &sink);

        for (const auto& kvp : data) {
          builder.Add(kvp.first, kvp.second);
          ASSERT_TRUE(builder.status().ok());
        }
        Status s = builder.Finish();
        ASSERT_TRUE(s.ok()) << s.ToString();

        ASSERT_EQ(sink.contents().size(), builder.FileSize());

        // Open the table
        source_ = new StringSource(sink.contents());
        Options table_options;
        table_options.comparator = options.comparator;
        return Table::Open(table_options, source_, sink.contents().size(), &table_);
        */
    }
    
    pub fn new_iterator(&self) -> *mut LevelDBIterator {
        
        todo!();
        /*
            return table_->NewIterator(ReadOptions());
        */
    }
    
    pub fn approximate_offset_of(&self, key_: &Slice) -> u64 {
        
        todo!();
        /*
            return table_->ApproximateOffsetOf(key);
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            delete table_;
        delete source_;
        table_ = nullptr;
        source_ = nullptr;
        */
    }
}

/**
  | A helper class that converts internal
  | format keys into user keys
  |
  */
struct KeyConvertingIterator {
    base:   LevelDBIterator,
    status: RefCell<Status>,
    iter:   *mut LevelDBIterator,
}

impl Drop for KeyConvertingIterator {
    fn drop(&mut self) {
        todo!();
        /*
            delete iter_;
        */
    }
}

impl KeyConvertingIterator {

    pub fn new(iter: *mut LevelDBIterator) -> Self {
    
        todo!();
        /*
        : iter(iter),

        
        */
    }
    
    pub fn valid(&self) -> bool {
        
        todo!();
        /*
            return iter_->Valid();
        */
    }
    
    pub fn seek(&mut self, target: &Slice)  {
        
        todo!();
        /*
            ParsedInternalKey ikey(target, kMaxSequenceNumber, kTypeValue);
        std::string encoded;
        AppendInternalKey(&encoded, ikey);
        iter_->Seek(encoded);
        */
    }
    
    pub fn seek_to_first(&mut self)  {
        
        todo!();
        /*
            iter_->SeekToFirst();
        */
    }
    
    pub fn seek_to_last(&mut self)  {
        
        todo!();
        /*
            iter_->SeekToLast();
        */
    }
    
    pub fn next(&mut self)  {
        
        todo!();
        /*
            iter_->Next();
        */
    }
    
    pub fn prev(&mut self)  {
        
        todo!();
        /*
            iter_->Prev();
        */
    }
    
    pub fn key(&self) -> Slice {
        
        todo!();
        /*
            assert(Valid());
        ParsedInternalKey key;
        if (!ParseInternalKey(iter_->key(), &key)) {
          status_ = Status::Corruption("malformed internal key");
          return Slice("corrupted key");
        }
        return key.user_key;
        */
    }
    
    pub fn value(&self) -> Slice {
        
        todo!();
        /*
            return iter_->value();
        */
    }
    
    pub fn status(&self) -> crate::Status {
        
        todo!();
        /*
            return status_.ok() ? iter_->status() : status_;
        */
    }
}
