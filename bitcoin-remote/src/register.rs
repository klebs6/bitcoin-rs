// ---------------- [ File: bitcoin-remote/src/register.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/rpc/register.h]

/**
  | Register block chain RPC commands
  |
  */
pub fn register_blockchain_rpc_commands(tablerpc: &mut RPCTable)  {
    
    todo!();
        /*
        
        */
}

/**
  | Register P2P networking RPC commands
  |
  */
pub fn register_net_rpc_commands(tablerpc: &mut RPCTable)  {
    
    todo!();
        /*
        
        */
}

/**
  | Register miscellaneous RPC commands
  |
  */
pub fn register_misc_rpc_commands(tablerpc: &mut RPCTable)  {
    
    todo!();
        /*
        
        */
}

/**
  | Register mining RPC commands
  |
  */
pub fn register_mining_rpc_commands(tablerpc: &mut RPCTable)  {
    
    todo!();
        /*
        
        */
}

/**
  | Register raw transaction RPC commands
  |
  */
pub fn register_raw_transaction_rpc_commands(tablerpc: &mut RPCTable)  {
    
    todo!();
        /*
        
        */
}

/**
  | Register raw transaction RPC commands
  |
  */
pub fn register_signer_rpc_commands(tablerpc: &mut RPCTable)  {
    
    todo!();
        /*
        
        */
}

#[inline] pub fn register_all_core_rpc_commands(t: &mut RPCTable)  {
    
    todo!();
        /*
            RegisterBlockchainRPCCommands(t);
        RegisterNetRPCCommands(t);
        RegisterMiscRPCCommands(t);
        RegisterMiningRPCCommands(t);
        RegisterRawTransactionRPCCommands(t);
    #ifdef ENABLE_EXTERNAL_SIGNER
        RegisterSignerRPCCommands(t);
    #endif // ENABLE_EXTERNAL_SIGNER
        */
}
