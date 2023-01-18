crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/node/ui_interface.h]

/**
  | Current sync state passed to tip changed
  | callbacks.
  |
  */
pub enum SynchronizationState {
    INIT_REINDEX,
    INIT_DOWNLOAD,
    POST_INIT
}

pub fn get_synchronization_state(init: bool) -> SynchronizationState {
    
    todo!();
        /*
            if (!init) return SynchronizationState::POST_INIT;
        if (::fReindex) return SynchronizationState::INIT_REINDEX;
        return SynchronizationState::INIT_DOWNLOAD;
        */
}

/**
  | Signals for UI communication.
  |
  */
#[derive(Default,Clone)]
pub struct ClientUIInterface {

}

pub mod client_ui_interface {

    use super::*;

    macro_rules! add_signals_decl_wrapper {
        ($signal_name:ident, 
         $rtype:ty, 
         $($arg:ty: $argty:ty),*) => {
            /*
            
                rtype signal_name(__VA_ARGS__);                                                        
                using signal_name##Sig = rtype(__VA_ARGS__);                                           
                boost::signals2::connection signal_name##_connect(std::function<signal_name##Sig> fn);
            */
        }
    }

    /**
      | Show message box.
      |
      */
    add_signals_decl_wrapper!{
        ThreadSafeMessageBox, 
        bool, 
        message: &BilingualStr, 
        caption: &String, 
        style:   u32
    }

    /**
      | If possible, ask the user a question.
      | If not, falls back to ThreadSafeMessageBox(noninteractive_message,
      | caption, style) and returns false.
      |
      */
    add_signals_decl_wrapper!{
        ThreadSafeQuestion, 
        bool, 
        message: &BilingualStr, 
        noninteractive_message: &String, 
        caption: &String, 
        style: u32
    }

    /**
      | Progress message during initialization.
      |
      */
    add_signals_decl_wrapper!{
        InitMessage, 
        c_void, 
        message: &String
    }

    /**
      | Number of network connections changed.
      |
      */
    add_signals_decl_wrapper!{
        NotifyNumConnectionsChanged, 
        c_void, 
        new_num_connections: i32
    }

    /**
      | Network activity state changed.
      |
      */
    add_signals_decl_wrapper!{
        NotifyNetworkActiveChanged, 
        c_void, 
        network_active: bool
    }

    /**
      | Status bar alerts changed.
      |
      */
    add_signals_decl_wrapper!{
        NotifyAlertChanged, 
        c_void,
    }

    /**
      | Show progress e.g. for verifychain.
      | resume_possible indicates shutting
      | down now will result in the current progress
      | action resuming upon restart.
      |
      */
    add_signals_decl_wrapper!{
        ShowProgress, 
        c_void, 
        title: &String, 
        n_progress: i32, 
        resume_possible: bool
    }

    /**
      | New block has been accepted
      |
      */
    add_signals_decl_wrapper!{
        NotifyBlockTip, 
        c_void, 
        _0: SynchronizationState, 
        _1: *const CBlockIndex
    }

    /**
      | Best header has changed
      |
      */
    add_signals_decl_wrapper!{
        NotifyHeaderTip, 
        c_void, 
        _0: SynchronizationState, 
        _1: *const CBlockIndex
    }

    /**
      | Banlist did change.
      |
      */
    add_signals_decl_wrapper!{
        BannedListChanged, 
        c_void, 
        _0: c_void
    }
}


impl ClientUIInterface {

    pub fn thread_safe_message_box(&mut self, 
        message: &BilingualStr,
        caption: &str,
        style:   u32) -> bool {
        
        todo!();
        /*
            return g_ui_signals.ThreadSafeMessageBox(message, caption, style).value_or(false);
        */
    }
    
    pub fn thread_safe_question(&mut self, 
        message:                 &BilingualStr,
        non_interactive_message: &str,
        caption:                 &str,
        style:                   u32) -> bool {
        
        todo!();
        /*
            return g_ui_signals.ThreadSafeQuestion(message, non_interactive_message, caption, style).value_or(false);
        */
    }
    
    pub fn init_message(&mut self, message: &str)  {
        
        todo!();
        /*
            return g_ui_signals.InitMessage(message);
        */
    }
    
    pub fn notify_num_connections_changed(&mut self, new_num_connections: i32)  {
        
        todo!();
        /*
            return g_ui_signals.NotifyNumConnectionsChanged(newNumConnections);
        */
    }
    
    pub fn notify_network_active_changed(&mut self, network_active: bool)  {
        
        todo!();
        /*
            return g_ui_signals.NotifyNetworkActiveChanged(networkActive);
        */
    }
    
    pub fn notify_alert_changed(&mut self)  {
        
        todo!();
        /*
            return g_ui_signals.NotifyAlertChanged();
        */
    }
    
    pub fn show_progress(&mut self, 
        title:           &str,
        n_progress:      i32,
        resume_possible: bool)  {
        
        todo!();
        /*
            return g_ui_signals.ShowProgress(title, nProgress, resume_possible);
        */
    }
    
    pub fn notify_block_tip(&mut self, 
        s: SynchronizationState,
        i: *const BlockIndex)  {
        
        todo!();
        /*
            return g_ui_signals.NotifyBlockTip(s, i);
        */
    }
    
    pub fn notify_header_tip(&mut self, 
        s: SynchronizationState,
        i: *const BlockIndex)  {
        
        todo!();
        /*
            return g_ui_signals.NotifyHeaderTip(s, i);
        */
    }
    
    pub fn banned_list_changed(&mut self)  {
        
        todo!();
        /*
            return g_ui_signals.BannedListChanged();
        */
    }
}
