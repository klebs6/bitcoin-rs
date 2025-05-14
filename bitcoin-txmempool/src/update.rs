// ---------------- [ File: bitcoin-txmempool/src/update.rs ]
crate::ix!();

/**
  | Helpers for modifying TxMemPool::mapTx,
  | which is a boost multi_index.
  |
  */
pub struct UpdateDescendantState {
    modify_size:  i64,
    modify_fee:   Amount,
    modify_count: i64,
}

impl UpdateDescendantState {
    
    pub fn new(
        modify_size:  i64,
        modify_fee:   Amount,
        modify_count: i64) -> Self {
    
        todo!();
        /*
        : modify_size(_modifySize),
        : modify_fee(_modifyFee),
        : modify_count(_modifyCount),

        
        */
    }
    
    pub fn invoke(&mut self, e: &mut TxMemPoolEntry)  {
        
        todo!();
        /*
            e.UpdateDescendantState(modifySize, modifyFee, modifyCount);
        */
    }
}

pub struct UpdateAncestorState {
    modify_size:         i64,
    modify_fee:          Amount,
    modify_count:        i64,
    modify_sig_ops_cost: i64,
}

impl UpdateAncestorState {
    
    pub fn new(
        modify_size:         i64,
        modify_fee:          Amount,
        modify_count:        i64,
        modify_sig_ops_cost: i64) -> Self {
    
        todo!();
        /*


            :
            modifySize(_modifySize), modifyFee(_modifyFee), modifyCount(_modifyCount), modifySigOpsCost(_modifySigOpsCost)
        */
    }
    
    pub fn invoke(&mut self, e: &mut TxMemPoolEntry)  {
        
        todo!();
        /*
            e.UpdateAncestorState(modifySize, modifyFee, modifyCount, modifySigOpsCost);
        */
    }
}

///------------------------
pub struct UpdateFeeDelta {
    fee_delta: i64,
}

impl UpdateFeeDelta {
    
    pub fn new(fee_delta: i64) -> Self {
    
        todo!();
        /*
        : fee_delta(_feeDelta),

        
        */
    }
    
    pub fn invoke(&mut self, e: &mut TxMemPoolEntry)  {
        
        todo!();
        /*
            e.UpdateFeeDelta(feeDelta);
        */
    }
}

///---------------------
pub struct UpdateLockPoints {
    lp: Rc<LockPoints>,
}

impl UpdateLockPoints {
    
    pub fn new(lp: &LockPoints) -> Self {
    
        todo!();
        /*
        : lp(_lp),

        
        */
    }
    
    pub fn invoke(&mut self, e: &mut TxMemPoolEntry)  {
        
        todo!();
        /*
            e.UpdateLockPoints(lp);
        */
    }
}
