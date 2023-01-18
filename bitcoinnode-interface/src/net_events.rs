crate::ix!();

/**
  | Interface for message handling
  |
  */
pub trait NetEventsInterface:
InitializeNode
+ Send
+ Sync
+ FinalizeNode
+ ProcessMessages
+ SendMessages { }

pub trait ProcessMessages {

    /**
      | Process protocol messages received
      | from a given node
      | 
      | -----------
      | @param[in] pnode
      | 
      | The node which we have received messages
      | from.
      | ----------
      | @param[in] interrupt
      | 
      | Interrupt condition for processing
      | threads
      | 
      | -----------
      | @return
      | 
      | True if there is more work to be done
      |
      */
    fn process_messages(
        self:      Arc<Self>, 
        pnode:     &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        interrupt: &AtomicBool) -> bool;
}

pub trait InitializeNode {

    /**
      | Initialize a peer (setup state, queue
      | any initial messages)
      |
      */
    fn initialize_node(&mut self, 
        node: &mut AmoWriteGuard<Box<dyn NodeInterface>>);
}

pub trait FinalizeNode {

    /**
      | Handle removal of a peer (clear state)
      |
      */
    fn finalize_node(&mut self, node: &mut AmoWriteGuard<Box<dyn NodeInterface>>);
}

pub trait SendMessages {

    /**
      | Send queued protocol messages to a given
      | node.
      | 
      | -----------
      | @param[in] pnode
      | 
      | The node which we are sending messages
      | to.
      | 
      | -----------
      | @return
      | 
      | True if there is more work to be done
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(pnode->cs_sendProcessing)]
    fn send_messages(
        self:  Arc<Self>, 
        pnode: Amo<Box<dyn NodeInterface>>) -> bool;
}
