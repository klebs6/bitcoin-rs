crate::ix!();

/**
  | Flags for CClientUIInterface::ThreadSafeMessageBox
  |
  */
bitflags!{

    pub struct MessageBoxFlags: u32
    {
        const ICON_INFORMATION    = 0;
        const ICON_WARNING        = 1 << 0;
        const ICON_ERROR          = 1 << 1;

        /*
          | Mask of all available icons in
          | CClientUIInterface::MessageBoxFlags
          | 
          | This needs to be updated, when icons
          | are changed there!
          |
          */
        const ICON_MASK = 
            Self::ICON_INFORMATION.bits 
            | Self::ICON_WARNING.bits 
            | Self::ICON_ERROR.bits;

        /*
          | These values are taken from
          | qmessagebox.h "enum StandardButton" to
          | be directly usable
          |
          */
        const BTN_OK      = 0x00000400; // QMessageBox::Ok
        const BTN_YES     = 0x00004000; // QMessageBox::Yes
        const BTN_NO      = 0x00010000; // QMessageBox::No
        const BTN_ABORT   = 0x00040000; // QMessageBox::Abort
        const BTN_RETRY   = 0x00080000; // QMessageBox::Retry
        const BTN_IGNORE  = 0x00100000; // QMessageBox::Ignore
        const BTN_CLOSE   = 0x00200000; // QMessageBox::Close
        const BTN_CANCEL  = 0x00400000; // QMessageBox::Cancel
        const BTN_DISCARD = 0x00800000; // QMessageBox::Discard
        const BTN_HELP    = 0x01000000; // QMessageBox::Help
        const BTN_APPLY   = 0x02000000; // QMessageBox::Apply
        const BTN_RESET   = 0x04000000; // QMessageBox::Reset

        /*
          | Mask of all available buttons in
          | 
          | CClientUIInterface::MessageBoxFlags
          | 
          | This needs to be updated, when buttons
          | are changed there!
          |
          */
        const BTN_MASK = 
            Self::BTN_OK.bits 
            | Self::BTN_YES.bits 
            | Self::BTN_NO.bits 
            | Self::BTN_ABORT.bits 
            | Self::BTN_RETRY.bits 
            | Self::BTN_IGNORE.bits 
            | Self::BTN_CLOSE.bits 
            | Self::BTN_CANCEL.bits 
            | Self::BTN_DISCARD.bits 
            | Self::BTN_HELP.bits 
            | Self::BTN_APPLY.bits 
            | Self::BTN_RESET.bits;

        /*
          | Force blocking, modal message box
          | dialog (not just OS notification)
          |
          */
        const MODAL               = 0x10000000;

        /*
          | Do not print contents of message to
          | debug log
          |
          */
        const SECURE              = 0x40000000;

        /*
          | Predefined combinations for certain
          | default usage cases
          |
          */
        const MSG_INFORMATION = Self::ICON_INFORMATION.bits;

        const MSG_WARNING     = 
            Self::ICON_WARNING.bits 
            | Self::BTN_OK.bits 
            | Self::MODAL.bits;

        const MSG_ERROR       = 
            Self::ICON_ERROR.bits 
            | Self::BTN_OK.bits 
            | Self::MODAL.bits;
    }
}
