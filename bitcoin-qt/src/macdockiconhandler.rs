crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/macdockiconhandler.h]

/**
  | macOS-specific Dock icon handler.
  |
  */
#[Q_OBJECT]
pub struct MacDockIconHandler {
    base: QObject,
}

impl MacDockIconHandler {
    
    pub fn instance() -> *mut MacDockIconHandler {
        
        todo!();
        /*
        
        */
    }
    
    pub fn cleanup()  {
        
        todo!();
        /*
        
        */
    }

    #[Q_SIGNAL]
    pub fn dock_icon_clicked(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    fn default() -> Self {
        todo!();
        /*
        
        */
    }
}
