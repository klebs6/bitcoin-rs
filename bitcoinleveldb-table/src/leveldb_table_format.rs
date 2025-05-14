// ---------------- [ File: bitcoinleveldb-table/src/leveldb_table_format.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/format.h]

/**
  | TableMagicNumber was picked by running
  | echo http://code.google.com/p/leveldb/ | sha1sum 
  | and taking the leading 64 bits.
  |
  */
pub const TABLE_MAGIC_NUMBER: u64 = 0xdb4775248b80fb57;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/format.cc]

/**
  | Read the block identified by "handle"
  | from "file". On failure return non-OK.
  | On success fill *result and return OK.
  |
  */
pub fn read_block(
        file:    Rc<RefCell<dyn RandomAccessFile>>,
        options: &ReadOptions,
        handle:  &BlockHandle,
        result:  *mut BlockContents) -> crate::Status {
    
    todo!();
        /*
            result->data = Slice();
      result->cachable = false;
      result->heap_allocated = false;

      // Read the block contents as well as the type/crc footer.
      // See table_builder.cc for the code that built this structure.
      size_t n = static_cast<size_t>(handle.size());
      char* buf = new char[n + kBlockTrailerSize];
      Slice contents;
      Status s = file->Read(handle.offset(), n + kBlockTrailerSize, &contents, buf);
      if (!s.ok()) {
        delete[] buf;
        return s;
      }
      if (contents.size() != n + kBlockTrailerSize) {
        delete[] buf;
        return Status::Corruption("truncated block read", file->GetName());
      }

      // Check the crc of the type and the block contents
      const char* data = contents.data();  // Pointer to where Read put the data
      if (options.verify_checksums) {
        const uint32_t crc = crc32c::Unmask(DecodeFixed32(data + n + 1));
        const uint32_t actual = crc32c::Value(data, n + 1);
        if (actual != crc) {
          delete[] buf;
          s = Status::Corruption("block checksum mismatch", file->GetName());
          return s;
        }
      }

      switch (data[n]) {
        case kNoCompression:
          if (data != buf) {
            // File implementation gave us pointer to some other data.
            // Use it directly under the assumption that it will be live
            // while the file is open.
            delete[] buf;
            result->data = Slice(data, n);
            result->heap_allocated = false;
            result->cachable = false;  // Do not double-cache
          } else {
            result->data = Slice(buf, n);
            result->heap_allocated = true;
            result->cachable = true;
          }

          // Ok
          break;
        case kSnappyCompression: {
          size_t ulength = 0;
          if (!Snappy_GetUncompressedLength(data, n, &ulength)) {
            delete[] buf;
            return Status::Corruption("corrupted compressed block contents", file->GetName());
          }
          char* ubuf = new char[ulength];
          if (!Snappy_Uncompress(data, n, ubuf)) {
            delete[] buf;
            delete[] ubuf;
            return Status::Corruption("corrupted compressed block contents", file->GetName());
          }
          delete[] buf;
          result->data = Slice(ubuf, ulength);
          result->heap_allocated = true;
          result->cachable = true;
          break;
        }
        default:
          delete[] buf;
          return Status::Corruption("bad block type", file->GetName());
      }

      return Status::OK();
        */
}
