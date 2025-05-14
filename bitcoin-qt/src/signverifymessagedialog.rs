// ---------------- [ File: bitcoin-qt/src/signverifymessagedialog.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/signverifymessagedialog.h]

#[Q_OBJECT]
pub struct SignVerifyMessageDialog {
    base:           QDialog,
    ui:             *mut UiSignVerifyMessageDialog,
    model:          *mut WalletModel,
    platform_style: *const PlatformStyle,
}

//-------------------------------------------[.cpp/bitcoin/src/qt/signverifymessagedialog.cpp]
impl Drop for SignVerifyMessageDialog {
    fn drop(&mut self) {
        todo!();
        /*
            delete ui;
        */
    }
}

impl SignVerifyMessageDialog {
    
    /* ------------------ sign message  ------------------ */
    /* ----------------- verify message  ----------------- */
    pub fn new(
        platform_style: *const PlatformStyle,
        parent:         *mut QWidget) -> Self {
    
        todo!();
        /*


            :
        QDialog(parent, typename gui_util::dialog_flags),
        ui(new UiSignVerifyMessageDialog),
        model(nullptr),
        platformStyle(_platformStyle)

        ui->setupUi(this);

        ui->addressBookButton_SM->setIcon(platformStyle->SingleColorIcon(":/icons/address-book"));
        ui->pasteButton_SM->setIcon(platformStyle->SingleColorIcon(":/icons/editpaste"));
        ui->copySignatureButton_SM->setIcon(platformStyle->SingleColorIcon(":/icons/editcopy"));
        ui->signMessageButton_SM->setIcon(platformStyle->SingleColorIcon(":/icons/edit"));
        ui->clearButton_SM->setIcon(platformStyle->SingleColorIcon(":/icons/remove"));
        ui->addressBookButton_VM->setIcon(platformStyle->SingleColorIcon(":/icons/address-book"));
        ui->verifyMessageButton_VM->setIcon(platformStyle->SingleColorIcon(":/icons/transaction_0"));
        ui->clearButton_VM->setIcon(platformStyle->SingleColorIcon(":/icons/remove"));

        typename gui_util::setupAddressWidget(ui->addressIn_SM, this);
        typename gui_util::setupAddressWidget(ui->addressIn_VM, this);

        ui->addressIn_SM->installEventFilter(this);
        ui->messageIn_SM->installEventFilter(this);
        ui->signatureOut_SM->installEventFilter(this);
        ui->addressIn_VM->installEventFilter(this);
        ui->messageIn_VM->installEventFilter(this);
        ui->signatureIn_VM->installEventFilter(this);

        ui->signatureOut_SM->setFont(typename gui_util::fixedPitchFont());
        ui->signatureIn_VM->setFont(typename gui_util::fixedPitchFont());

        typename gui_util::handleCloseWindowShortcut(this);
        */
    }
    
    pub fn set_model(&mut self, model: *mut WalletModel)  {
        
        todo!();
        /*
            this->model = _model;
        */
    }
    
    pub fn set_address_sm(&mut self, address: &String)  {
        
        todo!();
        /*
            ui->addressIn_SM->setText(address);
        ui->messageIn_SM->setFocus();
        */
    }
    
    pub fn set_address_vm(&mut self, address: &String)  {
        
        todo!();
        /*
            ui->addressIn_VM->setText(address);
        ui->messageIn_VM->setFocus();
        */
    }
    
    pub fn show_tab_sm(&mut self, show: bool)  {
        
        todo!();
        /*
            ui->tabWidget->setCurrentIndex(0);
        if (fShow)
            this->show();
        */
    }
    
    pub fn show_tab_vm(&mut self, show: bool)  {
        
        todo!();
        /*
            ui->tabWidget->setCurrentIndex(1);
        if (fShow)
            this->show();
        */
    }
    
    #[Q_SLOT]
    pub fn on_address_book_button_sm_clicked(&mut self)  {
        
        todo!();
        /*
            if (model && model->getAddressTableModel())
        {
            model->refresh(/* pk_hash_only */ true);
            AddressBookPage dlg(platformStyle, AddressBookPage::ForSelection, AddressBookPage::ReceivingTab, this);
            dlg.setModel(model->getAddressTableModel());
            if (dlg.exec())
            {
                setAddress_SM(dlg.getReturnValue());
            }
        }
        */
    }
    
    #[Q_SLOT]
    pub fn on_paste_button_sm_clicked(&mut self)  {
        
        todo!();
        /*
            setAddress_SM(QApplication::clipboard()->text());
        */
    }
    
    #[Q_SLOT]
    pub fn on_sign_message_button_sm_clicked(&mut self)  {
        
        todo!();
        /*
            if (!model)
            return;

        /* Clear old signature to ensure users don't get confused on error with an old signature displayed */
        ui->signatureOut_SM->clear();

        TxDestination destination = DecodeDestination(ui->addressIn_SM->text().toStdString());
        if (!IsValidDestination(destination)) {
            ui->statusLabel_SM->setStyleSheet("QLabel { color: red; }");
            ui->statusLabel_SM->setText(tr("The entered address is invalid.") + QString(" ") + tr("Please check the address and try again."));
            return;
        }
        const PKHash* pkhash = std::get_if<PKHash>(&destination);
        if (!pkhash) {
            ui->addressIn_SM->setValid(false);
            ui->statusLabel_SM->setStyleSheet("QLabel { color: red; }");
            ui->statusLabel_SM->setText(tr("The entered address does not refer to a key.") + QString(" ") + tr("Please check the address and try again."));
            return;
        }

        WalletModel::UnlockContext ctx(model->requestUnlock());
        if (!ctx.isValid())
        {
            ui->statusLabel_SM->setStyleSheet("QLabel { color: red; }");
            ui->statusLabel_SM->setText(tr("Wallet unlock was cancelled."));
            return;
        }

        const std::string& message = ui->messageIn_SM->document()->toPlainText().toStdString();
        std::string signature;
        SigningResult res = model->wallet().signMessage(message, *pkhash, signature);

        QString error;
        switch (res) {
            case SigningResult::OK:
                error = tr("No error");
                break;
            case SigningResult::PRIVATE_KEY_NOT_AVAILABLE:
                error = tr("Private key for the entered address is not available.");
                break;
            case SigningResult::SIGNING_FAILED:
                error = tr("Message signing failed.");
                break;
            // no default case, so the compiler can warn about missing cases
        }

        if (res != SigningResult::OK) {
            ui->statusLabel_SM->setStyleSheet("QLabel { color: red; }");
            ui->statusLabel_SM->setText(QString("<nobr>") + error + QString("</nobr>"));
            return;
        }

        ui->statusLabel_SM->setStyleSheet("QLabel { color: green; }");
        ui->statusLabel_SM->setText(QString("<nobr>") + tr("Message signed.") + QString("</nobr>"));

        ui->signatureOut_SM->setText(QString::fromStdString(signature));
        */
    }
    
    #[Q_SLOT]
    pub fn on_copy_signature_button_sm_clicked(&mut self)  {
        
        todo!();
        /*
            typename gui_util::setClipboard(ui->signatureOut_SM->text());
        */
    }
    
    #[Q_SLOT]
    pub fn on_clear_button_sm_clicked(&mut self)  {
        
        todo!();
        /*
            ui->addressIn_SM->clear();
        ui->messageIn_SM->clear();
        ui->signatureOut_SM->clear();
        ui->statusLabel_SM->clear();

        ui->addressIn_SM->setFocus();
        */
    }
    
    #[Q_SLOT]
    pub fn on_address_book_button_vm_clicked(&mut self)  {
        
        todo!();
        /*
            if (model && model->getAddressTableModel())
        {
            AddressBookPage dlg(platformStyle, AddressBookPage::ForSelection, AddressBookPage::SendingTab, this);
            dlg.setModel(model->getAddressTableModel());
            if (dlg.exec())
            {
                setAddress_VM(dlg.getReturnValue());
            }
        }
        */
    }
    
    #[Q_SLOT]
    pub fn on_verify_message_button_vm_clicked(&mut self)  {
        
        todo!();
        /*
            const std::string& address = ui->addressIn_VM->text().toStdString();
        const std::string& signature = ui->signatureIn_VM->text().toStdString();
        const std::string& message = ui->messageIn_VM->document()->toPlainText().toStdString();

        const auto result = MessageVerify(address, signature, message);

        if (result == MessageVerificationResult::OK) {
            ui->statusLabel_VM->setStyleSheet("QLabel { color: green; }");
        } else {
            ui->statusLabel_VM->setStyleSheet("QLabel { color: red; }");
        }

        switch (result) {
        case MessageVerificationResult::OK:
            ui->statusLabel_VM->setText(
                QString("<nobr>") + tr("Message verified.") + QString("</nobr>")
            );
            return;
        case MessageVerificationResult::ERR_INVALID_ADDRESS:
            ui->statusLabel_VM->setText(
                tr("The entered address is invalid.") + QString(" ") +
                tr("Please check the address and try again.")
            );
            return;
        case MessageVerificationResult::ERR_ADDRESS_NO_KEY:
            ui->addressIn_VM->setValid(false);
            ui->statusLabel_VM->setText(
                tr("The entered address does not refer to a key.") + QString(" ") +
                tr("Please check the address and try again.")
            );
            return;
        case MessageVerificationResult::ERR_MALFORMED_SIGNATURE:
            ui->signatureIn_VM->setValid(false);
            ui->statusLabel_VM->setText(
                tr("The signature could not be decoded.") + QString(" ") +
                tr("Please check the signature and try again.")
            );
            return;
        case MessageVerificationResult::ERR_PUBKEY_NOT_RECOVERED:
            ui->signatureIn_VM->setValid(false);
            ui->statusLabel_VM->setText(
                tr("The signature did not match the message digest.") + QString(" ") +
                tr("Please check the signature and try again.")
            );
            return;
        case MessageVerificationResult::ERR_NOT_SIGNED:
            ui->statusLabel_VM->setText(
                QString("<nobr>") + tr("Message verification failed.") + QString("</nobr>")
            );
            return;
        }
        */
    }
    
    #[Q_SLOT]
    pub fn on_clear_button_vm_clicked(&mut self)  {
        
        todo!();
        /*
            ui->addressIn_VM->clear();
        ui->signatureIn_VM->clear();
        ui->messageIn_VM->clear();
        ui->statusLabel_VM->clear();

        ui->addressIn_VM->setFocus();
        */
    }
    
    pub fn event_filter(&mut self, 
        object: *mut QObject,
        event:  *mut QEvent) -> bool {
        
        todo!();
        /*
            if (event->type() == QEvent::MouseButtonPress || event->type() == QEvent::FocusIn)
        {
            if (ui->tabWidget->currentIndex() == 0)
            {
                /* Clear status message on focus change */
                ui->statusLabel_SM->clear();

                /* Select generated signature */
                if (object == ui->signatureOut_SM)
                {
                    ui->signatureOut_SM->selectAll();
                    return true;
                }
            }
            else if (ui->tabWidget->currentIndex() == 1)
            {
                /* Clear status message on focus change */
                ui->statusLabel_VM->clear();
            }
        }
        return QDialog::eventFilter(object, event);
        */
    }
    
    pub fn change_event(&mut self, e: *mut QEvent)  {
        
        todo!();
        /*
            if (e->type() == QEvent::PaletteChange) {
            ui->addressBookButton_SM->setIcon(platformStyle->SingleColorIcon(QStringLiteral(":/icons/address-book")));
            ui->pasteButton_SM->setIcon(platformStyle->SingleColorIcon(QStringLiteral(":/icons/editpaste")));
            ui->copySignatureButton_SM->setIcon(platformStyle->SingleColorIcon(QStringLiteral(":/icons/editcopy")));
            ui->signMessageButton_SM->setIcon(platformStyle->SingleColorIcon(QStringLiteral(":/icons/edit")));
            ui->clearButton_SM->setIcon(platformStyle->SingleColorIcon(QStringLiteral(":/icons/remove")));
            ui->addressBookButton_VM->setIcon(platformStyle->SingleColorIcon(QStringLiteral(":/icons/address-book")));
            ui->verifyMessageButton_VM->setIcon(platformStyle->SingleColorIcon(QStringLiteral(":/icons/transaction_0")));
            ui->clearButton_VM->setIcon(platformStyle->SingleColorIcon(QStringLiteral(":/icons/remove")));
        }

        QDialog::changeEvent(e);
        */
    }
}
