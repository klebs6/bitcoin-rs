// ---------------- [ File: bitcoin-qt/src/macnotificationhandler.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/macnotificationhandler.h]

/**
  | Macintosh-specific notification
  | handler (supports UserNotificationCenter).
  |
  */
#[Q_OBJECT]
pub struct MacNotificationHandler {
    base: QObject,
}

impl MacNotificationHandler {

    /**
      | shows a macOS 10.8+ UserNotification
      | in the UserNotificationCenter
      |
      */
    pub fn show_notification(&mut self, 
        title: &String,
        text:  &String)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | check if OS can handle UserNotifications
      |
      */
    pub fn has_user_notification_center_support(&mut self) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn instance() -> *mut MacNotificationHandler {
        
        todo!();
        /*
        
        */
    }
}
