// ---------------- [ File: bitcoinleveldb-key/src/dbformat.rs ]
crate::ix!();

/*
/**
  | An STL comparator that uses a Comparator
  |
  */
pub struct TLLessThan {
    cmp: Box<dyn SliceComparator>,
}

impl Default for TLLessThan {
    
    fn default() -> Self {
        todo!();
        /*
        : cmp(BytewiseComparator()),

        
        */
    }
}

impl TLLessThan {

    pub fn new(c: Box<dyn SliceComparator>) -> Self {
    
        todo!();
        /*
        : cmp(c),

        
        */
    }
    
    pub fn invoke(&self, 
        a: &String,
        b: &String) -> bool {
        
        todo!();
        /*
            return cmp->Compare(Slice(a), Slice(b)) < 0;
        */
    }
}

pub type KVMap = HashMap<String,String,TLLessThan>;
*/

pub type KVMap              = HashMap<String,String>;
pub type KVMapConstIterator<'a> = dyn std::iter::Iterator<Item = (&'a String,&'a String)>;

pub trait Key {

    /**
      | Return the key for the current entry.  The
      | underlying storage for the returned slice is
      | valid only until the next modification of the
      | iterator.
      |
      | REQUIRES: Valid()
      */
    fn key(&self) -> Slice;
}

pub trait Value {

    /**
      | Return the value for the current entry.  The
      | underlying storage for the returned slice is
      | valid only until the next modification of the
      | iterator.
      |
      | REQUIRES: Valid()
      */
    fn value(&self) -> Slice;
}

pub type SequenceNumber = u64;

/**
  | We leave eight bits empty at the bottom
  | so a type and sequence# can be packed
  | together into 64-bits.
  |
  */
pub const MAX_SEQUENCE_NUMBER: SequenceNumber = ((0x1 << 56) - 1);

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/dbformat.h]

/**
  | Value types encoded as the last component of
  | internal keys.
  |
  | DO NOT CHANGE THESE ENUM VALUES: they are
  | embedded in the on-disk data structures.
  */
pub enum ValueType { 
    TypeDeletion = 0x0, 
    TypeValue    = 0x1 
}

/**
  | kValueTypeForSeek defines the ValueType that
  | should be passed when constructing
  | a ParsedInternalKey object for seeking to
  | a particular sequence number (since we sort
  | sequence numbers in decreasing order and the
  | value type is embedded as the low 8 bits in the
  | sequence number in internal keys, we need to
  | use the highest-numbered ValueType, not the
  | lowest).
  */
pub const VALUE_TYPE_FOR_SEEK: ValueType = ValueType::TypeValue;

pub struct ParsedInternalKey {
    user_key: Slice,
    sequence: SequenceNumber,
    ty:       ValueType,
}

impl Default for ParsedInternalKey {
    
    /**
      | Intentionally left uninitialized
      | (for speed)
      |
      */
    fn default() -> Self {
        todo!();
    }
}

impl ParsedInternalKey {

    pub fn new(
        u:   &Slice,
        seq: &SequenceNumber,
        t:   ValueType) -> Self {
    
        todo!();
        /*
        : user_key(u),
        : sequence(seq),
        : ty(t),

        
        */
    }
}

/**
  | Return the length of the encoding of
  | "key".
  |
  */
#[inline] pub fn internal_key_encoding_length(k: &ParsedInternalKey) -> usize {
    
    todo!();
        /*
            return k.user_key.size() + 8;
        */
}

/**
  | Returns the user key portion of an internal
  | key.
  |
  */
#[inline] pub fn extract_user_key(internal_key_: &Slice) -> Slice {
    
    todo!();
        /*
            assert(internal_key.size() >= 8);
      return Slice(internal_key.data(), internal_key.size() - 8);
        */
}

/**
  | A comparator for internal keys that
  | uses a specified comparator for the
  | user key portion and breaks ties by decreasing
  | sequence number.
  |
  */
pub struct InternalKeyComparator {
    user_comparator: Box<dyn SliceComparator>,
}

impl SliceComparator for InternalKeyComparator {

    fn bytewise_comparator(&self) -> *const (dyn bitcoinleveldb_comparator::SliceComparator + 'static) { 
        todo!() 
    }
}

impl Compare for InternalKeyComparator {

    fn compare(&self, 
            a: &Slice,
            b: &Slice) -> i32 {
        self.user_comparator.compare(a,b)
    }
}

impl InternalKeyComparator {

    pub fn new(c: *const dyn SliceComparator) -> Self {
    
        todo!();
        /*
        : user_comparator(c),
        */
    }
    
    pub fn user_comparator(&self) -> *const dyn SliceComparator {
        
        todo!();
        /*
            return user_comparator_;
        */
    }
}

/**
  | Filter policy wrapper that converts
  | from internal keys to user keys
  |
  */
pub struct InternalFilterPolicy {
    user_policy: *const dyn FilterPolicy,
}

impl FilterPolicy for InternalFilterPolicy {

}

impl Name for InternalFilterPolicy {

    fn name(&self) -> *const u8 {
        
        todo!();
        /*
            return user_policy_->Name();
        */
    }
}

impl CreateFilter for InternalFilterPolicy {

    fn create_filter(
        &self, 
        keys: *const Slice,
        n:    i32,
        dst:  &mut Vec<u8>)  {
        
        todo!();
        /*
            // We rely on the fact that the code in table.cc does not mind us
      // adjusting keys[].
      Slice* mkey = const_cast<Slice*>(keys);
      for (int i = 0; i < n; i++) {
        mkey[i] = ExtractUserKey(keys[i]);
        // TODO(sanjay): Suppress dups?
      }
      user_policy_->CreateFilter(keys, n, dst);
        */
    }
}

impl KeyMayMatch for InternalFilterPolicy {

    fn key_may_match(&self, 
        k: &Slice,
        f: &Slice) -> bool {
        
        todo!();
        /*
            return user_policy_->KeyMayMatch(ExtractUserKey(k), f);
        */
    }
}

impl InternalFilterPolicy {

    pub fn new(p: *const dyn FilterPolicy) -> Self {
    
        todo!();
        /*
        : user_policy(p),
        */
    }
}

/**
  | Modules in this directory should keep internal
  | keys wrapped inside the following class instead
  | of plain strings so that we do not incorrectly
  | use string comparisons instead of an
  | InternalKeyComparator.
  */
pub struct InternalKey {
    rep: String,
}

impl Default for InternalKey {
    
    /**
      | Leave rep_ as empty to indicate it is
      | invalid
      |
      */
    fn default() -> Self {
        todo!();
        /*


        
        */
    }
}

impl InternalKey {
    
    pub fn new(
        user_key_: &Slice,
        s:        SequenceNumber,
        t:        ValueType) -> Self {
    
        todo!();
        /*
            AppendInternalKey(&rep_, ParsedInternalKey(user_key, s, t));
        */
    }
    
    pub fn decode_from(&mut self, s: &Slice) -> bool {
        
        todo!();
        /*
            rep_.assign(s.data(), s.size());
        return !rep_.empty();
        */
    }
    
    pub fn encode(&self) -> Slice {
        
        todo!();
        /*
            assert(!rep_.empty());
        return rep_;
        */
    }
    
    pub fn user_key(&self) -> Slice {
        
        todo!();
        /*
            return ExtractUserKey(rep_);
        */
    }
    
    pub fn set_from(&mut self, p: &ParsedInternalKey)  {
        
        todo!();
        /*
            rep_.clear();
        AppendInternalKey(&rep_, p);
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            rep_.clear();
        */
    }
}

/**
  | Attempt to parse an internal key from
  | "internal_key".  On success, stores the parsed
  | data in "*result", and returns true.
  |
  | On error, returns false, leaves "*result" in an
  | undefined state.
  */
#[inline] pub fn parse_internal_key(
        internal_key_: &Slice,
        result:       *mut ParsedInternalKey) -> bool {
    
    todo!();
        /*
            const size_t n = internal_key.size();
      if (n < 8) return false;
      uint64_t num = DecodeFixed64(internal_key.data() + n - 8);
      uint8_t c = num & 0xff;
      result->sequence = num >> 8;
      result->type = static_cast<ValueType>(c);
      result->user_key = Slice(internal_key.data(), n - 8);
      return (c <= static_cast<uint8_t>(kTypeValue));
        */
}

/**
  | A helper class useful for DBImpl::Get()
  |
  */
pub struct LookupKey {

    /**
      | We construct a char array of the form:
      |
      |    klength  varint32               <-- start_
      |    userkey  char[klength]          <-- kstart_
      |    tag      uint64
      |                                    <-- end_
      | The array is a suitable MemTable key.
      |
      | The suffix starting with "userkey" can be
      | used as an InternalKey.
      */
    start:  *const u8,
    kstart: *const u8,
    end:    *const u8,

    /**
      | Avoid allocation for short keys
      |
      */
    space:  [u8; 200],
}

impl Drop for LookupKey {
    fn drop(&mut self) {
        todo!();
        /*
            if (start_ != space_) delete[] start_;
        */
    }
}

impl LookupKey {

    /**
      | Return a key suitable for lookup in a
      | MemTable.
      |
      */
    pub fn memtable_key(&self) -> Slice {
        
        todo!();
        /*
            return Slice(start_, end_ - start_);
        */
    }

    /**
      | Return an internal key (suitable for
      | passing to an internal iterator)
      |
      */
    pub fn internal_key(&self) -> Slice {
        
        todo!();
        /*
            return Slice(kstart_, end_ - kstart_);
        */
    }

    /**
      | Return the user key
      |
      */
    pub fn user_key(&self) -> Slice {
        
        todo!();
        /*
            return Slice(kstart_, end_ - kstart_ - 8);
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/dbformat.cc]

pub fn pack_sequence_and_type(
        seq: u64,
        t:   ValueType) -> u64 {
    
    todo!();
        /*
            assert(seq <= kMaxSequenceNumber);
      assert(t <= kValueTypeForSeek);
      return (seq << 8) | t;
        */
}

/**
  | Append the serialization of "key" to
  | *result.
  |
  */
pub fn append_internal_key(
        result: *mut String,
        k:    &ParsedInternalKey)  {
    
    todo!();
        /*
           result->append(k.user_key.data(), k.user_key.size());
           PutFixed64(result, PackSequenceAndType(k.sequence, k.type));
        */
}

impl ParsedInternalKey {

    pub fn debug_string(&self) -> String {
        
        todo!();
        /*
            std::ostringstream ss;
      ss << '\'' << EscapeString(user_key.ToString()) << "' @ " << sequence << " : "
         << static_cast<int>(type);
      return ss.str();
        */
    }
}

impl InternalKey {

    pub fn debug_string(&self) -> String {
        
        todo!();
        /*
            ParsedInternalKey parsed;
      if (ParseInternalKey(rep_, &parsed)) {
        return parsed.DebugString();
      }
      std::ostringstream ss;
      ss << "(bad)" << EscapeString(rep_);
      return ss.str();
        */
    }
}

impl FindShortSuccessor for InternalKeyComparator {

    fn find_short_successor(&self, k: &mut Vec<u8>)  {

        todo!();
        /*
            Slice user_key = ExtractUserKey(*k);
      std::string tmp(user_key.data(), user_key.size());
      user_comparator_->FindShortSuccessor(&tmp);
      if (tmp.size() < user_key.size() &&
          user_comparator_->Compare(user_key, tmp) < 0) {
        // User key has become shorter physically, but larger logically.
        // Tack on the earliest possible number to the shortened user key.
        PutFixed64(&tmp,
                   PackSequenceAndType(kMaxSequenceNumber, kValueTypeForSeek));
        assert(this->Compare(*k, tmp) < 0);
        k->swap(tmp);
      }
        */
    }

}

impl Name for InternalKeyComparator {

    fn name(&self) -> *const u8 {
        
        todo!();
        /*
            return "leveldb.InternalKeyComparator";
        */
    }
}

impl InternalKeyComparator {
    pub fn compare_slices(&self, 
        akey_: &Slice,
        bkey_: &Slice) -> i32 {
        
        todo!();
        /*
            // Order by:
      //    increasing user key (according to user-supplied comparator)
      //    decreasing sequence number
      //    decreasing type (though sequence# should be enough to disambiguate)
      int r = user_comparator_->Compare(ExtractUserKey(akey), ExtractUserKey(bkey));
      if (r == 0) {
        const uint64_t anum = DecodeFixed64(akey.data() + akey.size() - 8);
        const uint64_t bnum = DecodeFixed64(bkey.data() + bkey.size() - 8);
        if (anum > bnum) {
          r = -1;
        } else if (anum < bnum) {
          r = +1;
        }
      }
      return r;
        */
    }
}

impl FindShortestSeparator for InternalKeyComparator {

    fn find_shortest_separator(
        &self, 
        start: &mut Vec<u8>,
        limit: &[u8]
    ) {
        
        todo!();
        /*
            // Attempt to shorten the user portion of the key
      Slice user_start = ExtractUserKey(*start);
      Slice user_limit = ExtractUserKey(limit);
      std::string tmp(user_start.data(), user_start.size());
      user_comparator_->FindShortestSeparator(&tmp, user_limit);
      if (tmp.size() < user_start.size() &&
          user_comparator_->Compare(user_start, tmp) < 0) {
        // User key has become shorter physically, but larger logically.
        // Tack on the earliest possible number to the shortened user key.
        PutFixed64(&tmp,
                   PackSequenceAndType(kMaxSequenceNumber, kValueTypeForSeek));
        assert(this->Compare(*start, tmp) < 0);
        assert(this->Compare(tmp, limit) < 0);
        start->swap(tmp);
      }
        */
    }
}

impl InternalKeyComparator {
    
    #[inline] pub fn compare_internal_key(&self, 
        a: &InternalKey,
        b: &InternalKey) -> i32 {
        
        todo!();
        /*
            return Compare(a.Encode(), b.Encode());
        */
    }
}


impl LookupKey {
    
    /**
      | Initialize *this for looking up user_key
      | at a snapshot with the specified sequence
      | number.
      |
      */
    pub fn new(
        user_key_: &Slice,
        sequence:  SequenceNumber) -> Self {
    
        todo!();
        /*


            size_t usize = user_key.size();
      size_t needed = usize + 13;  // A conservative estimate
      char* dst;
      if (needed <= sizeof(space_)) {
        dst = space_;
      } else {
        dst = new char[needed];
      }
      start_ = dst;
      dst = EncodeVarint32(dst, usize + 8);
      kstart_ = dst;
      memcpy(dst, user_key.data(), usize);
      dst += usize;
      EncodeFixed64(dst, PackSequenceAndType(s, kValueTypeForSeek));
      dst += 8;
      end_ = dst;
        */
    }
}
