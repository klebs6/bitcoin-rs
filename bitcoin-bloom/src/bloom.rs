// ---------------- [ File: bitcoin-bloom/src/bloom.rs ]
crate::ix!();

/**
  | BloomFilter is a probabilistic filter
  | which SPV clients provide so that we
  | can filter the transactions we send
  | them.
  | 
  | This allows for significantly more
  | efficient transaction and block downloads.
  | 
  | Because bloom filters are probabilistic,
  | a SPV node can increase the false- positive
  | rate, making us send it transactions
  | which aren't actually its, allowing
  | clients to trade more bandwidth for
  | more privacy by obfuscating which keys
  | are controlled by them.
  |
  */
#[derive(Clone,Default,Serialize,Deserialize)]
pub struct BloomFilter {
    data:         Vec<u8>,
    n_hash_funcs: u32,
    n_tweak:      u32,
    n_flags:      u8,
}

impl BloomFilter {

    /**
      | Creates a new bloom filter which will
      | provide the given fp rate when filled
      | with the given number of elements
      | 
      | -----------
      | @note
      | 
      | if the given parameters will result
      | in a filter outside the bounds of the
      | protocol limits, the filter created
      | will be as close to the given parameters
      | as possible within the protocol limits.
      | 
      | This will apply if nFPRate is very low
      | or nElements is unreasonably high.
      | nTweak is a constant which is added to
      | the seed value passed to the hash function
      | 
      | It should generally always be a random
      | value (and is largely only exposed for
      | unit testing) nFlags should be one of
      | the BLOOM_UPDATE_* enums (not _MASK)
      |
      */
    pub fn new(
        n_elements: u32,
        n_fp_rate:  f64,
        n_tweak_in: u32,
        n_flags_in: u8) -> Self {
    
        /**
          | The ideal size for a bloom filter with
          | a given number of elements and false
          | positive rate is:
          | 
          | - nElements * log(fp rate) / ln(2)^2
          | 
          | We ignore filter parameters which will
          | create a bloom filter larger than the
          | protocol limits
          |
          */
        let cap: usize = {
            let n  = n_elements as f64;
            let lr = n_fp_rate.log10();
            let m0 = -1.0 / LN2SQUARED * n * lr;
            let m1 = MAX_BLOOM_FILTER_SIZE * 8;

            let res = min(m0 as u32, m1 as u32) / 8;

            res.try_into().unwrap()
        };

        /**
          | The ideal number of hash functions is
          | filter size * ln(2) / number of elements
          | 
          | Again, we ignore filter parameters
          | which will create a bloom filter with
          | more hash functions than the protocol
          | limits
          | 
          | See https://en.wikipedia.org/wiki/Bloom_filter
          | for an explanation of these formulas
          |
          */
        let n_hash_funcs = {

            let n = n_elements;

            let cap8 = cap * 8;
            let nln2 = ((n as f64) * LN2) as usize;

            let m0 = (cap8 / nln2) as u32;

            min(m0, MAX_HASH_FUNCS)
        };

        let n_tweak = n_tweak_in;
        let n_flags = n_flags_in;

        Self {
            data: Vec::with_capacity(cap),
            n_hash_funcs,
            n_tweak,
            n_flags,
        }
    }
    
    #[inline] pub fn hash(&self, 
        n_hash_num:   u32,
        data_to_hash: &[u8]) -> u32 {
        
        // 0xFBA4C795 chosen as it guarantees
        // a reasonable bit difference between
        // nHashNum values.
        let x = murmur_hash3(
            n_hash_num * 0xFBA4C795 + self.n_tweak,
            data_to_hash
        );

        let modulus: u32 = (self.data.len() * 8).try_into().unwrap();

        x % modulus
    }

    pub fn insert_key(&mut self, key: &[u8])  {

        if self.data.is_empty() {
            //  Avoid divide-by-zero (CVE-2013-5700)
            return;
        }

        for i in 0..self.n_hash_funcs {

            let n_index:   u32 = self.hash(i,key);
            let pos:     usize = (n_index >> 3).try_into().unwrap();

            //  Sets bit nIndex of vData
            self.data[pos] |= (1 << (7 & n_index));
        }
    }

    pub fn insert_outpoint(&mut self, outpoint: &OutPoint)  {
        
        let mut stream: DataStream 
            = DataStream::new(SER_NETWORK, PROTOCOL_VERSION);

        stream.stream(outpoint);

        self.insert_key(stream.as_slice());
    }

    pub fn contains_key(&self, key: &[u8]) -> bool {
        
        if self.data.is_empty() {

            // Avoid divide-by-zero
            // (CVE-2013-5700)
            return true;
        }

        for i in 0..self.n_hash_funcs {

            let n_index: u32 = self.hash(i,key);

            let pos: usize = (n_index >> 3).try_into().unwrap();

            //  Checks bit nIndex of vData
            if (self.data[pos] & (1 << (7 & n_index))) == 0 {
                return false;
            }
        }

        true
    }

    pub fn contains_outpoint(&self, outpoint: &OutPoint) -> bool {
        
        let mut stream: DataStream 
        = DataStream::new(
            SER_NETWORK.try_into().unwrap(), 
            PROTOCOL_VERSION
        );

        stream.stream(&outpoint);

        self.contains_key(stream.as_slice())
    }

    /**
      | True if the size is <=
      | MAX_BLOOM_FILTER_SIZE and the number of
      | hash functions is <= MAX_HASH_FUNCS (catch
      | a filter which was just deserialized which
      | was too big)
      */
    pub fn is_within_size_constraints(&self) -> bool {
        
        return 
        self.data.len() <= MAX_BLOOM_FILTER_SIZE 
        && self.n_hash_funcs <= MAX_HASH_FUNCS;
    }

    /**
      | Also adds any outputs which match the
      | filter to the filter (to match their
      | spending txes)
      |
      */
    pub fn is_relevant_and_update(&mut self, tx: &Transaction) -> bool {

        let mut found: bool = false;

        // Match if the filter contains the hash
        // of tx for finding tx when they appear
        // in a block
        if self.data.is_empty() {

            // zero-size = "match-all" filter
            return true;
        }

        let hash: &u256 = tx.get_hash();

        if self.contains_key(hash.as_slice()) {
            found = true;
        }

        for i in 0..tx.vout.len() {

            let txout:  &TxOut  = &tx.vout[i];
            let pubkey: &Script = &txout.script_pub_key;

            // Match if the filter contains any
            // arbitrary script data element in
            // any scriptPubKey in tx
            //
            // If this matches, also add the
            // specific output that was matched.
            //
            // This means clients don't have to
            // update the filter themselves when
            // a new relevant tx is discovered in
            // order to find spending
            // transactions, which avoids
            // round-tripping and race conditions.
            let mut pc = pubkey.iter().peekable();

            let mut data = Vec::<u8>::default();

            let mut some_peek: bool = pc.peek().is_some();

            while some_peek {

                let mut opcode = OpcodeType::default();

                if !pubkey.get_op(
                    &mut pc, 
                    &mut opcode, 
                    Some(&mut data)
                ) {
                    break;
                }

                if data.len() != 0 && self.contains_key(&data) {

                    found = true;

                    if (self.n_flags & BloomFlags::BLOOM_UPDATE_MASK as u8) == BloomFlags::BLOOM_UPDATE_ALL as u8 {

                        self.insert_outpoint(&OutPoint::new(hash,i.try_into().unwrap()));

                    } else {

                        if (self.n_flags & BloomFlags::BLOOM_UPDATE_MASK as u8) == BloomFlags::BLOOM_UPDATE_P2PUBKEY_ONLY as u8 {

                            let mut solutions = Vec::<Vec::<u8>>::default();

                            let ty: TxoutType = solver(&pubkey,&mut solutions);

                            if ty == TxoutType::PUBKEY || ty == TxoutType::MULTISIG {
                                self.insert_outpoint(&OutPoint::new(hash,i.try_into().unwrap()));
                            }
                        }
                    }
                    break;
                }

                some_peek = pc.peek().is_some();
            }
        }

        if found {
            return true;
        }

        for txin in tx.vin.iter() {

            // Match if the filter contains an
            // outpoint tx spends
            if self.contains_outpoint(&txin.prevout) {
                return true;
            }

            let mut data = Vec::<u8>::default();

            // Match if the filter contains any
            // arbitrary script data element in
            // any scriptSig in tx
            let mut pc = txin.script_sig.iter().peekable();

            while pc.peek() != None {

                let mut opcode = OpcodeType::default();

                let op = txin.script_sig.get_op(
                    &mut pc, 
                    &mut opcode, 
                    Some(&mut data)
                );

                if !op {
                    break;
                }

                if data.len() != 0 && self.contains_key(data.as_slice()) {
                    return true;
                }
            }
        }

        false
    }
}
