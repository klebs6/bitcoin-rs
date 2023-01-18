crate::ix!();

/**
  | Warning: be very careful when changing this!
  | assumeutxo and UTXO snapshot validation
  | commitments are reliant on the hash
  | constructed by this function.
  |
  | If the construction of this hash is changed,
  | it will invalidate existing UTXO
  | snapshots. This will not result in any kind of
  | consensus failure, but it will force clients
  | that were expecting to make use of assumeutxo
  | to do traditional IBD instead.
  |
  | It is also possible, though very unlikely,
  | that a change in this construction could cause
  | a previously invalid (and potentially
  | malicious) UTXO snapshot to be considered
  | valid.
  */
pub fn apply_hash_with_hash_writer(
    ss:      &mut HashWriter,
    hash:    &u256,
    outputs: &HashMap<u32,Coin>)  {
    
    todo!();
        /*
            for (auto it = outputs.begin(); it != outputs.end(); ++it) {
            if (it == outputs.begin()) {
                ss << hash;
                ss << VARINT(it->second.nHeight * 2 + it->second.fCoinBase ? 1u : 0u);
            }

            ss << VARINT(it->first + 1);
            ss << it->second.out.scriptPubKey;
            ss << VARINT_MODE(it->second.out.nValue, VarIntMode::NONNEGATIVE_SIGNED);

            if (it == std::prev(outputs.end())) {
                ss << VARINT(0u);
            }
        }
        */
}

pub fn apply_hash(
    muhash:  &mut MuHash3072,
    hash:    &u256,
    outputs: &HashMap<u32,Coin>)  {
    
    todo!();
        /*
            for (auto it = outputs.begin(); it != outputs.end(); ++it) {
            OutPoint outpoint = OutPoint(hash, it->first);
            Coin coin = it->second;
            muhash.Insert(MakeUCharSpan(TxOutSer(outpoint, coin)));
        }
        */
}

pub fn apply_stats(
    stats:   &mut CoinsStats,
    hash:    &u256,
    outputs: &HashMap<u32,Coin>)  {
    
    todo!();
        /*
            assert(!outputs.empty());
        stats.nTransactions++;
        for (auto it = outputs.begin(); it != outputs.end(); ++it) {
            stats.nTransactionOutputs++;
            stats.nTotalAmount += it->second.out.nValue;
            stats.nBogoSize += GetBogoSize(it->second.out.scriptPubKey);
        }
        */
}
