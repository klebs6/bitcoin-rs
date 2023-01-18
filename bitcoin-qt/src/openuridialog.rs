crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/openuridialog.h]
//-------------------------------------------[.cpp/bitcoin/src/qt/openuridialog.cpp]

#[Q_OBJECT]
pub struct OpenURIDialog {
    base: QDialog,
    ui:   *mut UiOpenURIDialog,
}

impl Drop for OpenURIDialog {
    fn drop(&mut self) {
        todo!();
        /*
            delete ui;
        */
    }
}

impl OpenURIDialog {

    pub fn new(parent: *mut QWidget) -> Self {
    
        todo!();
        /*
           :
           QDialog(parent, typename gui_util::dialog_flags),
           ui(new UiOpenURIDialog)
           ui->setupUi(this);

           typename gui_util::handleCloseWindowShortcut(this);
        */
    }
    
    pub fn geturi(&mut self) -> String {
        
        todo!();
        /*
            return ui->uriEdit->text();
        */
    }
    
    #[Q_SLOT]
    pub fn accept(&mut self)  {
        
        todo!();
        /*
            SendCoinsRecipient rcp;
        if(typename gui_util::parseBitcoinURI(getURI(), &rcp))
        {
            /* Only accept value URIs */
            QDialog::accept();
        } else {
            ui->uriEdit->setValid(false);
        }
        */
    }
}
