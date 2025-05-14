// ---------------- [ File: bitcoin-coinselect/src/comparator.rs ]
crate::ix!();

/**
  | Descending order comparator
  |
  */
pub struct DescendingOrderComparator {

}

impl DescendingOrderComparator {
    
    pub fn invoke(&self, 
        a: &OutputGroup,
        b: &OutputGroup) -> bool {
        
        todo!();
        /*
            return a.GetSelectionAmount() > b.GetSelectionAmount();
        */
    }
}
