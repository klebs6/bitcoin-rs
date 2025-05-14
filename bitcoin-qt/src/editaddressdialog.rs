// ---------------- [ File: bitcoin-qt/src/editaddressdialog.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/editaddressdialog.h]

/**
  | Dialog for editing an address and associated
  | information.
  |
  */
#[Q_OBJECT]
pub struct EditAddressDialog {
    base:    QDialog,
    ui:      *mut UiEditAddressDialog,
    mapper:  *mut QDataWidgetMapper,
    mode:    EditAddressDialogMode,
    model:   *mut AddressTableModel,
    address: String,
}

pub enum EditAddressDialogMode {
    NewSendingAddress,
    EditReceivingAddress,
    EditSendingAddress
}

//-------------------------------------------[.cpp/bitcoin/src/qt/editaddressdialog.cpp]
impl Drop for EditAddressDialog {
    fn drop(&mut self) {
        todo!();
        /*
            delete ui;
        */
    }
}

impl EditAddressDialog {

    pub fn new(
        mode:   EditAddressDialogMode,
        parent: *mut QWidget) -> Self {
    
        todo!();
        /*
            :
        QDialog(parent, typename gui_util::dialog_flags),
        ui(new UiEditAddressDialog),
        mapper(nullptr),
        mode(_mode),
        model(nullptr)
        ui->setupUi(this);

        typename gui_util::setupAddressWidget(ui->addressEdit, this);

        switch(mode)
        {
        case NewSendingAddress:
            setWindowTitle(tr("New sending address"));
            break;
        case EditReceivingAddress:
            setWindowTitle(tr("Edit receiving address"));
            ui->addressEdit->setEnabled(false);
            break;
        case EditSendingAddress:
            setWindowTitle(tr("Edit sending address"));
            break;
        }

        mapper = new QDataWidgetMapper(this);
        mapper->setSubmitPolicy(QDataWidgetMapper::ManualSubmit);

        typename gui_util::ItemDelegate* delegate = new typename gui_util::ItemDelegate(mapper);
        connect(delegate, &typename gui_util::ItemDelegate::keyEscapePressed, this, &EditAddressDialog::reject);
        mapper->setItemDelegate(delegate);

        typename gui_util::handleCloseWindowShortcut(this);
        */
    }
    
    pub fn set_model(&mut self, model: *mut AddressTableModel)  {
        
        todo!();
        /*
            this->model = _model;
        if(!_model)
            return;

        mapper->setModel(_model);
        mapper->addMapping(ui->labelEdit, AddressTableModel::Label);
        mapper->addMapping(ui->addressEdit, AddressTableModel::Address);
        */
    }
    
    pub fn load_row(&mut self, row: i32)  {
        
        todo!();
        /*
            mapper->setCurrentIndex(row);
        */
    }
    
    #[Q_SLOT]
    pub fn save_current_row(&mut self) -> bool {
        
        todo!();
        /*
            if(!model)
            return false;

        switch(mode)
        {
        case NewSendingAddress:
            address = model->addRow(
                    AddressTableModel::Send,
                    ui->labelEdit->text(),
                    ui->addressEdit->text(),
                    model->GetDefaultAddressType());
            break;
        case EditReceivingAddress:
        case EditSendingAddress:
            if(mapper->submit())
            {
                address = ui->addressEdit->text();
            }
            break;
        }
        return !address.isEmpty();
        */
    }
    
    #[Q_SLOT]
    pub fn accept(&mut self)  {
        
        todo!();
        /*
            if(!model)
            return;

        if(!saveCurrentRow())
        {
            switch(model->getEditStatus())
            {
            case AddressTableModel::OK:
                // Failed with unknown reason. Just reject.
                break;
            case AddressTableModel::NO_CHANGES:
                // No changes were made during edit operation. Just reject.
                break;
            case AddressTableModel::INVALID_ADDRESS:
                QMessageBox::warning(this, windowTitle(),
                    tr("The entered address \"%1\" is not a valid Bitcoin address.").arg(ui->addressEdit->text()),
                    QMessageBox::Ok, QMessageBox::Ok);
                break;
            case AddressTableModel::DUPLICATE_ADDRESS:
                QMessageBox::warning(this, windowTitle(),
                    getDuplicateAddressWarning(),
                    QMessageBox::Ok, QMessageBox::Ok);
                break;
            case AddressTableModel::WALLET_UNLOCK_FAILURE:
                QMessageBox::critical(this, windowTitle(),
                    tr("Could not unlock wallet."),
                    QMessageBox::Ok, QMessageBox::Ok);
                break;
            case AddressTableModel::KEY_GENERATION_FAILURE:
                QMessageBox::critical(this, windowTitle(),
                    tr("New key generation failed."),
                    QMessageBox::Ok, QMessageBox::Ok);
                break;

            }
            return;
        }
        QDialog::accept();
        */
    }
    
    /**
      | Return a descriptive string when adding
      | an already-existing address fails.
      |
      */
    pub fn get_duplicate_address_warning(&self) -> String {
        
        todo!();
        /*
            QString dup_address = ui->addressEdit->text();
        QString existing_label = model->labelForAddress(dup_address);
        QString existing_purpose = model->purposeForAddress(dup_address);

        if (existing_purpose == "receive" &&
                (mode == NewSendingAddress || mode == EditSendingAddress)) {
            return tr(
                "Address \"%1\" already exists as a receiving address with label "
                "\"%2\" and so cannot be added as a sending address."
                ).arg(dup_address).arg(existing_label);
        }
        return tr(
            "The entered address \"%1\" is already in the address book with "
            "label \"%2\"."
            ).arg(dup_address).arg(existing_label);
        */
    }
    
    pub fn get_address(&self) -> String {
        
        todo!();
        /*
            return address;
        */
    }
    
    pub fn set_address(&mut self, address: &String)  {
        
        todo!();
        /*
            this->address = _address;
        ui->addressEdit->setText(_address);
        */
    }
}
