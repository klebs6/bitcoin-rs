// ---------------- [ File: bitcoin-qt/src/winshutdownmonitor.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/winshutdownmonitor.h]
//-------------------------------------------[.cpp/bitcoin/src/qt/winshutdownmonitor.cpp]

#[cfg(WIN32)]
pub struct WinShutdownMonitor {
    base: QAbstractNativeEventFilter,
}

#[cfg(WIN32)]
impl WinShutdownMonitor {

    /**
      | Implements QAbstractNativeEventFilter
      | interface for processing Windows messages
      |
      */
    pub fn native_event_filter(&mut self, 
        event_type: &QByteArray,
        message:    *mut c_void,
        pn_result:  *mut i64) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Register the reason for blocking shutdown
      | on Windows to allow clean client exit
      |
      */
    pub fn register_shutdown_block_reason(
        str_reason:  &String,
        main_win_id: &HWND)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | If we don't want a message to be processed by
      | Qt, return true and set result to the value
      | that the window procedure should
      | return. Otherwise return false.
      */
    #[cfg(Q_OS_WIN)]
    pub fn native_event_filter(&mut self, 
        event_type: &QByteArray,
        message:    *mut c_void,
        pn_result:  *mut i64) -> bool {
        
        todo!();
        /*
            Q_UNUSED(eventType);

           MSG *pMsg = static_cast<MSG *>(pMessage);

           switch(pMsg->message)
           {
               case WM_QUERYENDSESSION:
               {
                   // Initiate a client shutdown after receiving a WM_QUERYENDSESSION and block
                   // Windows session end until we have finished client shutdown.
                   StartShutdown();
                   *pnResult = FALSE;
                   return true;
               }

               case WM_ENDSESSION:
               {
                   *pnResult = FALSE;
                   return true;
               }
           }

           return false;
        */
    }
    
    #[cfg(Q_OS_WIN)]
    pub fn register_shutdown_block_reason(&mut self, 
        str_reason:  &String,
        main_win_id: &HWND)  {
        
        todo!();
        /*
            typedef BOOL (WINAPI *PSHUTDOWNBRCREATE)(HWND, LPCWSTR);
        PSHUTDOWNBRCREATE shutdownBRCreate = (PSHUTDOWNBRCREATE)GetProcAddress(GetModuleHandleA("User32.dll"), "ShutdownBlockReasonCreate");
        if (shutdownBRCreate == nullptr) {
            qWarning() << "registerShutdownBlockReason: GetProcAddress for ShutdownBlockReasonCreate failed";
            return;
        }

        if (shutdownBRCreate(mainWinId, strReason.toStdWString().c_str()))
            qInfo() << "registerShutdownBlockReason: Successfully registered: " + strReason;
        else
            qWarning() << "registerShutdownBlockReason: Failed to register: " + strReason;
        */
    }
}
