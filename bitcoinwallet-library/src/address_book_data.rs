crate::ix!();

/**
  | Address book data
  |
  */
pub struct AddressBookData {
    change:   bool, // default = { true }
    label:    String,
    purpose:  String,
    destdata: AddressBookDataStringMap,
}

pub type AddressBookDataStringMap = HashMap<String,String>;

pub trait FindAddressBookEntry {

    fn find_address_book_entry(&self, 
        _0:           &TxDestination,
        allow_change: Option<bool>) -> *const AddressBookData;
}

impl Default for AddressBookData {
    
    fn default() -> Self {
        todo!();
        /*
        : purpose("unknown"),

        
        */
    }
}

impl AddressBookData {
    
    pub fn is_change(&self) -> bool {
        
        todo!();
        /*
            return m_change;
        */
    }
    
    pub fn get_label(&self) -> &String {
        
        todo!();
        /*
            return m_label;
        */
    }
    
    pub fn set_label(&mut self, label: &String)  {
        
        todo!();
        /*
            m_change = false;
            m_label = label;
        */
    }
}
