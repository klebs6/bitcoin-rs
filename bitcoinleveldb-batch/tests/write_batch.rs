// ---------------- [ File: bitcoinleveldb-batch/tests/write_batch.rs ]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/write_batch_test.cc]

use bitcoinleveldb_batch::*;
use bitcoinleveldb_iterator::*;
use bitcoinleveldb_iteratorinner::*;
use bitcoin_imports::*;

fn print_contents(b: *mut WriteBatch) -> String {

    trace!("print_contents: batch_ptr={:p}", b);

    assert!(!b.is_null(), "print_contents: batch pointer must not be null");

    let user_cmp_ptr = bitcoinleveldb_key::null_slice_comparator();
    let cmp = bitcoinleveldb_key::InternalKeyComparator::new(user_cmp_ptr);

    let mem_val = bitcoinleveldb_memtable::MemTable::new(&cmp);
    let mut mem_box = Box::new(mem_val);
    mem_box.ref_();
    let mem_ptr: *mut bitcoinleveldb_memtable::MemTable = Box::into_raw(mem_box);

    let mut state = String::new();

    let s = write_batch_internal::insert_into(b as *const WriteBatch, mem_ptr);

    let mut count: i32 = 0;

    let iter_ptr: *mut LevelDBIterator = unsafe { (*mem_ptr).new_iterator() };
    assert!(
        !iter_ptr.is_null(),
        "print_contents: iterator pointer must not be null"
    );

    unsafe {
        (*iter_ptr).seek_to_first();
        while (*iter_ptr).valid() {
            let k = (*iter_ptr).key();

            let mut ikey = bitcoinleveldb_key::ParsedInternalKey::default();
            let ok = bitcoinleveldb_key::parse_internal_key(&k, &mut ikey as *mut _);
            assert!(ok);

            match *ikey.ty() {
                bitcoinleveldb_key::ValueType::TypeValue => {
                    state.push_str("Put(");
                    state.push_str(&ikey.user_key().to_string());
                    state.push_str(", ");
                    state.push_str(&(*iter_ptr).value().to_string());
                    state.push_str(")");
                    count += 1;
                }
                bitcoinleveldb_key::ValueType::TypeDeletion => {
                    state.push_str("Delete(");
                    state.push_str(&ikey.user_key().to_string());
                    state.push_str(")");
                    count += 1;
                }
            }

            state.push_str("@");
            state.push_str(&ikey.sequence().to_string());

            (*iter_ptr).next();
        }
    }

    unsafe {
        drop(Box::from_raw(iter_ptr));
    }

    if !s.is_ok() {
        state.push_str("ParseError()");
    } else if count != write_batch_internal::count(b as *const WriteBatch) {
        state.push_str("CountMismatch()");
    }

    unsafe {
        (*mem_ptr).unref();
    }

    state

        /*
            InternalKeyComparator cmp(BytewiseComparator());
      MemTable* mem = new MemTable(cmp);
      mem->Ref();
      std::string state;
      Status s = WriteBatchInternal::InsertInto(b, mem);
      int count = 0;
      Iterator* iter = mem->NewIterator();
      for (iter->SeekToFirst(); iter->Valid(); iter->Next()) {
        ParsedInternalKey ikey;
        ASSERT_TRUE(ParseInternalKey(iter->key(), &ikey));
        switch (ikey.type) {
          case kTypeValue:
            state.append("Put(");
            state.append(ikey.user_key.ToString());
            state.append(", ");
            state.append(iter->value().ToString());
            state.append(")");
            count++;
            break;
          case kTypeDeletion:
            state.append("Delete(");
            state.append(ikey.user_key.ToString());
            state.append(")");
            count++;
            break;
        }
        state.append("@");
        state.append(NumberToString(ikey.sequence));
      }
      delete iter;
      if (!s.ok()) {
        state.append("ParseError()");
      } else if (count != WriteBatchInternal::Count(b)) {
        state.append("CountMismatch()");
      }
      mem->Unref();
      return state;
        */
}

#[traced_test]
fn write_batch_test_empty() {

    trace!("write_batch_test_empty: begin");

    let mut batch = WriteBatch::new();
    assert_eq!("", print_contents(&mut batch as *mut WriteBatch));
    assert_eq!(0, write_batch_internal::count(&batch as *const WriteBatch));

    trace!("write_batch_test_empty: end");

    /*
    
      WriteBatch batch;
      ASSERT_EQ("", PrintContents(&batch));
      ASSERT_EQ(0, WriteBatchInternal::Count(&batch));

    */
}

#[traced_test]
fn write_batch_test_multiple() {

    trace!("write_batch_test_multiple: begin");

    let mut batch = WriteBatch::new();

    let kfoo = bitcoinleveldb_slice::Slice::from("foo");
    let vbar = bitcoinleveldb_slice::Slice::from("bar");
    batch.put(&kfoo, &vbar);

    let kbox = bitcoinleveldb_slice::Slice::from("box");
    batch.delete(&kbox);

    let kbaz = bitcoinleveldb_slice::Slice::from("baz");
    let vboo = bitcoinleveldb_slice::Slice::from("boo");
    batch.put(&kbaz, &vboo);

    write_batch_internal::set_sequence(&mut batch as *mut WriteBatch, 100);

    assert_eq!(100, write_batch_internal::sequence(&batch as *const WriteBatch));
    assert_eq!(3, write_batch_internal::count(&batch as *const WriteBatch));

    assert_eq!(
        "Put(baz, boo)@102Delete(box)@101Put(foo, bar)@100",
        print_contents(&mut batch as *mut WriteBatch)
    );

    trace!("write_batch_test_multiple: end");

    /*
    
      WriteBatch batch;
      batch.Put(Slice("foo"), Slice("bar"));
      batch.Delete(Slice("box"));
      batch.Put(Slice("baz"), Slice("boo"));
      WriteBatchInternal::SetSequence(&batch, 100);
      ASSERT_EQ(100, WriteBatchInternal::Sequence(&batch));
      ASSERT_EQ(3, WriteBatchInternal::Count(&batch));
      ASSERT_EQ(
          "Put(baz, boo)@102"
          "Delete(box)@101"
          "Put(foo, bar)@100",
          PrintContents(&batch));

    */
}

#[traced_test]
fn write_batch_test_corruption() {

    trace!("write_batch_test_corruption: begin");

    let mut batch = WriteBatch::new();

    let kfoo = bitcoinleveldb_slice::Slice::from("foo");
    let vbar = bitcoinleveldb_slice::Slice::from("bar");
    batch.put(&kfoo, &vbar);

    let kbox = bitcoinleveldb_slice::Slice::from("box");
    batch.delete(&kbox);

    write_batch_internal::set_sequence(&mut batch as *mut WriteBatch, 200);

    let contents = write_batch_internal::contents(&batch as *const WriteBatch);

    let truncated = unsafe {
        bitcoinleveldb_slice::Slice::from_ptr_len(
            *contents.data(),
            (*contents.size()).saturating_sub(1),
        )
    };

    write_batch_internal::set_contents(&mut batch as *mut WriteBatch, &truncated);

    assert_eq!(
        "Put(foo, bar)@200ParseError()",
        print_contents(&mut batch as *mut WriteBatch)
    );

    trace!("write_batch_test_corruption: end");

    /*
    
      WriteBatch batch;
      batch.Put(Slice("foo"), Slice("bar"));
      batch.Delete(Slice("box"));
      WriteBatchInternal::SetSequence(&batch, 200);
      Slice contents = WriteBatchInternal::Contents(&batch);
      WriteBatchInternal::SetContents(&batch,
                                      Slice(contents.data(), contents.size() - 1));
      ASSERT_EQ(
          "Put(foo, bar)@200"
          "ParseError()",
          PrintContents(&batch));

    */
}

#[traced_test]
fn write_batch_test_append() {

    trace!("write_batch_test_append: begin");

    let mut b1 = WriteBatch::new();
    let mut b2 = WriteBatch::new();

    write_batch_internal::set_sequence(&mut b1 as *mut WriteBatch, 200);
    write_batch_internal::set_sequence(&mut b2 as *mut WriteBatch, 300);

    b1.append(&b2);
    assert_eq!("", print_contents(&mut b1 as *mut WriteBatch));

    let ka = bitcoinleveldb_slice::Slice::from("a");
    let kva = bitcoinleveldb_slice::Slice::from("va");
    b2.put(&ka, &kva);

    b1.append(&b2);
    assert_eq!("Put(a, va)@200", print_contents(&mut b1 as *mut WriteBatch));

    b2.clear();

    let kb = bitcoinleveldb_slice::Slice::from("b");
    let kvb = bitcoinleveldb_slice::Slice::from("vb");
    b2.put(&kb, &kvb);

    b1.append(&b2);
    assert_eq!(
        "Put(a, va)@200Put(b, vb)@201",
        print_contents(&mut b1 as *mut WriteBatch)
    );

    let kfoo = bitcoinleveldb_slice::Slice::from("foo");
    b2.delete(&kfoo);

    b1.append(&b2);
    assert_eq!(
        "Put(a, va)@200Put(b, vb)@202Put(b, vb)@201Delete(foo)@203",
        print_contents(&mut b1 as *mut WriteBatch)
    );

    trace!("write_batch_test_append: end");

    /*
    
      WriteBatch b1, b2;
      WriteBatchInternal::SetSequence(&b1, 200);
      WriteBatchInternal::SetSequence(&b2, 300);
      b1.Append(b2);
      ASSERT_EQ("", PrintContents(&b1));
      b2.Put("a", "va");
      b1.Append(b2);
      ASSERT_EQ("Put(a, va)@200", PrintContents(&b1));
      b2.Clear();
      b2.Put("b", "vb");
      b1.Append(b2);
      ASSERT_EQ(
          "Put(a, va)@200"
          "Put(b, vb)@201",
          PrintContents(&b1));
      b2.Delete("foo");
      b1.Append(b2);
      ASSERT_EQ(
          "Put(a, va)@200"
          "Put(b, vb)@202"
          "Put(b, vb)@201"
          "Delete(foo)@203",
          PrintContents(&b1));

    */
}

#[traced_test]
fn write_batch_test_approximate_size() {

    trace!("write_batch_test_approximate_size: begin");

    let mut batch = WriteBatch::new();
    let empty_size = batch.approximate_size();

    let kfoo = bitcoinleveldb_slice::Slice::from("foo");
    let vbar = bitcoinleveldb_slice::Slice::from("bar");
    batch.put(&kfoo, &vbar);

    let one_key_size = batch.approximate_size();
    assert!(empty_size < one_key_size);

    let kbaz = bitcoinleveldb_slice::Slice::from("baz");
    let vboo = bitcoinleveldb_slice::Slice::from("boo");
    batch.put(&kbaz, &vboo);

    let two_keys_size = batch.approximate_size();
    assert!(one_key_size < two_keys_size);

    let kbox = bitcoinleveldb_slice::Slice::from("box");
    batch.delete(&kbox);

    let post_delete_size = batch.approximate_size();
    assert!(two_keys_size < post_delete_size);

    trace!("write_batch_test_approximate_size: end");

    /*
    
      WriteBatch batch;
      size_t empty_size = batch.ApproximateSize();

      batch.Put(Slice("foo"), Slice("bar"));
      size_t one_key_size = batch.ApproximateSize();
      ASSERT_LT(empty_size, one_key_size);

      batch.Put(Slice("baz"), Slice("boo"));
      size_t two_keys_size = batch.ApproximateSize();
      ASSERT_LT(one_key_size, two_keys_size);

      batch.Delete(Slice("box"));
      size_t post_delete_size = batch.ApproximateSize();
      ASSERT_LT(two_keys_size, post_delete_size);

    */
}
