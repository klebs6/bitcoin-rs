// ---------------- [ File: bitcoin-indexed-chain/src/wallet_interface.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/node/interfaces.cpp]

//-------------------------------------------[.cpp/bitcoin/src/util/ui_change_type.h]

//-------------------------------------------[.cpp/bitcoin/src/wallet/interfaces.cpp]

/**
  | Return implementation of Init interface
  | for the wallet process.
  |
  */
pub fn make_wallet_init(
        argc:        i32,
        argv:        &[*mut u8],
        exit_status: &mut i32) -> Box<dyn Init> {
    
    todo!();
        /*
        
        */
}

/**
  | Return implementation of Init interface
  | for the gui process.
  |
  */
pub fn make_gui_init(
        argc: i32,
        argv: &[*mut u8]) -> Box<dyn Init> {
    
    todo!();
        /*
        
        */
}

pub struct NotificationsHandlerImpl {
    proxy: Arc<NotificationsProxy>,
}

impl Handler for NotificationsHandlerImpl {

}

impl Drop for NotificationsHandlerImpl {
    fn drop(&mut self) {
        todo!();
        /*
            disconnect();
        */
    }
}

impl From<Arc<dyn ChainNotifications>> for NotificationsHandlerImpl {

    fn from(notifications: Arc<dyn ChainNotifications>) -> Self {
    
        todo!();
        /*


            : m_proxy(std::make_shared<NotificationsProxy>(std::move(notifications)))

            RegisterSharedValidationInterface(m_proxy);
        */
    }
}
    
impl Disconnect for NotificationsHandlerImpl {
    fn disconnect(&mut self)  {
        
        todo!();
        /*
            if (m_proxy) {
                UnregisterSharedValidationInterface(m_proxy);
                m_proxy.reset();
            }
        */
    }
}

pub struct NotificationsProxy {
    notifications: Arc<dyn ChainNotifications>,
}

impl From<Arc<dyn ChainNotifications>> for NotificationsProxy {
    
    fn from(notifications: Arc<dyn ChainNotifications>) -> Self {
    
        todo!();
        /*
        : notifications(std::move(notifications)),

        
        */
    }
}

impl ValidationInterface for NotificationsProxy { }

impl NewPoWValidBlock for NotificationsProxy { }
impl BlockChecked     for NotificationsProxy { }

impl TransactionAddedToMempool for NotificationsProxy {
    fn transaction_added_to_mempool(&mut self, 
        tx:               &TransactionRef,
        mempool_sequence: u64)  {
        
        todo!();
        /*
            m_notifications->transactionAddedToMempool(tx, mempool_sequence);
        */
    }
}
    
impl TransactionRemovedFromMempool for NotificationsProxy {

    fn transaction_removed_from_mempool(&mut self, 
        tx:               &TransactionRef,
        reason:           MemPoolRemovalReason,
        mempool_sequence: u64)  {
        
        todo!();
        /*
            m_notifications->transactionRemovedFromMempool(tx, reason, mempool_sequence);
        */
    }
}

impl BlockConnected for NotificationsProxy {

    fn block_connected(&mut self, 
        block: Arc<Block>,
        index: Arc<BlockIndex>)  {
        
        todo!();
        /*
            m_notifications->blockConnected(*block, index->nHeight);
        */
    }
}
    
impl BlockDisconnected for NotificationsProxy {
    fn block_disconnected(&mut self, 
        block: Arc<Block>,
        index: Arc<BlockIndex>)  {
        
        todo!();
        /*
            m_notifications->blockDisconnected(*block, index->nHeight);
        */
    }
}
    
impl UpdatedBlockTip for NotificationsProxy {

    fn updated_block_tip(&mut self, 
        index:      Option<Arc<BlockIndex>>,
        fork_index: Option<Arc<BlockIndex>>,
        is_ibd:     bool)  {
        
        todo!();
        /*
            m_notifications->updatedBlockTip();
        */
    }
}
    
impl ChainStateFlushed for NotificationsProxy {
    fn chain_state_flushed(&mut self, locator: &BlockLocator)  {
        
        todo!();
        /*
            m_notifications->chainStateFlushed(locator);
        */
    }
}

///-----------------------
pub struct RpcHandlerImpl {
    command:         RPCCommand,
    wrapped_command: *const RPCCommand,
}

impl Handler for RpcHandlerImpl {

}

impl Drop for RpcHandlerImpl {
    fn drop(&mut self) {
        todo!();
        /*
            disconnect();
        */
    }
}

impl From<&RPCCommand> for RpcHandlerImpl {
    
    fn from(command: &RPCCommand) -> Self {
    
        todo!();
        /*
        : command(command),
        : wrapped_command(&command),

            m_command.actor = [this](const JSONRPCRequest& request, UniValue& result, bool last_handler) {
                if (!m_wrapped_command) return false;
                try {
                    return m_wrapped_command->actor(request, result, last_handler);
                } catch (const UniValue& e) {
                    // If this is not the last handler and a wallet not found
                    // exception was thrown, return false so the next handler can
                    // try to handle the request. Otherwise, reraise the exception.
                    if (!last_handler) {
                        const UniValue& code = e["code"];
                        if (code.isNum() && code.get_int() == RPC_WALLET_NOT_FOUND) {
                            return false;
                        }
                    }
                    throw;
                }
            };
            ::tableRPC.appendCommand(m_command.name, &m_command);
        */
    }
}

impl Disconnect for RpcHandlerImpl {

    fn disconnect(&mut self)  {
        
        todo!();
        /*
            if (m_wrapped_command) {
                m_wrapped_command = nullptr;
                ::tableRPC.removeCommand(m_command.name, &m_command);
            }
        */
    }
}
