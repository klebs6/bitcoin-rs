crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/zmq/zmqabstractnotifier.cpp]

//-------------------------------------------[.cpp/bitcoin/src/zmq/zmqabstractnotifier.h]

pub type ZMQNotifierFactory = fn() -> Box<ZMQAbstractNotifier>;

///-----------------------------------
pub struct ZMQAbstractNotifier {
    psocket:                          *mut c_void,
    ty:                               String,
    address:                          String,

    /**
      | aka SNDHWM
      |
      */
    outbound_message_high_water_mark: i32,
}

pub const DEFAULT_ZMQ_SNDHWM: i32 = 1000;

pub trait ZmqAbstractNotifierInterface:
Initialize
+ crate::traits::Shutdown
+ NotifyBlock
+ NotifyBlockConnect
+ NotifyBlockDisconnect
+ NotifyTransactionAcceptance
+ NotifyTransactionRemoval
+ NotifyTransaction { }

impl Default for ZMQAbstractNotifier {
    
    fn default() -> Self {
        todo!();
        /*
        : psocket(nullptr),
        : outbound_message_high_water_mark(DEFAULT_ZMQ_SNDHWM),

        
        */
    }
}

impl Drop for ZMQAbstractNotifier {
    fn drop(&mut self) {
        todo!();
        /*
            assert(!psocket);
        */
    }
}

impl ZMQAbstractNotifier {
    
    pub fn create<T>() -> Box<ZMQAbstractNotifier> {
    
        todo!();
        /*
            return std::make_unique<T>();
        */
    }
    
    pub fn get_type(&self) -> String {
        
        todo!();
        /*
            return type;
        */
    }
    
    pub fn set_type(&mut self, t: &String)  {
        
        todo!();
        /*
            type = t;
        */
    }
    
    pub fn get_address(&self) -> String {
        
        todo!();
        /*
            return address;
        */
    }
    
    pub fn set_address(&mut self, a: &String)  {
        
        todo!();
        /*
            address = a;
        */
    }
    
    pub fn get_outbound_message_high_water_mark(&self) -> i32 {
        
        todo!();
        /*
            return outbound_message_high_water_mark;
        */
    }
    
    pub fn set_outbound_message_high_water_mark(&mut self, sndhwm: i32)  {
        
        todo!();
        /*
            if (sndhwm >= 0) {
                outbound_message_high_water_mark = sndhwm;
            }
        */
    }
    
    pub fn notify_block(&mut self, block_index: Arc<BlockIndex>) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn notify_transaction(&mut self, transaction: &Transaction) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn notify_block_connect(&mut self, block_index: Arc<BlockIndex>) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn notify_block_disconnect(&mut self, block_index: Arc<BlockIndex>) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn notify_transaction_acceptance(&mut self, 
        transaction:      &Transaction,
        mempool_sequence: u64) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn notify_transaction_removal(&mut self, 
        transaction:      &Transaction,
        mempool_sequence: u64) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}
