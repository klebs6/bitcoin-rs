// ---------------- [ File: bitcoin-qt/src/sendcoinsentry.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/sendcoinsentry.h]

/**
  | A single entry in the dialog for sending
  | bitcoins.
  | 
  | Stacked widget, with different UIs
  | for payment requests with a strong payee
  | identity.
  |
  */
#[Q_OBJECT]
pub struct SendCoinsEntry {
    base:           QStackedWidget,
    recipient:      SendCoinsRecipient,
    ui:             *mut UiSendCoinsEntry,
    model:          *mut WalletModel,
    platform_style: *const PlatformStyle,
}

//-------------------------------------------[.cpp/bitcoin/src/qt/sendcoinsentry.cpp]
impl Drop for SendCoinsEntry {
    fn drop(&mut self) {
        todo!();
        /*
            delete ui;
        */
    }
}

impl SendCoinsEntry {
    
    #[Q_SIGNAL]
    pub fn remove_entry(&mut self, entry: *mut SendCoinsEntry)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn use_available_balance(&mut self, entry: *mut SendCoinsEntry)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn pay_amount_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn subtract_fee_from_amount_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }

    pub fn new(
        platform_style: *const PlatformStyle,
        parent:         *mut QWidget) -> Self {
    
        todo!();
        /*


            :
        QStackedWidget(parent),
        ui(new UiSendCoinsEntry),
        model(nullptr),
        platformStyle(_platformStyle)

        ui->setupUi(this);

        ui->addressBookButton->setIcon(platformStyle->SingleColorIcon(":/icons/address-book"));
        ui->pasteButton->setIcon(platformStyle->SingleColorIcon(":/icons/editpaste"));
        ui->deleteButton->setIcon(platformStyle->SingleColorIcon(":/icons/remove"));
        ui->deleteButton_is->setIcon(platformStyle->SingleColorIcon(":/icons/remove"));
        ui->deleteButton_s->setIcon(platformStyle->SingleColorIcon(":/icons/remove"));

        setCurrentWidget(ui->SendCoins);

        if (platformStyle->getUseExtraSpacing())
            ui->payToLayout->setSpacing(4);

        // normal bitcoin address field
        typename gui_util::setupAddressWidget(ui->payTo, this);
        // just a label for displaying bitcoin address(es)
        ui->payTo_is->setFont(typename gui_util::fixedPitchFont());

        // Connect signals
        connect(ui->payAmount, &BitcoinAmountField::valueChanged, this, &SendCoinsEntry::payAmountChanged);
        connect(ui->checkboxSubtractFeeFromAmount, &QCheckBox::toggled, this, &SendCoinsEntry::subtractFeeFromAmountChanged);
        connect(ui->deleteButton, &QPushButton::clicked, this, &SendCoinsEntry::deleteClicked);
        connect(ui->deleteButton_is, &QPushButton::clicked, this, &SendCoinsEntry::deleteClicked);
        connect(ui->deleteButton_s, &QPushButton::clicked, this, &SendCoinsEntry::deleteClicked);
        connect(ui->useAvailableBalanceButton, &QPushButton::clicked, this, &SendCoinsEntry::useAvailableBalanceClicked);
        */
    }
    
    #[Q_SLOT]
    pub fn on_paste_button_clicked(&mut self)  {
        
        todo!();
        /*
            // Paste text from clipboard into recipient field
        ui->payTo->setText(QApplication::clipboard()->text());
        */
    }
    
    #[Q_SLOT]
    pub fn on_address_book_button_clicked(&mut self)  {
        
        todo!();
        /*
            if(!model)
            return;
        AddressBookPage dlg(platformStyle, AddressBookPage::ForSelection, AddressBookPage::SendingTab, this);
        dlg.setModel(model->getAddressTableModel());
        if(dlg.exec())
        {
            ui->payTo->setText(dlg.getReturnValue());
            ui->payAmount->setFocus();
        }
        */
    }
    
    #[Q_SLOT]
    pub fn on_pay_to_text_changed(&mut self, address: &String)  {
        
        todo!();
        /*
            updateLabel(address);
        */
    }
    
    pub fn set_model(&mut self, model: *mut WalletModel)  {
        
        todo!();
        /*
            this->model = _model;

        if (_model && _model->getOptionsModel())
            connect(_model->getOptionsModel(), &OptionsModel::displayUnitChanged, this, &SendCoinsEntry::updateDisplayUnit);

        clear();
        */
    }
    
    #[Q_SLOT]
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            // clear UI elements for normal payment
        ui->payTo->clear();
        ui->addAsLabel->clear();
        ui->payAmount->clear();
        if (model && model->getOptionsModel()) {
            ui->checkboxSubtractFeeFromAmount->setChecked(model->getOptionsModel()->getSubFeeFromAmount());
        }
        ui->messageTextLabel->clear();
        ui->messageTextLabel->hide();
        ui->messageLabel->hide();
        // clear UI elements for unauthenticated payment request
        ui->payTo_is->clear();
        ui->memoTextLabel_is->clear();
        ui->payAmount_is->clear();
        // clear UI elements for authenticated payment request
        ui->payTo_s->clear();
        ui->memoTextLabel_s->clear();
        ui->payAmount_s->clear();

        // update the display unit, to not use the default ("BTC")
        updateDisplayUnit();
        */
    }
    
    #[Q_SLOT]
    pub fn check_subtract_fee_from_amount(&mut self)  {
        
        todo!();
        /*
            ui->checkboxSubtractFeeFromAmount->setChecked(true);
        */
    }
    
    #[Q_SLOT]
    pub fn delete_clicked(&mut self)  {
        
        todo!();
        /*
            Q_EMIT removeEntry(this);
        */
    }
    
    #[Q_SLOT]
    pub fn use_available_balance_clicked(&mut self)  {
        
        todo!();
        /*
            Q_EMIT useAvailableBalance(this);
        */
    }
    
    pub fn validate(&mut self, node: Rc<RefCell<dyn NodeInterface>>) -> bool {
        
        todo!();
        /*
            if (!model)
            return false;

        // Check input validity
        bool retval = true;

        if (!model->validateAddress(ui->payTo->text()))
        {
            ui->payTo->setValid(false);
            retval = false;
        }

        if (!ui->payAmount->validate())
        {
            retval = false;
        }

        // Sending a zero amount is invalid
        if (ui->payAmount->value(nullptr) <= 0)
        {
            ui->payAmount->setValid(false);
            retval = false;
        }

        // Reject dust outputs:
        if (retval && typename gui_util::isDust(node, ui->payTo->text(), ui->payAmount->value())) {
            ui->payAmount->setValid(false);
            retval = false;
        }

        return retval;
        */
    }
    
    pub fn get_value(&mut self) -> SendCoinsRecipient {
        
        todo!();
        /*
            recipient.address = ui->payTo->text();
        recipient.label = ui->addAsLabel->text();
        recipient.amount = ui->payAmount->value();
        recipient.message = ui->messageTextLabel->text();
        recipient.fSubtractFeeFromAmount = (ui->checkboxSubtractFeeFromAmount->checkState() == QtChecked);

        return recipient;
        */
    }
    
    /**
      | Set up the tab chain manually, as Qt messes
      | up the tab chain by default in some cases
      | (issue https://bugreports.qt-project.org/browse/QTBUG-10907).
      |
      */
    pub fn setup_tab_chain(&mut self, prev: *mut QWidget) -> *mut QWidget {
        
        todo!();
        /*
            QWidget::setTabOrder(prev, ui->payTo);
        QWidget::setTabOrder(ui->payTo, ui->addAsLabel);
        QWidget *w = ui->payAmount->setupTabChain(ui->addAsLabel);
        QWidget::setTabOrder(w, ui->checkboxSubtractFeeFromAmount);
        QWidget::setTabOrder(ui->checkboxSubtractFeeFromAmount, ui->addressBookButton);
        QWidget::setTabOrder(ui->addressBookButton, ui->pasteButton);
        QWidget::setTabOrder(ui->pasteButton, ui->deleteButton);
        return ui->deleteButton;
        */
    }
    
    pub fn set_value(&mut self, value: &SendCoinsRecipient)  {
        
        todo!();
        /*
            recipient = value;
        {
            // message
            ui->messageTextLabel->setText(recipient.message);
            ui->messageTextLabel->setVisible(!recipient.message.isEmpty());
            ui->messageLabel->setVisible(!recipient.message.isEmpty());

            ui->addAsLabel->clear();
            ui->payTo->setText(recipient.address); // this may set a label from addressbook
            if (!recipient.label.isEmpty()) // if a label had been set from the addressbook, don't overwrite with an empty label
                ui->addAsLabel->setText(recipient.label);
            ui->payAmount->setValue(recipient.amount);
        }
        */
    }
    
    pub fn set_address(&mut self, address: &String)  {
        
        todo!();
        /*
            ui->payTo->setText(address);
        ui->payAmount->setFocus();
        */
    }
    
    pub fn set_amount(&mut self, amount: &Amount)  {
        
        todo!();
        /*
            ui->payAmount->setValue(amount);
        */
    }
    
    /**
      | Return whether the entry is still empty
      | and unedited
      |
      */
    pub fn is_clear(&mut self) -> bool {
        
        todo!();
        /*
            return ui->payTo->text().isEmpty() && ui->payTo_is->text().isEmpty() && ui->payTo_s->text().isEmpty();
        */
    }
    
    pub fn set_focus(&mut self)  {
        
        todo!();
        /*
            ui->payTo->setFocus();
        */
    }
    
    #[Q_SLOT]
    pub fn update_display_unit(&mut self)  {
        
        todo!();
        /*
            if(model && model->getOptionsModel())
        {
            // Update payAmount with the current unit
            ui->payAmount->setDisplayUnit(model->getOptionsModel()->getDisplayUnit());
            ui->payAmount_is->setDisplayUnit(model->getOptionsModel()->getDisplayUnit());
            ui->payAmount_s->setDisplayUnit(model->getOptionsModel()->getDisplayUnit());
        }
        */
    }
    
    #[Q_SLOT]
    pub fn change_event(&mut self, e: *mut QEvent)  {
        
        todo!();
        /*
            if (e->type() == QEvent::PaletteChange) {
            ui->addressBookButton->setIcon(platformStyle->SingleColorIcon(QStringLiteral(":/icons/address-book")));
            ui->pasteButton->setIcon(platformStyle->SingleColorIcon(QStringLiteral(":/icons/editpaste")));
            ui->deleteButton->setIcon(platformStyle->SingleColorIcon(QStringLiteral(":/icons/remove")));
            ui->deleteButton_is->setIcon(platformStyle->SingleColorIcon(QStringLiteral(":/icons/remove")));
            ui->deleteButton_s->setIcon(platformStyle->SingleColorIcon(QStringLiteral(":/icons/remove")));
        }

        QStackedWidget::changeEvent(e);
        */
    }
    
    #[Q_SLOT]
    pub fn update_label(&mut self, address: &String) -> bool {
        
        todo!();
        /*
            if(!model)
            return false;

        // Fill in label from address book, if address has an associated label
        QString associatedLabel = model->getAddressTableModel()->labelForAddress(address);
        if(!associatedLabel.isEmpty())
        {
            ui->addAsLabel->setText(associatedLabel);
            return true;
        }

        return false;
        */
    }
}
