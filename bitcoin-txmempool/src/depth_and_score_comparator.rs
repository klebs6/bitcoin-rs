crate::ix!();

pub struct DepthAndScoreComparator {

}

impl DepthAndScoreComparator {

    pub fn invoke(&mut self, 
        a: &TxMemPoolIndexedTransactionSetConstIterator,
        b: &TxMemPoolIndexedTransactionSetConstIterator) -> bool {
        
        todo!();
        /*
            uint64_t counta = a->GetCountWithAncestors();
            uint64_t countb = b->GetCountWithAncestors();
            if (counta == countb) {
                return CompareTxMemPoolEntryByScore()(*a, *b);
            }
            return counta < countb;
        */
    }
}
