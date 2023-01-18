crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/createwalletdialog.h]
//-------------------------------------------[.cpp/bitcoin/src/qt/createwalletdialog.cpp]

/**
  | Dialog for creating wallets
  |
  */
#[Q_OBJECT]
pub struct CreateWalletDialog {
    base:        QDialog,
    ui:          *mut UiCreateWalletDialog,
    has_signers: bool, // default = false
}

impl Drop for CreateWalletDialog {
    fn drop(&mut self) {
        todo!();
        /*
            delete ui;
        */
    }
}

impl CreateWalletDialog {

    pub fn new(parent: *mut QWidget) -> Self {
    
        todo!();
        /*
            :
        QDialog(parent, gui_util::dialog_flags),
        ui(new UiCreateWalletDialog)

        ui->setupUi(this);
        ui->buttonBox->button(QDialogButtonBox::Ok)->setText(tr("Create"));
        ui->buttonBox->button(QDialogButtonBox::Ok)->setEnabled(false);
        ui->wallet_name_line_edit->setFocus(QtActiveWindowFocusReason);

        connect(ui->wallet_name_line_edit, &QLineEdit::textEdited, [this](const QString& text) {
            ui->buttonBox->button(QDialogButtonBox::Ok)->setEnabled(!text.isEmpty());
        });

        connect(ui->encrypt_wallet_checkbox, &QCheckBox::toggled, [this](bool checked) {
            // Disable the disable_privkeys_checkbox and external_signer_checkbox when isEncryptWalletChecked is
            // set to true, enable it when isEncryptWalletChecked is false.
            ui->disable_privkeys_checkbox->setEnabled(!checked);
    #ifdef ENABLE_EXTERNAL_SIGNER
            ui->external_signer_checkbox->setEnabled(m_has_signers && !checked);
    #endif
            // When the disable_privkeys_checkbox is disabled, uncheck it.
            if (!ui->disable_privkeys_checkbox->isEnabled()) {
                ui->disable_privkeys_checkbox->setChecked(false);
            }

            // When the external_signer_checkbox box is disabled, uncheck it.
            if (!ui->external_signer_checkbox->isEnabled()) {
                ui->external_signer_checkbox->setChecked(false);
            }

        });

        connect(ui->external_signer_checkbox, &QCheckBox::toggled, [this](bool checked) {
            ui->encrypt_wallet_checkbox->setEnabled(!checked);
            ui->blank_wallet_checkbox->setEnabled(!checked);
            ui->disable_privkeys_checkbox->setEnabled(!checked);
            ui->descriptor_checkbox->setEnabled(!checked);

            // The external signer checkbox is only enabled when a device is detected.
            // In that case it is checked by default. Toggling it restores the other
            // options to their default.
            ui->descriptor_checkbox->setChecked(checked);
            ui->encrypt_wallet_checkbox->setChecked(false);
            ui->disable_privkeys_checkbox->setChecked(checked);
            // The blank check box is ambiguous. This flag is always true for a
            // watch-only wallet, even though we immedidately fetch keys from the
            // external signer.
            ui->blank_wallet_checkbox->setChecked(checked);
        });

        connect(ui->disable_privkeys_checkbox, &QCheckBox::toggled, [this](bool checked) {
            // Disable the encrypt_wallet_checkbox when isDisablePrivateKeysChecked is
            // set to true, enable it when isDisablePrivateKeysChecked is false.
            ui->encrypt_wallet_checkbox->setEnabled(!checked);

            // Wallets without private keys start out blank
            if (checked) {
                ui->blank_wallet_checkbox->setChecked(true);
            }

            // When the encrypt_wallet_checkbox is disabled, uncheck it.
            if (!ui->encrypt_wallet_checkbox->isEnabled()) {
                ui->encrypt_wallet_checkbox->setChecked(false);
            }
        });

        connect(ui->blank_wallet_checkbox, &QCheckBox::toggled, [this](bool checked) {
            if (!checked) {
              ui->disable_privkeys_checkbox->setChecked(false);
            }
        });

    #ifndef USE_SQLITE
            ui->descriptor_checkbox->setToolTip(tr("Compiled without sqlite support (required for descriptor wallets)"));
            ui->descriptor_checkbox->setEnabled(false);
            ui->descriptor_checkbox->setChecked(false);
            ui->external_signer_checkbox->setEnabled(false);
            ui->external_signer_checkbox->setChecked(false);
    #endif

    #ifndef USE_BDB
            ui->descriptor_checkbox->setEnabled(false);
            ui->descriptor_checkbox->setChecked(true);
    #endif

    #ifndef ENABLE_EXTERNAL_SIGNER
            //: "External signing" means using devices such as hardware wallets.
            ui->external_signer_checkbox->setToolTip(tr("Compiled without external signing support (required for external signing)"));
            ui->external_signer_checkbox->setEnabled(false);
            ui->external_signer_checkbox->setChecked(false);
    #endif
        */
    }
    
    pub fn set_signers(&mut self, signers: &Vec<ExternalSigner>)  {
        
        todo!();
        /*
            m_has_signers = !signers.empty();
        if (m_has_signers) {
            ui->external_signer_checkbox->setEnabled(true);
            ui->external_signer_checkbox->setChecked(true);
            ui->encrypt_wallet_checkbox->setEnabled(false);
            ui->encrypt_wallet_checkbox->setChecked(false);
            // The order matters, because connect() is called when toggling a checkbox:
            ui->blank_wallet_checkbox->setEnabled(false);
            ui->blank_wallet_checkbox->setChecked(false);
            ui->disable_privkeys_checkbox->setEnabled(false);
            ui->disable_privkeys_checkbox->setChecked(true);
            const std::string label = signers[0].m_name;
            ui->wallet_name_line_edit->setText(QString::fromStdString(label));
            ui->buttonBox->button(QDialogButtonBox::Ok)->setEnabled(true);
        } else {
            ui->external_signer_checkbox->setEnabled(false);
        }
        */
    }
    
    pub fn wallet_name(&self) -> String {
        
        todo!();
        /*
            return ui->wallet_name_line_edit->text();
        */
    }
    
    pub fn is_encrypt_wallet_checked(&self) -> bool {
        
        todo!();
        /*
            return ui->encrypt_wallet_checkbox->isChecked();
        */
    }
    
    pub fn is_disable_private_keys_checked(&self) -> bool {
        
        todo!();
        /*
            return ui->disable_privkeys_checkbox->isChecked();
        */
    }
    
    pub fn is_make_blank_wallet_checked(&self) -> bool {
        
        todo!();
        /*
            return ui->blank_wallet_checkbox->isChecked();
        */
    }
    
    pub fn is_descriptor_wallet_checked(&self) -> bool {
        
        todo!();
        /*
            return ui->descriptor_checkbox->isChecked();
        */
    }
    
    pub fn is_external_signer_checked(&self) -> bool {
        
        todo!();
        /*
            return ui->external_signer_checkbox->isChecked();
        */
    }
}
