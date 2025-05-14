// ---------------- [ File: bitcoin-txmempoolentry/src/compare.rs ]
crate::ix!();

/**
  | \class CompareTxMemPoolEntryByDescendantScore
  | 
  | Sort an entry by max(score/size of entry's
  | tx, score/size with all descendants).
  |
  */
pub struct CompareTxMemPoolEntryByDescendantScore {

}

impl CompareTxMemPoolEntryByDescendantScore {
    
    pub fn invoke(&self, 
        a: &TxMemPoolEntry,
        b: &TxMemPoolEntry) -> bool {
        
        todo!();
        /*
            double a_mod_fee, a_size, b_mod_fee, b_size;

            GetModFeeAndSize(a, a_mod_fee, a_size);
            GetModFeeAndSize(b, b_mod_fee, b_size);

            // Avoid division by rewriting (a/b > c/d) as (a*d > c*b).
            double f1 = a_mod_fee * b_size;
            double f2 = a_size * b_mod_fee;

            if (f1 == f2) {
                return a.GetTime() >= b.GetTime();
            }
            return f1 < f2;
        */
    }

    /**
      | Return the fee/size we're using for
      | sorting this entry.
      |
      */
    pub fn get_mod_fee_and_size(&self, 
        a:       &TxMemPoolEntry,
        mod_fee: &mut f64,
        size:    &mut f64)  {
        
        todo!();
        /*
            // Compare feerate with descendants to feerate of the transaction, and
            // return the fee/size for the max.
            double f1 = (double)a.GetModifiedFee() * a.GetSizeWithDescendants();
            double f2 = (double)a.GetModFeesWithDescendants() * a.GetTxSize();

            if (f2 > f1) {
                mod_fee = a.GetModFeesWithDescendants();
                size = a.GetSizeWithDescendants();
            } else {
                mod_fee = a.GetModifiedFee();
                size = a.GetTxSize();
            }
        */
    }
}

/**
  | \class CompareTxMemPoolEntryByScore
  | 
  | Sort by feerate of entry (fee/size)
  | in descending order
  | 
  | This is only used for transaction relay,
  | so we use GetFee() instead of GetModifiedFee()
  | to avoid leaking prioritization information
  | via the sort order.
  |
  */
pub struct CompareTxMemPoolEntryByScore {

}

impl CompareTxMemPoolEntryByScore {
    
    pub fn invoke(&self, 
        a: &TxMemPoolEntry,
        b: &TxMemPoolEntry) -> bool {
        
        todo!();
        /*
            double f1 = (double)a.GetFee() * b.GetTxSize();
            double f2 = (double)b.GetFee() * a.GetTxSize();
            if (f1 == f2) {
                return b.GetTx().GetHash() < a.GetTx().GetHash();
            }
            return f1 > f2;
        */
    }
}

pub struct CompareTxMemPoolEntryByEntryTime {

}

impl CompareTxMemPoolEntryByEntryTime {
    
    pub fn invoke(&self, 
        a: &TxMemPoolEntry,
        b: &TxMemPoolEntry) -> bool {
        
        todo!();
        /*
            return a.GetTime() < b.GetTime();
        */
    }
}


/**
  | \class CompareTxMemPoolEntryByAncestorScore
  | 
  | Sort an entry by min(score/size of entry's
  | tx, score/size with all ancestors).
  |
  */
pub struct CompareTxMemPoolEntryByAncestorFee {

}

impl CompareTxMemPoolEntryByAncestorFee {
    
    pub fn invoke<T>(&self, a: &T, b: &T) -> bool {
    
        todo!();
        /*
            double a_mod_fee, a_size, b_mod_fee, b_size;

            GetModFeeAndSize(a, a_mod_fee, a_size);
            GetModFeeAndSize(b, b_mod_fee, b_size);

            // Avoid division by rewriting (a/b > c/d) as (a*d > c*b).
            double f1 = a_mod_fee * b_size;
            double f2 = a_size * b_mod_fee;

            if (f1 == f2) {
                return a.GetTx().GetHash() < b.GetTx().GetHash();
            }
            return f1 > f2;
        */
    }

    /**
      | Return the fee/size we're using for
      | sorting this entry.
      |
      */
    pub fn get_mod_fee_and_size<T>(&self, 
        a:       &T,
        mod_fee: &mut f64,
        size:    &mut f64)  {
    
        todo!();
        /*
            // Compare feerate with ancestors to feerate of the transaction, and
            // return the fee/size for the min.
            double f1 = (double)a.GetModifiedFee() * a.GetSizeWithAncestors();
            double f2 = (double)a.GetModFeesWithAncestors() * a.GetTxSize();

            if (f1 > f2) {
                mod_fee = a.GetModFeesWithAncestors();
                size = a.GetSizeWithAncestors();
            } else {
                mod_fee = a.GetModifiedFee();
                size = a.GetTxSize();
            }
        */
    }
}

pub struct CompareIteratorByHash {

}

impl CompareIteratorByHash {

    /**
      | SFINAE for T where T is either a pointer
      | type (e.g., a txiter) or
      | a reference_wrapper<T> (e.g. a wrapped
      | TxMemPoolEntry&)
      */
    pub fn invoke_with_refwrapper<T>(&self, 
        a: Amo<T>,
        b: Amo<T>) -> bool {
    
        todo!();
        /*
            return a.get().GetTx().GetHash() < b.get().GetTx().GetHash();
        */
    }
    
    
    pub fn invoke<T>(&self, a: &T, b: &T) -> bool {
    
        todo!();
        /*
            return a->GetTx().GetHash() < b->GetTx().GetHash();
        */
    }
}
