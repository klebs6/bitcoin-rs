// ---------------- [ File: bitcoin-addrman/src/serialize.rs ]
crate::ix!();

impl AddrManImpl {
    
    /**
     | Serialized format.
     |
     | - format version byte (@see `Format`)
     |
     | - lowest compatible format version
     |   byte. This is used to help old software
     |   decide whether to parse the file. For
     |   example:
     |   - Bitcoin Core version N knows how to
     |     parse up to format=3. If a new format=4
     |     is introduced in version N+1 that is
     |     compatible with format=3 and it is
     |     known that version N will be able to
     |     parse it, then version N+1 will write
     |     (format=4, lowest_compatible=3) in the
     |     first two bytes of the file, and so
     |     version N will still try to parse it.
     |   - Bitcoin Core version N+2 introduces
     |     a new incompatible format=5. It will
     |     write (format=5, lowest_compatible=5)
     |     and so any versions that do not know
     |     how to parse format=5 will not try to
     |     read the file.
     |
     | - nKey
     | - n_new
     | - nTried
     | - number of "new" buckets XOR 2**30
     | - all new addresses (total count: n_new)
     | - all tried addresses (total count: nTried)
     | - for each new bucket:
     |   - number of elements
     |   - for each element: index in the
     |   serialized "all new addresses"
     |
     | - asmap checksum
     |
     | 2**30 is xorred with the number of buckets
     | to make addrman deserializer v0 detect it
     | as incompatible. This is necessary because
     | it did not check the version number on
     | deserialization.
     |
     | vvNew, vvTried, mapInfo, mapAddr and
     | vRandom are never encoded explicitly; they
     | are instead reconstructed from the other
     | information.
     |
     | This format is more complex, but
     | significantly smaller (at most 1.5 MiB),
     | and supports changes to the ADDRMAN_
     | parameters without breaking the on-disk
     | structure.
     |
     | We don't use SERIALIZE_METHODS since the
     | serialization and deserialization code has
     | very little in common.
     */
    pub fn serialize<Stream: GetVersion + GetType>(&self, stream: &mut Stream)  {

        let inner = self.cs.lock();

        // Always serialize in the latest version
        // (FILE_FORMAT).
        let mut s: OverrideStream::<Stream> = OverrideStream::<Stream>::new(
            stream, 
            stream.get_type(), 
            stream.get_version() | ADDRV2_FORMAT
        );

        s.stream(ADDR_MAN_FILE_FORMAT as u8);

        // Increment `lowest_compatible` iff
        // a newly introduced format is
        // incompatible with the previous one.
        const lowest_compatible: u8 = AddrManFormat::V3_BIP155 as u8;

        s.stream((ADDR_MAN_INCOMPATIBILITY_BASE + lowest_compatible) as u8);

        s.stream(&self.n_key);
        s.stream(&inner.n_new);
        s.stream(&inner.n_tried);

        let n_ubuckets: i32 = (ADDRMAN_NEW_BUCKET_COUNT ^ (1 << 30)).try_into().unwrap();

        s.stream(&n_ubuckets);

        let mut map_unk_ids: HashMap<i32,RefCell<i32>> = HashMap::<i32,RefCell<i32>>::default();
        let mut n_ids:       i32 = 0;

        for entry in &inner.map_info {

            *map_unk_ids[entry.0].borrow_mut() = n_ids;

            let info = &entry.1;

            if info.n_ref_count != 0 {
                assert!{n_ids != inner.n_new}; // this means n_new was wrong, oh ow
                s.stream(&info);
                n_ids += 1;
            }
        }

        n_ids = 0;

        for entry in &inner.map_info {

            let info = &entry.1;

            if info.in_tried {

                // this means nTried was wrong, oh ow
                assert!{n_ids != inner.n_tried}; 
                s.stream(&info);
                n_ids += 1;
            }
        }

        for bucket in 0..ADDRMAN_NEW_BUCKET_COUNT {
            let mut n_size = 0;

            for i in 0..ADDRMAN_BUCKET_SIZE {
                if inner.vv_new[bucket][i] != -1 {
                    n_size += 1;
                }
            }

            s.stream(&n_size);

            for i in 0..ADDRMAN_BUCKET_SIZE {

                if inner.vv_new[bucket][i] != -1 {
                    let n_index: i32 = *map_unk_ids[&inner.vv_new[bucket][i]].borrow();
                    s.stream(&n_index);
                }
            }
        }

        /*
          | Store asmap checksum after bucket
          | entries so that it can be ignored
          | by older clients for backward
          | compatibility.
          */
        let mut asmap_checksum: u256 = u256::default();

        if self.asmap.len() != 0 {
            asmap_checksum = serialize_hash(&self.asmap, None, None);
        }

        s.stream(&asmap_checksum);
    }
}
