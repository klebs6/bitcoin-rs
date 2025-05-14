// ---------------- [ File: bitcoin-noui/src/noui.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/noui.h]
//-------------------------------------------[.cpp/bitcoin/src/noui.cpp]

/**
  | Store connections so we can disconnect
  | them when suppressing output
  |
  */
lazy_static!{
    /*
    boost::signals2::connection noui_ThreadSafeMessageBoxConn;
    boost::signals2::connection noui_ThreadSafeQuestionConn;
    boost::signals2::connection noui_InitMessageConn;
    */
}

/**
  | Non-GUI handler, which logs and prints
  | messages.
  |
  */
pub fn noui_thread_safe_message_box(
        message: &BilingualStr,
        caption: &str,
        style:   u32) -> bool {
    
    todo!();
        /*
            bool fSecure = style & CClientUIInterface::SECURE;
        style &= ~CClientUIInterface::SECURE;

        std::string strCaption;
        switch (style) {
        case CClientUIInterface::MSG_ERROR:
            strCaption = "Error: ";
            break;
        case CClientUIInterface::MSG_WARNING:
            strCaption = "Warning: ";
            break;
        case CClientUIInterface::MSG_INFORMATION:
            strCaption = "Information: ";
            break;
        default:
            strCaption = caption + ": "; // Use supplied caption (can be empty)
        }

        if (!fSecure) {
            LogPrintf("%s%s\n", strCaption, message.original);
        }
        tfm::format(std::cerr, "%s%s\n", strCaption, message.original);
        return false;
        */
}

/**
  | Non-GUI handler, which logs and prints
  | questions.
  |
  */
pub fn noui_thread_safe_question(
    /* ignored interactive message */
    _0:      &BilingualStr,
    message: &str,
    caption: &str,
    style:   u32) -> bool {

    todo!();
        /*
            return noui_ThreadSafeMessageBox(Untranslated(message), caption, style);
        */
}

/**
  | Non-GUI handler, which only logs a message.
  |
  */
pub fn noui_init_message(message: &String)  {
    
    todo!();
        /*
            LogPrintf("init message: %s\n", message);
        */
}

/**
  | Connect all bitcoind signal handlers
  |
  */
pub fn noui_thread_safe_message_box_redirect(
        message: &BilingualStr,
        caption: &str,
        style:   u32) -> bool {
    
    todo!();
        /*
            LogPrintf("%s: %s\n", caption, message.original);
        return false;
        */
}

pub fn noui_thread_safe_question_redirect(

    /* ignored interactive message */
    _0:      &BilingualStr,

    message: &str,
    caption: &str,
    style:   u32) -> bool {
    
    todo!();
        /*
            LogPrintf("%s: %s\n", caption, message);
        return false;
        */
}

pub fn noui_init_message_redirect(message: &str)  {
    
    todo!();
        /*
            LogPrintf("init message: %s\n", message);
        */
}

/**
  | Redirect all bitcoind signal handlers
  | to LogPrintf. Used to check or suppress
  | output during test runs that produce
  | expected errors
  |
  */
pub fn noui_test_redirect()  {
    
    todo!();
        /*
            noui_ThreadSafeMessageBoxConn.disconnect();
        noui_ThreadSafeQuestionConn.disconnect();
        noui_InitMessageConn.disconnect();
        noui_ThreadSafeMessageBoxConn = uiInterface.ThreadSafeMessageBox_connect(noui_ThreadSafeMessageBoxRedirect);
        noui_ThreadSafeQuestionConn = uiInterface.ThreadSafeQuestion_connect(noui_ThreadSafeQuestionRedirect);
        noui_InitMessageConn = uiInterface.InitMessage_connect(noui_InitMessageRedirect);
        */
}

/**
  | Reconnects the regular Non-GUI handlers
  | after having used noui_test_redirect
  |
  */
pub fn noui_reconnect()  {
    
    todo!();
        /*
            noui_ThreadSafeMessageBoxConn.disconnect();
        noui_ThreadSafeQuestionConn.disconnect();
        noui_InitMessageConn.disconnect();
        noui_connect();
        */
}
