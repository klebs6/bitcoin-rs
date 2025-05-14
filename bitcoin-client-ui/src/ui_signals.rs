// ---------------- [ File: bitcoin-client-ui/src/ui_signals.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/node/ui_interface.cpp]

#[derive(Default)]
pub struct UISignals {

    /*TODO
    boost::signals2::signal<CClientUIInterface::ThreadSafeMessageBoxSig, boost::signals2::optional_last_value<bool>> ThreadSafeMessageBox;
    boost::signals2::signal<CClientUIInterface::ThreadSafeQuestionSig, boost::signals2::optional_last_value<bool>> ThreadSafeQuestion;
    boost::signals2::signal<CClientUIInterface::InitMessageSig> InitMessage;
    boost::signals2::signal<CClientUIInterface::NotifyNumConnectionsChangedSig> NotifyNumConnectionsChanged;
    boost::signals2::signal<CClientUIInterface::NotifyNetworkActiveChangedSig> NotifyNetworkActiveChanged;
    boost::signals2::signal<CClientUIInterface::NotifyAlertChangedSig> NotifyAlertChanged;
    boost::signals2::signal<CClientUIInterface::ShowProgressSig> ShowProgress;
    boost::signals2::signal<CClientUIInterface::NotifyBlockTipSig> NotifyBlockTip;
    boost::signals2::signal<CClientUIInterface::NotifyHeaderTipSig> NotifyHeaderTip;
    boost::signals2::signal<CClientUIInterface::BannedListChangedSig> BannedListChanged;
    */
}

macro_rules! add_signals_impl_wrapper {
    ($signal_name:ident) => {
        /*
        
            boost::signals2::connection CClientUIInterface::signal_name##_connect(std::function<signal_name##Sig> fn) 
            {                                                                                                         
                return g_ui_signals.signal_name.connect(fn);                                                          
            }
        */
    }
}

add_signals_impl_wrapper!{ ThreadSafeMessageBox }
add_signals_impl_wrapper!{ ThreadSafeQuestion }
add_signals_impl_wrapper!{ InitMessage }
add_signals_impl_wrapper!{ NotifyNumConnectionsChanged }
add_signals_impl_wrapper!{ NotifyNetworkActiveChanged }
add_signals_impl_wrapper!{ NotifyAlertChanged }
add_signals_impl_wrapper!{ ShowProgress }
add_signals_impl_wrapper!{ NotifyBlockTip }
add_signals_impl_wrapper!{ NotifyHeaderTip }
add_signals_impl_wrapper!{ BannedListChanged }
