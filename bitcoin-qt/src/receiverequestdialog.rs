// ---------------- [ File: bitcoin-qt/src/receiverequestdialog.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/receiverequestdialog.h]

#[Q_OBJECT]
pub struct ReceiveRequestDialog {
    base:  QDialog,
    ui:    *mut UiReceiveRequestDialog,
    model: *mut WalletModel,
    info:  SendCoinsRecipient,
}

//-------------------------------------------[.cpp/bitcoin/src/qt/receiverequestdialog.cpp]
impl Drop for ReceiveRequestDialog {
    fn drop(&mut self) {
        todo!();
        /*
            delete ui;
        */
    }
}

impl ReceiveRequestDialog {

    pub fn new(parent: *mut QWidget) -> Self {
    
        todo!();
        /*
            :
        QDialog(parent, typename gui_util::dialog_flags),
        ui(new UiReceiveRequestDialog),
        model(nullptr)
        ui->setupUi(this);
        typename gui_util::handleCloseWindowShortcut(this);
        */
    }
    
    pub fn set_model(&mut self, model: *mut WalletModel)  {
        
        todo!();
        /*
            this->model = _model;

        if (_model)
            connect(_model->getOptionsModel(), &OptionsModel::displayUnitChanged, this, &ReceiveRequestDialog::updateDisplayUnit);

        // update the display unit if necessary
        update();
        */
    }
    
    pub fn set_info(&mut self, info: &SendCoinsRecipient)  {
        
        todo!();
        /*
            this->info = _info;
        setWindowTitle(tr("Request payment to %1").arg(info.label.isEmpty() ? info.address : info.label));
        QString uri = typename gui_util::formatBitcoinURI(info);

    #ifdef USE_QRCODE
        if (ui->qr_code->setQR(uri, info.address)) {
            connect(ui->btnSaveAs, &QPushButton::clicked, ui->qr_code, &QRImageWidget::saveImage);
        } else {
            ui->btnSaveAs->setEnabled(false);
        }
    #else
        ui->btnSaveAs->hide();
        ui->qr_code->hide();
    #endif

        ui->uri_content->setText("<a href=\"" + uri + "\">" + typename gui_util::HtmlEscape(uri) + "</a>");
        ui->address_content->setText(info.address);

        if (!info.amount) {
            ui->amount_tag->hide();
            ui->amount_content->hide();
        } // Amount is set in updateDisplayUnit() slot.
        updateDisplayUnit();

        if (!info.label.isEmpty()) {
            ui->label_content->setText(info.label);
        } else {
            ui->label_tag->hide();
            ui->label_content->hide();
        }

        if (!info.message.isEmpty()) {
            ui->message_content->setText(info.message);
        } else {
            ui->message_tag->hide();
            ui->message_content->hide();
        }

        if (!model->getWalletName().isEmpty()) {
            ui->wallet_content->setText(model->getWalletName());
        } else {
            ui->wallet_tag->hide();
            ui->wallet_content->hide();
        }

        ui->btnVerify->setVisible(model->wallet().hasExternalSigner());

        connect(ui->btnVerify, &QPushButton::clicked, [this] {
            model->displayAddress(info.address.toStdString());
        });
        */
    }
    
    #[Q_SLOT]
    pub fn update_display_unit(&mut self)  {
        
        todo!();
        /*
            if (!model) return;
        ui->amount_content->setText(BitcoinUnits::formatWithUnit(model->getOptionsModel()->getDisplayUnit(), info.amount));
        */
    }
    
    #[Q_SLOT]
    pub fn on_btn_copyuri_clicked(&mut self)  {
        
        todo!();
        /*
            typename gui_util::setClipboard(typename gui_util::formatBitcoinURI(info));
        */
    }
    
    #[Q_SLOT]
    pub fn on_btn_copy_address_clicked(&mut self)  {
        
        todo!();
        /*
            typename gui_util::setClipboard(info.address);
        */
    }
}
