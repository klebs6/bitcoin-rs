// ---------------- [ File: bitcoin-qt/src/sendcoinsdialog.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/sendcoinsdialog.h]

/**
  | Dialog for sending bitcoins
  |
  */
#[Q_OBJECT]
pub struct SendCoinsDialog {
    base:                  QDialog,
    ui:                    *mut UiSendCoinsDialog,
    client_model:          *mut ClientModel,
    model:                 *mut WalletModel,
    coin_control:          Box<CoinControl>,
    current_transaction:   Box<WalletModelTransaction>,
    new_recipient_allowed: bool,
    fee_minimized:         bool,
    platform_style:        *const PlatformStyle,
}

impl SendCoinsDialog {
    
    #[Q_SIGNAL]
    pub fn coins_sent(&mut self, txid: &u256)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Fired when a message should be reported
      | to the user
      |
      */
    #[Q_SIGNAL]
    pub fn message(&mut self, 
        title:   &str,
        message: &str,
        style:   u32)  {
        
        todo!();
        /*
        
        */
    }
}

pub const SEND_CONFIRM_DELAY: usize = 3;

#[Q_OBJECT]
pub struct SendConfirmationDialog {
    base:                QMessageBox,
    yes_button:          *mut QAbstractButton,
    count_down_timer:    QTimer,
    sec_delay:           i32,
    confirm_button_text: String,
}

//-------------------------------------------[.cpp/bitcoin/src/qt/sendcoinsdialog.cpp]

pub const conf_targets: &[i32] = &[2, 4, 6, 12, 24, 48, 144, 504, 1008];

pub fn get_conf_target_for_index(index: i32) -> i32 {
    
    todo!();
        /*
            if (index+1 > static_cast<int>(confTargets.size())) {
            return confTargets.back();
        }
        if (index < 0) {
            return confTargets[0];
        }
        return confTargets[index];
        */
}

pub fn get_index_for_conf_target(target: i32) -> i32 {
    
    todo!();
        /*
            for (unsigned int i = 0; i < confTargets.size(); i++) {
            if (confTargets[i] >= target) {
                return i;
            }
        }
        return confTargets.size() - 1;
        */
}

///---------------------------------
impl Drop for SendCoinsDialog {
    fn drop(&mut self) {
        todo!();
        /*
            QSettings settings;
        settings.setValue("fFeeSectionMinimized", fFeeMinimized);
        settings.setValue("nFeeRadio", ui->groupFee->checkedId());
        settings.setValue("nConfTarget", getConfTargetForIndex(ui->confTargetSelector->currentIndex()));
        settings.setValue("nTransactionFee", (i64)ui->customFee->value());

        delete ui;
        */
    }
}

impl SendCoinsDialog {
    
    pub fn new(
        platform_style: *const PlatformStyle,
        parent:         *mut QWidget) -> Self {
    
        todo!();
        /*


            :
        QDialog(parent, typename gui_util::dialog_flags),
        ui(new UiSendCoinsDialog),
        clientModel(nullptr),
        model(nullptr),
        m_coin_control(new CCoinControl),
        fNewRecipientAllowed(true),
        fFeeMinimized(true),
        platformStyle(_platformStyle)

        ui->setupUi(this);

        if (!_platformStyle->getImagesOnButtons()) {
            ui->addButton->setIcon(QIcon());
            ui->clearButton->setIcon(QIcon());
            ui->sendButton->setIcon(QIcon());
        } else {
            ui->addButton->setIcon(_platformStyle->SingleColorIcon(":/icons/add"));
            ui->clearButton->setIcon(_platformStyle->SingleColorIcon(":/icons/remove"));
            ui->sendButton->setIcon(_platformStyle->SingleColorIcon(":/icons/send"));
        }

        typename gui_util::setupAddressWidget(ui->lineEditCoinControlChange, this);

        addEntry();

        connect(ui->addButton, &QPushButton::clicked, this, &SendCoinsDialog::addEntry);
        connect(ui->clearButton, &QPushButton::clicked, this, &SendCoinsDialog::clear);

        // Coin Control
        connect(ui->pushButtonCoinControl, &QPushButton::clicked, this, &SendCoinsDialog::coinControlButtonClicked);
        connect(ui->checkBoxCoinControlChange, &QCheckBox::stateChanged, this, &SendCoinsDialog::coinControlChangeChecked);
        connect(ui->lineEditCoinControlChange, &QValidatedLineEdit::textEdited, this, &SendCoinsDialog::coinControlChangeEdited);

        // Coin Control: clipboard actions
        QAction *clipboardQuantityAction = new QAction(tr("Copy quantity"), this);
        QAction *clipboardAmountAction = new QAction(tr("Copy amount"), this);
        QAction *clipboardFeeAction = new QAction(tr("Copy fee"), this);
        QAction *clipboardAfterFeeAction = new QAction(tr("Copy after fee"), this);
        QAction *clipboardBytesAction = new QAction(tr("Copy bytes"), this);
        QAction *clipboardLowOutputAction = new QAction(tr("Copy dust"), this);
        QAction *clipboardChangeAction = new QAction(tr("Copy change"), this);
        connect(clipboardQuantityAction, &QAction::triggered, this, &SendCoinsDialog::coinControlClipboardQuantity);
        connect(clipboardAmountAction, &QAction::triggered, this, &SendCoinsDialog::coinControlClipboardAmount);
        connect(clipboardFeeAction, &QAction::triggered, this, &SendCoinsDialog::coinControlClipboardFee);
        connect(clipboardAfterFeeAction, &QAction::triggered, this, &SendCoinsDialog::coinControlClipboardAfterFee);
        connect(clipboardBytesAction, &QAction::triggered, this, &SendCoinsDialog::coinControlClipboardBytes);
        connect(clipboardLowOutputAction, &QAction::triggered, this, &SendCoinsDialog::coinControlClipboardLowOutput);
        connect(clipboardChangeAction, &QAction::triggered, this, &SendCoinsDialog::coinControlClipboardChange);
        ui->labelCoinControlQuantity->addAction(clipboardQuantityAction);
        ui->labelCoinControlAmount->addAction(clipboardAmountAction);
        ui->labelCoinControlFee->addAction(clipboardFeeAction);
        ui->labelCoinControlAfterFee->addAction(clipboardAfterFeeAction);
        ui->labelCoinControlBytes->addAction(clipboardBytesAction);
        ui->labelCoinControlLowOutput->addAction(clipboardLowOutputAction);
        ui->labelCoinControlChange->addAction(clipboardChangeAction);

        // init transaction fee section
        QSettings settings;
        if (!settings.contains("fFeeSectionMinimized"))
            settings.setValue("fFeeSectionMinimized", true);
        if (!settings.contains("nFeeRadio") && settings.contains("nTransactionFee") && settings.value("nTransactionFee").toLongLong() > 0) // compatibility
            settings.setValue("nFeeRadio", 1); // custom
        if (!settings.contains("nFeeRadio"))
            settings.setValue("nFeeRadio", 0); // recommended
        if (!settings.contains("nSmartFeeSliderPosition"))
            settings.setValue("nSmartFeeSliderPosition", 0);
        if (!settings.contains("nTransactionFee"))
            settings.setValue("nTransactionFee", (i64)DEFAULT_PAY_TX_FEE);
        ui->groupFee->setId(ui->radioSmartFee, 0);
        ui->groupFee->setId(ui->radioCustomFee, 1);
        ui->groupFee->button((int)std::max(0, std::min(1, settings.value("nFeeRadio").toInt())))->setChecked(true);
        ui->customFee->SetAllowEmpty(false);
        ui->customFee->setValue(settings.value("nTransactionFee").toLongLong());
        minimizeFeeSection(settings.value("fFeeSectionMinimized").toBool());

        typename gui_util::ExceptionSafeConnect(ui->sendButton, &QPushButton::clicked, this, &SendCoinsDialog::sendButtonClicked);
        */
    }
    
    pub fn set_client_model(&mut self, client_model: *mut ClientModel)  {
        
        todo!();
        /*
            this->clientModel = _clientModel;

        if (_clientModel) {
            connect(_clientModel, &ClientModel::numBlocksChanged, this, &SendCoinsDialog::updateNumberOfBlocks);
        }
        */
    }
    
    pub fn set_model(&mut self, model: *mut WalletModel)  {
        
        todo!();
        /*
            this->model = _model;

        if(_model && _model->getOptionsModel())
        {
            for(int i = 0; i < ui->entries->count(); ++i)
            {
                SendCoinsEntry *entry = qobject_cast<SendCoinsEntry*>(ui->entries->itemAt(i)->widget());
                if(entry)
                {
                    entry->setModel(_model);
                }
            }

            typename interfaces::WalletBalances balances = _model->wallet().getBalances();
            setBalance(balances);
            connect(_model, &WalletModel::balanceChanged, this, &SendCoinsDialog::setBalance);
            connect(_model->getOptionsModel(), &OptionsModel::displayUnitChanged, this, &SendCoinsDialog::updateDisplayUnit);
            updateDisplayUnit();

            // Coin Control
            connect(_model->getOptionsModel(), &OptionsModel::displayUnitChanged, this, &SendCoinsDialog::coinControlUpdateLabels);
            connect(_model->getOptionsModel(), &OptionsModel::coinControlFeaturesChanged, this, &SendCoinsDialog::coinControlFeatureChanged);
            ui->frameCoinControl->setVisible(_model->getOptionsModel()->getCoinControlFeatures());
            coinControlUpdateLabels();

            // fee section
            for (const int n : confTargets) {
                ui->confTargetSelector->addItem(tr("%1 (%2 blocks)").arg(typename gui_util::formatNiceTimeOffset(n*Params().GetConsensus().nPowTargetSpacing)).arg(n));
            }
            connect(ui->confTargetSelector, qOverload<int>(&QComboBox::currentIndexChanged), this, &SendCoinsDialog::updateSmartFeeLabel);
            connect(ui->confTargetSelector, qOverload<int>(&QComboBox::currentIndexChanged), this, &SendCoinsDialog::coinControlUpdateLabels);

    #if (QT_VERSION >= QT_VERSION_CHECK(5, 15, 0))
            connect(ui->groupFee, &QButtonGroup::idClicked, this, &SendCoinsDialog::updateFeeSectionControls);
            connect(ui->groupFee, &QButtonGroup::idClicked, this, &SendCoinsDialog::coinControlUpdateLabels);
    #else
            connect(ui->groupFee, qOverload<int>(&QButtonGroup::buttonClicked), this, &SendCoinsDialog::updateFeeSectionControls);
            connect(ui->groupFee, qOverload<int>(&QButtonGroup::buttonClicked), this, &SendCoinsDialog::coinControlUpdateLabels);
    #endif

            connect(ui->customFee, &BitcoinAmountField::valueChanged, this, &SendCoinsDialog::coinControlUpdateLabels);
            connect(ui->optInRBF, &QCheckBox::stateChanged, this, &SendCoinsDialog::updateSmartFeeLabel);
            connect(ui->optInRBF, &QCheckBox::stateChanged, this, &SendCoinsDialog::coinControlUpdateLabels);
            CAmount requiredFee = model->wallet().getRequiredFee(1000);
            ui->customFee->SetMinValue(requiredFee);
            if (ui->customFee->value() < requiredFee) {
                ui->customFee->setValue(requiredFee);
            }
            ui->customFee->setSingleStep(requiredFee);
            updateFeeSectionControls();
            updateSmartFeeLabel();

            // set default rbf checkbox state
            ui->optInRBF->setCheckState(QtChecked);

            if (model->wallet().hasExternalSigner()) {
                //: "device" usually means a hardware wallet.
                ui->sendButton->setText(tr("Sign on device"));
                if (gArgs.GetArg("-signer", "") != "") {
                    ui->sendButton->setEnabled(true);
                    ui->sendButton->setToolTip(tr("Connect your hardware wallet first."));
                } else {
                    ui->sendButton->setEnabled(false);
                    //: "External signer" means using devices such as hardware wallets.
                    ui->sendButton->setToolTip(tr("Set external signer script path in Options -> Wallet"));
                }
            } else if (model->wallet().privateKeysDisabled()) {
                ui->sendButton->setText(tr("Cr&eate Unsigned"));
                ui->sendButton->setToolTip(tr("Creates a Partially Signed Bitcoin Transaction (PSBT) for use with e.g. an offline %1 wallet, or a PSBT-compatible hardware wallet.").arg(PACKAGE_NAME));
            }

            // set the smartfee-sliders default value (wallets default conf.target or last stored value)
            QSettings settings;
            if (settings.value("nSmartFeeSliderPosition").toInt() != 0) {
                // migrate nSmartFeeSliderPosition to nConfTarget
                // nConfTarget is available since 0.15 (replaced nSmartFeeSliderPosition)
                int nConfirmTarget = 25 - settings.value("nSmartFeeSliderPosition").toInt(); // 25 == old slider range
                settings.setValue("nConfTarget", nConfirmTarget);
                settings.remove("nSmartFeeSliderPosition");
            }
            if (settings.value("nConfTarget").toInt() == 0)
                ui->confTargetSelector->setCurrentIndex(getIndexForConfTarget(model->wallet().getConfirmTarget()));
            else
                ui->confTargetSelector->setCurrentIndex(getIndexForConfTarget(settings.value("nConfTarget").toInt()));
        }
        */
    }
    
    /**
      | Format confirmation message
      |
      */
    #[Q_SIGNAL]
    pub fn prepare_send_text(&mut self, 
        question_string:  &mut String,
        informative_text: &mut String,
        detailed_text:    &mut String) -> bool {
        
        todo!();
        /*
            QList<SendCoinsRecipient> recipients;
        bool valid = true;

        for(int i = 0; i < ui->entries->count(); ++i)
        {
            SendCoinsEntry *entry = qobject_cast<SendCoinsEntry*>(ui->entries->itemAt(i)->widget());
            if(entry)
            {
                if(entry->validate(model->node()))
                {
                    recipients.append(entry->getValue());
                }
                else if (valid)
                {
                    ui->scrollArea->ensureWidgetVisible(entry);
                    valid = false;
                }
            }
        }

        if(!valid || recipients.isEmpty())
        {
            return false;
        }

        fNewRecipientAllowed = false;
        WalletModel::UnlockContext ctx(model->requestUnlock());
        if(!ctx.isValid())
        {
            // Unlock wallet was cancelled
            fNewRecipientAllowed = true;
            return false;
        }

        // prepare transaction for getting txFee earlier
        m_current_transaction = std::make_unique<WalletModelTransaction>(recipients);
        WalletModel::SendCoinsReturn prepareStatus;

        updateCoinControlState();

        prepareStatus = model->prepareTransaction(*m_current_transaction, *m_coin_control);

        // process prepareStatus and on error generate message shown to user
        processSendCoinsReturn(prepareStatus,
            BitcoinUnits::formatWithUnit(model->getOptionsModel()->getDisplayUnit(), m_current_transaction->getTransactionFee()));

        if(prepareStatus.status != WalletModel::OK) {
            fNewRecipientAllowed = true;
            return false;
        }

        CAmount txFee = m_current_transaction->getTransactionFee();
        QStringList formatted;
        for (const SendCoinsRecipient &rcp : m_current_transaction->getRecipients())
        {
            // generate amount string with wallet name in case of multiwallet
            QString amount = BitcoinUnits::formatWithUnit(model->getOptionsModel()->getDisplayUnit(), rcp.amount);
            if (model->isMultiwallet()) {
                amount.append(tr(" from wallet '%1'").arg(typename gui_util::HtmlEscape(model->getWalletName())));
            }

            // generate address string
            QString address = rcp.address;

            QString recipientElement;

            {
                if(rcp.label.length() > 0) // label with address
                {
                    recipientElement.append(tr("%1 to '%2'").arg(amount, typename gui_util::HtmlEscape(rcp.label)));
                    recipientElement.append(QString(" (%1)").arg(address));
                }
                else // just address
                {
                    recipientElement.append(tr("%1 to %2").arg(amount, address));
                }
            }
            formatted.append(recipientElement);
        }

        if (model->wallet().privateKeysDisabled() && !model->wallet().hasExternalSigner()) {
            question_string.append(tr("Do you want to draft this transaction?"));
        } else {
            question_string.append(tr("Are you sure you want to send?"));
        }

        question_string.append("<br /><span style='font-size:10pt;'>");
        if (model->wallet().privateKeysDisabled() && !model->wallet().hasExternalSigner()) {
            question_string.append(tr("Please, review your transaction proposal. This will produce a Partially Signed Bitcoin Transaction (PSBT) which you can save or copy and then sign with e.g. an offline %1 wallet, or a PSBT-compatible hardware wallet.").arg(PACKAGE_NAME));
        } else {
            question_string.append(tr("Please, review your transaction."));
        }
        question_string.append("</span>%1");

        if(txFee > 0)
        {
            // append fee string if a fee is required
            question_string.append("<hr /><b>");
            question_string.append(tr("Transaction fee"));
            question_string.append("</b>");

            // append transaction size
            question_string.append(" (" + QString::number((double)m_current_transaction->getTransactionSize() / 1000) + " kB): ");

            // append transaction fee value
            question_string.append("<span style='color:#aa0000; font-weight:bold;'>");
            question_string.append(BitcoinUnits::formatHtmlWithUnit(model->getOptionsModel()->getDisplayUnit(), txFee));
            question_string.append("</span><br />");

            // append RBF message according to transaction's signalling
            question_string.append("<span style='font-size:10pt; font-weight:normal;'>");
            if (ui->optInRBF->isChecked()) {
                question_string.append(tr("You can increase the fee later (signals Replace-By-Fee, BIP-125)."));
            } else {
                question_string.append(tr("Not signalling Replace-By-Fee, BIP-125."));
            }
            question_string.append("</span>");
        }

        // add total amount in all subdivision units
        question_string.append("<hr />");
        CAmount totalAmount = m_current_transaction->getTotalTransactionAmount() + txFee;
        QStringList alternativeUnits;
        for (const BitcoinUnits::Unit u : BitcoinUnits::availableUnits())
        {
            if(u != model->getOptionsModel()->getDisplayUnit())
                alternativeUnits.append(BitcoinUnits::formatHtmlWithUnit(u, totalAmount));
        }
        question_string.append(QString("<b>%1</b>: <b>%2</b>").arg(tr("Total Amount"))
            .arg(BitcoinUnits::formatHtmlWithUnit(model->getOptionsModel()->getDisplayUnit(), totalAmount)));
        question_string.append(QString("<br /><span style='font-size:10pt; font-weight:normal;'>(=%1)</span>")
            .arg(alternativeUnits.join(" " + tr("or") + " ")));

        if (formatted.size() > 1) {
            question_string = question_string.arg("");
            informative_text = tr("To review recipient list click \"Show Details…\"");
            detailed_text = formatted.join("\n\n");
        } else {
            question_string = question_string.arg("<br /><br />" + formatted.at(0));
        }

        return true;
        */
    }
    
    #[Q_SLOT]
    pub fn send_button_clicked(&mut self, checked: bool)  {
        
        todo!();
        /*
            if(!model || !model->getOptionsModel())
            return;

        QString question_string, informative_text, detailed_text;
        if (!PrepareSendText(question_string, informative_text, detailed_text)) return;
        assert(m_current_transaction);

        const QString confirmation = model->wallet().privateKeysDisabled() && !model->wallet().hasExternalSigner() ? tr("Confirm transaction proposal") : tr("Confirm send coins");
        const QString confirmButtonText = model->wallet().privateKeysDisabled() && !model->wallet().hasExternalSigner() ? tr("Create Unsigned") : tr("Sign and send");
        auto confirmationDialog = new SendConfirmationDialog(confirmation, question_string, informative_text, detailed_text, SEND_CONFIRM_DELAY, confirmButtonText, this);
        confirmationDialog->setAttribute(QtWA_DeleteOnClose);
        // TODO: Replace QDialog::exec() with safer QDialog::show().
        const auto retval = static_cast<QMessageBox::StandardButton>(confirmationDialog->exec());

        if(retval != QMessageBox::Yes)
        {
            fNewRecipientAllowed = true;
            return;
        }

        bool send_failure = false;
        if (model->wallet().privateKeysDisabled()) {
            CMutableTransaction mtx = CMutableTransaction{*(m_current_transaction->getWtx())};
            PartiallySignedTransaction psbtx(mtx);
            bool complete = false;
            // Always fill without signing first. This prevents an external signer
            // from being called prematurely and is not expensive.
            TransactionError err = model->wallet().fillPSBT(SIGHASH_ALL, false /* sign */, true /* bip32derivs */, nullptr, psbtx, complete);
            assert(!complete);
            assert(err == TransactionError::OK);
            if (model->wallet().hasExternalSigner()) {
                try {
                    err = model->wallet().fillPSBT(SIGHASH_ALL, true /* sign */, true /* bip32derivs */, nullptr, psbtx, complete);
                } catch (const std::runtime_error& e) {
                    QMessageBox::critical(nullptr, tr("Sign failed"), e.what());
                    send_failure = true;
                    return;
                }
                if (err == TransactionError::EXTERNAL_SIGNER_NOT_FOUND) {
                    //: "External signer" means using devices such as hardware wallets.
                    QMessageBox::critical(nullptr, tr("External signer not found"), "External signer not found");
                    send_failure = true;
                    return;
                }
                if (err == TransactionError::EXTERNAL_SIGNER_FAILED) {
                    //: "External signer" means using devices such as hardware wallets.
                    QMessageBox::critical(nullptr, tr("External signer failure"), "External signer failure");
                    send_failure = true;
                    return;
                }
                if (err != TransactionError::OK) {
                    tfm::format(std::cerr, "Failed to sign PSBT");
                    processSendCoinsReturn(WalletModel::TransactionCreationFailed);
                    send_failure = true;
                    return;
                }
                // fillPSBT does not always properly finalize
                complete = FinalizeAndExtractPSBT(psbtx, mtx);
            }

            // Broadcast transaction if complete (even with an external signer this
            // is not always the case, e.g. in a multisig wallet).
            if (complete) {
                const CTransactionRef tx = MakeTransactionRef(mtx);
                m_current_transaction->setWtx(tx);
                WalletModel::SendCoinsReturn sendStatus = model->sendCoins(*m_current_transaction);
                // process sendStatus and on error generate message shown to user
                processSendCoinsReturn(sendStatus);

                if (sendStatus.status == WalletModel::OK) {
                    Q_EMIT coinsSent(m_current_transaction->getWtx()->GetHash());
                } else {
                    send_failure = true;
                }
                return;
            }

            // Copy PSBT to clipboard and offer to save
            assert(!complete);
            // Serialize the PSBT
            DataStream ssTx(SER_NETWORK, PROTOCOL_VERSION);
            ssTx << psbtx;
            typename gui_util::setClipboard(EncodeBase64(ssTx.str()).c_str());
            QMessageBox msgBox;
            msgBox.setText("Unsigned Transaction");
            msgBox.setInformativeText("The PSBT has been copied to the clipboard. You can also save it.");
            msgBox.setStandardButtons(QMessageBox::Save | QMessageBox::Discard);
            msgBox.setDefaultButton(QMessageBox::Discard);
            switch (msgBox.exec()) {
            case QMessageBox::Save: {
                QString selectedFilter;
                QString fileNameSuggestion = "";
                bool first = true;
                for (const SendCoinsRecipient &rcp : m_current_transaction->getRecipients()) {
                    if (!first) {
                        fileNameSuggestion.append(" - ");
                    }
                    QString labelOrAddress = rcp.label.isEmpty() ? rcp.address : rcp.label;
                    QString amount = BitcoinUnits::formatWithUnit(model->getOptionsModel()->getDisplayUnit(), rcp.amount);
                    fileNameSuggestion.append(labelOrAddress + "-" + amount);
                    first = false;
                }
                fileNameSuggestion.append(".psbt");
                QString filename = typename gui_util::getSaveFileName(this,
                    tr("Save Transaction Data"), fileNameSuggestion,
                    //: Expanded name of the binary PSBT file format. See: BIP 174.
                    tr("Partially Signed Transaction (Binary)") + QLatin1String(" (*.psbt)"), &selectedFilter);
                if (filename.isEmpty()) {
                    return;
                }
                std::ofstream out(filename.toLocal8Bit().data(), std::ofstream::out | std::ofstream::binary);
                out << ssTx.str();
                out.close();
                Q_EMIT message(tr("PSBT saved"), "PSBT saved to disk", CClientUIInterface::MSG_INFORMATION);
                break;
            }
            case QMessageBox::Discard:
                break;
            default:
                assert(false);
            } // msgBox.exec()
        } else {
            // now send the prepared transaction
            WalletModel::SendCoinsReturn sendStatus = model->sendCoins(*m_current_transaction);
            // process sendStatus and on error generate message shown to user
            processSendCoinsReturn(sendStatus);

            if (sendStatus.status == WalletModel::OK) {
                Q_EMIT coinsSent(m_current_transaction->getWtx()->GetHash());
            } else {
                send_failure = true;
            }
        }
        if (!send_failure) {
            accept();
            m_coin_control->UnSelectAll();
            coinControlUpdateLabels();
        }
        fNewRecipientAllowed = true;
        m_current_transaction.reset();
        */
    }
    
    #[Q_SLOT]
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            m_current_transaction.reset();

        // Clear coin control settings
        m_coin_control->UnSelectAll();
        ui->checkBoxCoinControlChange->setChecked(false);
        ui->lineEditCoinControlChange->clear();
        coinControlUpdateLabels();

        // Remove entries until only one left
        while(ui->entries->count())
        {
            ui->entries->takeAt(0)->widget()->deleteLater();
        }
        addEntry();

        updateTabsAndLabels();
        */
    }
    
    #[Q_SLOT]
    pub fn reject(&mut self)  {
        
        todo!();
        /*
            clear();
        */
    }
    
    #[Q_SLOT]
    pub fn accept(&mut self)  {
        
        todo!();
        /*
            clear();
        */
    }
    
    #[Q_SLOT]
    pub fn add_entry(&mut self) -> *mut SendCoinsEntry {
        
        todo!();
        /*
            SendCoinsEntry *entry = new SendCoinsEntry(platformStyle, this);
        entry->setModel(model);
        ui->entries->addWidget(entry);
        connect(entry, &SendCoinsEntry::removeEntry, this, &SendCoinsDialog::removeEntry);
        connect(entry, &SendCoinsEntry::useAvailableBalance, this, &SendCoinsDialog::useAvailableBalance);
        connect(entry, &SendCoinsEntry::payAmountChanged, this, &SendCoinsDialog::coinControlUpdateLabels);
        connect(entry, &SendCoinsEntry::subtractFeeFromAmountChanged, this, &SendCoinsDialog::coinControlUpdateLabels);

        // Focus the field, so that entry can start immediately
        entry->clear();
        entry->setFocus();
        ui->scrollAreaWidgetContents->resize(ui->scrollAreaWidgetContents->sizeHint());
        qApp->processEvents();
        QScrollBar* bar = ui->scrollArea->verticalScrollBar();
        if(bar)
            bar->setSliderPosition(bar->maximum());

        updateTabsAndLabels();
        return entry;
        */
    }
    
    #[Q_SLOT]
    pub fn update_tabs_and_labels(&mut self)  {
        
        todo!();
        /*
            setupTabChain(nullptr);
        coinControlUpdateLabels();
        */
    }
    
    #[Q_SLOT]
    pub fn remove_entry(&mut self, entry: *mut SendCoinsEntry)  {
        
        todo!();
        /*
            entry->hide();

        // If the last entry is about to be removed add an empty one
        if (ui->entries->count() == 1)
            addEntry();

        entry->deleteLater();

        updateTabsAndLabels();
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
            for(int i = 0; i < ui->entries->count(); ++i)
        {
            SendCoinsEntry *entry = qobject_cast<SendCoinsEntry*>(ui->entries->itemAt(i)->widget());
            if(entry)
            {
                prev = entry->setupTabChain(prev);
            }
        }
        QWidget::setTabOrder(prev, ui->sendButton);
        QWidget::setTabOrder(ui->sendButton, ui->clearButton);
        QWidget::setTabOrder(ui->clearButton, ui->addButton);
        return ui->addButton;
        */
    }
    
    pub fn set_address(&mut self, address: &String)  {
        
        todo!();
        /*
            SendCoinsEntry *entry = nullptr;
        // Replace the first entry if it is still unused
        if(ui->entries->count() == 1)
        {
            SendCoinsEntry *first = qobject_cast<SendCoinsEntry*>(ui->entries->itemAt(0)->widget());
            if(first->isClear())
            {
                entry = first;
            }
        }
        if(!entry)
        {
            entry = addEntry();
        }

        entry->setAddress(address);
        */
    }
    
    pub fn paste_entry(&mut self, rv: &SendCoinsRecipient)  {
        
        todo!();
        /*
            if(!fNewRecipientAllowed)
            return;

        SendCoinsEntry *entry = nullptr;
        // Replace the first entry if it is still unused
        if(ui->entries->count() == 1)
        {
            SendCoinsEntry *first = qobject_cast<SendCoinsEntry*>(ui->entries->itemAt(0)->widget());
            if(first->isClear())
            {
                entry = first;
            }
        }
        if(!entry)
        {
            entry = addEntry();
        }

        entry->setValue(rv);
        updateTabsAndLabels();
        */
    }
    
    pub fn handle_payment_request(&mut self, rv: &SendCoinsRecipient) -> bool {
        
        todo!();
        /*
            // Just paste the entry, all pre-checks
        // are done in paymentserver.cpp.
        pasteEntry(rv);
        return true;
        */
    }
    
    #[Q_SLOT]
    pub fn set_balance(&mut self, balances: &WalletBalances)  {
        
        todo!();
        /*
            if(model && model->getOptionsModel())
        {
            CAmount balance = balances.balance;
            if (model->wallet().hasExternalSigner()) {
                ui->labelBalanceName->setText(tr("External balance:"));
            } else if (model->wallet().privateKeysDisabled()) {
                balance = balances.watch_only_balance;
                ui->labelBalanceName->setText(tr("Watch-only balance:"));
            }
            ui->labelBalance->setText(BitcoinUnits::formatWithUnit(model->getOptionsModel()->getDisplayUnit(), balance));
        }
        */
    }
    
    #[Q_SLOT]
    pub fn update_display_unit(&mut self)  {
        
        todo!();
        /*
            setBalance(model->wallet().getBalances());
        ui->customFee->setDisplayUnit(model->getOptionsModel()->getDisplayUnit());
        updateSmartFeeLabel();
        */
    }
    
    /**
      | Process WalletModel::SendCoinsReturn and
      | generate a pair consisting of a message and
      | message flags for use in Q_EMIT message().
      |
      | Additional parameter msgArg can be used via
      | .arg(msgArg).
      */
    #[Q_SIGNAL]
    pub fn process_send_coins_return(&mut self, 
        send_coins_return: &wallet_model::SendCoinsReturn,
        msg_arg:           &String)  {
        
        todo!();
        /*
            QPair<QString, CClientUIInterface::MessageBoxFlags> msgParams;
        // Default to a warning message, override if error message is needed
        msgParams.second = CClientUIInterface::MSG_WARNING;

        // This comment is specific to SendCoinsDialog usage of WalletModel::SendCoinsReturn.
        // All status values are used only in WalletModel::prepareTransaction()
        switch(sendCoinsReturn.status)
        {
        case WalletModel::InvalidAddress:
            msgParams.first = tr("The recipient address is not valid. Please recheck.");
            break;
        case WalletModel::InvalidAmount:
            msgParams.first = tr("The amount to pay must be larger than 0.");
            break;
        case WalletModel::AmountExceedsBalance:
            msgParams.first = tr("The amount exceeds your balance.");
            break;
        case WalletModel::AmountWithFeeExceedsBalance:
            msgParams.first = tr("The total exceeds your balance when the %1 transaction fee is included.").arg(msgArg);
            break;
        case WalletModel::DuplicateAddress:
            msgParams.first = tr("Duplicate address found: addresses should only be used once each.");
            break;
        case WalletModel::TransactionCreationFailed:
            msgParams.first = tr("Transaction creation failed!");
            msgParams.second = CClientUIInterface::MSG_ERROR;
            break;
        case WalletModel::AbsurdFee:
            msgParams.first = tr("A fee higher than %1 is considered an absurdly high fee.").arg(BitcoinUnits::formatWithUnit(model->getOptionsModel()->getDisplayUnit(), model->wallet().getDefaultMaxTxFee()));
            break;
        case WalletModel::PaymentRequestExpired:
            msgParams.first = tr("Payment request expired.");
            msgParams.second = CClientUIInterface::MSG_ERROR;
            break;
        // included to prevent a compiler warning.
        case WalletModel::OK:
        default:
            return;
        }

        Q_EMIT message(tr("Send Coins"), msgParams.first, msgParams.second);
        */
    }
    
    #[Q_SIGNAL]
    pub fn minimize_fee_section(&mut self, minimize: bool)  {
        
        todo!();
        /*
            ui->labelFeeMinimized->setVisible(fMinimize);
        ui->buttonChooseFee  ->setVisible(fMinimize);
        ui->buttonMinimizeFee->setVisible(!fMinimize);
        ui->frameFeeSelection->setVisible(!fMinimize);
        ui->horizontalLayoutSmartFee->setContentsMargins(0, (fMinimize ? 0 : 6), 0, 0);
        fFeeMinimized = fMinimize;
        */
    }
    
    #[Q_SLOT]
    pub fn on_button_choose_fee_clicked(&mut self)  {
        
        todo!();
        /*
            minimizeFeeSection(false);
        */
    }
    
    #[Q_SLOT]
    pub fn on_button_minimize_fee_clicked(&mut self)  {
        
        todo!();
        /*
            updateFeeMinimizedLabel();
        minimizeFeeSection(true);
        */
    }
    
    #[Q_SLOT]
    pub fn use_available_balance(&mut self, entry: *mut SendCoinsEntry)  {
        
        todo!();
        /*
            // Include watch-only for wallets without private key
        m_coin_control->fAllowWatchOnly = model->wallet().privateKeysDisabled() && !model->wallet().hasExternalSigner();

        // Calculate available amount to send.
        CAmount amount = model->wallet().getAvailableBalance(*m_coin_control);
        for (int i = 0; i < ui->entries->count(); ++i) {
            SendCoinsEntry* e = qobject_cast<SendCoinsEntry*>(ui->entries->itemAt(i)->widget());
            if (e && !e->isHidden() && e != entry) {
                amount -= e->getValue().amount;
            }
        }

        if (amount > 0) {
          entry->checkSubtractFeeFromAmount();
          entry->setAmount(amount);
        } else {
          entry->setAmount(0);
        }
        */
    }
    
    #[Q_SLOT]
    pub fn update_fee_section_controls(&mut self)  {
        
        todo!();
        /*
            ui->confTargetSelector      ->setEnabled(ui->radioSmartFee->isChecked());
        ui->labelSmartFee           ->setEnabled(ui->radioSmartFee->isChecked());
        ui->labelSmartFee2          ->setEnabled(ui->radioSmartFee->isChecked());
        ui->labelSmartFee3          ->setEnabled(ui->radioSmartFee->isChecked());
        ui->labelFeeEstimation      ->setEnabled(ui->radioSmartFee->isChecked());
        ui->labelCustomFeeWarning   ->setEnabled(ui->radioCustomFee->isChecked());
        ui->labelCustomPerKilobyte  ->setEnabled(ui->radioCustomFee->isChecked());
        ui->customFee               ->setEnabled(ui->radioCustomFee->isChecked());
        */
    }
    
    #[Q_SIGNAL]
    pub fn update_fee_minimized_label(&mut self)  {
        
        todo!();
        /*
            if(!model || !model->getOptionsModel())
            return;

        if (ui->radioSmartFee->isChecked())
            ui->labelFeeMinimized->setText(ui->labelSmartFee->text());
        else {
            ui->labelFeeMinimized->setText(BitcoinUnits::formatWithUnit(model->getOptionsModel()->getDisplayUnit(), ui->customFee->value()) + "/kvB");
        }
        */
    }
    
    #[Q_SIGNAL]
    pub fn update_coin_control_state(&mut self)  {
        
        todo!();
        /*
            if (ui->radioCustomFee->isChecked()) {
            m_coin_control->m_feerate = CFeeRate(ui->customFee->value());
        } else {
            m_coin_control->m_feerate.reset();
        }
        // Avoid using global defaults when sending money from the GUI
        // Either custom fee will be used or if not selected, the confirmation target from dropdown box
        m_coin_control->m_confirm_target = getConfTargetForIndex(ui->confTargetSelector->currentIndex());
        m_coin_control->m_signal_bip125_rbf = ui->optInRBF->isChecked();
        // Include watch-only for wallets without private key
        m_coin_control->fAllowWatchOnly = model->wallet().privateKeysDisabled() && !model->wallet().hasExternalSigner();
        */
    }
    
    #[Q_SLOT]
    pub fn update_number_of_blocks(&mut self, 
        count:                   i32,
        block_date:              &QDateTime,
        n_verification_progress: f64,
        headers:                 bool,
        sync_state:              SynchronizationState)  {
        
        todo!();
        /*
            if (sync_state == SynchronizationState::POST_INIT) {
            updateSmartFeeLabel();
        }
        */
    }
    
    #[Q_SLOT]
    pub fn update_smart_fee_label(&mut self)  {
        
        todo!();
        /*
            if(!model || !model->getOptionsModel())
            return;
        updateCoinControlState();
        m_coin_control->m_feerate.reset(); // Explicitly use only fee estimation rate for smart fee labels
        int returned_target;
        FeeReason reason;
        CFeeRate feeRate = CFeeRate(model->wallet().getMinimumFee(1000, *m_coin_control, &returned_target, &reason));

        ui->labelSmartFee->setText(BitcoinUnits::formatWithUnit(model->getOptionsModel()->getDisplayUnit(), feeRate.GetFeePerK()) + "/kvB");

        if (reason == FeeReason::FALLBACK) {
            ui->labelSmartFee2->show(); // (Smart fee not initialized yet. This usually takes a few blocks...)
            ui->labelFeeEstimation->setText("");
            ui->fallbackFeeWarningLabel->setVisible(true);
            int lightness = ui->fallbackFeeWarningLabel->palette().color(QPalette::WindowText).lightness();
            QColor warning_colour(255 - (lightness / 5), 176 - (lightness / 3), 48 - (lightness / 14));
            ui->fallbackFeeWarningLabel->setStyleSheet("QLabel { color: " + warning_colour.name() + "; }");
            ui->fallbackFeeWarningLabel->setIndent(typename gui_util::TextWidth(QFontMetrics(ui->fallbackFeeWarningLabel->font()), "x"));
        }
        else
        {
            ui->labelSmartFee2->hide();
            ui->labelFeeEstimation->setText(tr("Estimated to begin confirmation within %n block(s).", "", returned_target));
            ui->fallbackFeeWarningLabel->setVisible(false);
        }

        updateFeeMinimizedLabel();
        */
    }

    /**
      | Coin Control: copy label "Quantity"
      | to clipboard
      |
      */
    #[Q_SLOT]
    pub fn coin_control_clipboard_quantity(&mut self)  {
        
        todo!();
        /*
            typename gui_util::setClipboard(ui->labelCoinControlQuantity->text());
        */
    }

    /**
      | Coin Control: copy label "Amount" to
      | clipboard
      |
      */
    #[Q_SLOT]
    pub fn coin_control_clipboard_amount(&mut self)  {
        
        todo!();
        /*
            typename gui_util::setClipboard(ui->labelCoinControlAmount->text().left(ui->labelCoinControlAmount->text().indexOf(" ")));
        */
    }

    /**
      | Coin Control: copy label "Fee" to clipboard
      |
      */
    #[Q_SLOT]
    pub fn coin_control_clipboard_fee(&mut self)  {
        
        todo!();
        /*
            typename gui_util::setClipboard(ui->labelCoinControlFee->text().left(ui->labelCoinControlFee->text().indexOf(" ")).replace(ASYMP_UTF8, ""));
        */
    }

    /**
      | Coin Control: copy label "After fee"
      | to clipboard
      |
      */
    #[Q_SLOT]
    pub fn coin_control_clipboard_after_fee(&mut self)  {
        
        todo!();
        /*
            typename gui_util::setClipboard(ui->labelCoinControlAfterFee->text().left(ui->labelCoinControlAfterFee->text().indexOf(" ")).replace(ASYMP_UTF8, ""));
        */
    }

    /**
      | Coin Control: copy label "Bytes" to
      | clipboard
      |
      */
    #[Q_SLOT]
    pub fn coin_control_clipboard_bytes(&mut self)  {
        
        todo!();
        /*
            typename gui_util::setClipboard(ui->labelCoinControlBytes->text().replace(ASYMP_UTF8, ""));
        */
    }

    /**
      | Coin Control: copy label "Dust" to clipboard
      |
      */
    #[Q_SLOT]
    pub fn coin_control_clipboard_low_output(&mut self)  {
        
        todo!();
        /*
            typename gui_util::setClipboard(ui->labelCoinControlLowOutput->text());
        */
    }

    /**
      | Coin Control: copy label "Change" to
      | clipboard
      |
      */
    #[Q_SLOT]
    pub fn coin_control_clipboard_change(&mut self)  {
        
        todo!();
        /*
            typename gui_util::setClipboard(ui->labelCoinControlChange->text().left(ui->labelCoinControlChange->text().indexOf(" ")).replace(ASYMP_UTF8, ""));
        */
    }

    /**
      | Coin Control: settings menu - coin control
      | enabled/disabled by user
      |
      */
    #[Q_SLOT]
    pub fn coin_control_feature_changed(&mut self, checked: bool)  {
        
        todo!();
        /*
            ui->frameCoinControl->setVisible(checked);

        if (!checked && model) { // coin control features disabled
            m_coin_control = std::make_unique<CCoinControl>();
        }

        coinControlUpdateLabels();
        */
    }

    /**
      | Coin Control: button inputs -> show
      | actual coin control dialog
      |
      */
    #[Q_SLOT]
    pub fn coin_control_button_clicked(&mut self)  {
        
        todo!();
        /*
            auto dlg = new CoinControlDialog(*m_coin_control, model, platformStyle);
        connect(dlg, &QDialog::finished, this, &SendCoinsDialog::coinControlUpdateLabels);
        typename gui_util::ShowModalDialogAndDeleteOnClose(dlg);
        */
    }

    /**
      | Coin Control: checkbox custom change
      | address
      |
      */
    #[Q_SLOT]
    pub fn coin_control_change_checked(&mut self, state: i32)  {
        
        todo!();
        /*
            if (state == QtUnchecked)
        {
            m_coin_control->destChange = CNoDestination();
            ui->labelCoinControlChangeLabel->clear();
        }
        else
            // use this to re-validate an already entered address
            coinControlChangeEdited(ui->lineEditCoinControlChange->text());

        ui->lineEditCoinControlChange->setEnabled((state == QtChecked));
        */
    }

    /**
      | Coin Control: custom change address
      | changed
      |
      */
    #[Q_SLOT]
    pub fn coin_control_change_edited(&mut self, text: &String)  {
        
        todo!();
        /*
            if (model && model->getAddressTableModel())
        {
            // Default to no change address until verified
            m_coin_control->destChange = CNoDestination();
            ui->labelCoinControlChangeLabel->setStyleSheet("QLabel{color:red;}");

            const TxDestination dest = DecodeDestination(text.toStdString());

            if (text.isEmpty()) // Nothing entered
            {
                ui->labelCoinControlChangeLabel->setText("");
            }
            else if (!IsValidDestination(dest)) // Invalid address
            {
                ui->labelCoinControlChangeLabel->setText(tr("Warning: Invalid Bitcoin address"));
            }
            else // Valid address
            {
                if (!model->wallet().isSpendable(dest)) {
                    ui->labelCoinControlChangeLabel->setText(tr("Warning: Unknown change address"));

                    // confirmation dialog
                    QMessageBox::StandardButton btnRetVal = QMessageBox::question(this, tr("Confirm custom change address"), tr("The address you selected for change is not part of this wallet. Any or all funds in your wallet may be sent to this address. Are you sure?"),
                        QMessageBox::Yes | QMessageBox::Cancel, QMessageBox::Cancel);

                    if(btnRetVal == QMessageBox::Yes)
                        m_coin_control->destChange = dest;
                    else
                    {
                        ui->lineEditCoinControlChange->setText("");
                        ui->labelCoinControlChangeLabel->setStyleSheet("QLabel{color:black;}");
                        ui->labelCoinControlChangeLabel->setText("");
                    }
                }
                else // Known change address
                {
                    ui->labelCoinControlChangeLabel->setStyleSheet("QLabel{color:black;}");

                    // Query label
                    QString associatedLabel = model->getAddressTableModel()->labelForAddress(text);
                    if (!associatedLabel.isEmpty())
                        ui->labelCoinControlChangeLabel->setText(associatedLabel);
                    else
                        ui->labelCoinControlChangeLabel->setText(tr("(no label)"));

                    m_coin_control->destChange = dest;
                }
            }
        }
        */
    }

    /**
      | Coin Control: update labels
      |
      */
    #[Q_SLOT]
    pub fn coin_control_update_labels(&mut self)  {
        
        todo!();
        /*
            if (!model || !model->getOptionsModel())
            return;

        updateCoinControlState();

        // set pay amounts
        CoinControlDialog::payAmounts.clear();
        CoinControlDialog::fSubtractFeeFromAmount = false;

        for(int i = 0; i < ui->entries->count(); ++i)
        {
            SendCoinsEntry *entry = qobject_cast<SendCoinsEntry*>(ui->entries->itemAt(i)->widget());
            if(entry && !entry->isHidden())
            {
                SendCoinsRecipient rcp = entry->getValue();
                CoinControlDialog::payAmounts.append(rcp.amount);
                if (rcp.fSubtractFeeFromAmount)
                    CoinControlDialog::fSubtractFeeFromAmount = true;
            }
        }

        if (m_coin_control->HasSelected())
        {
            // actual coin control calculation
            CoinControlDialog::updateLabels(*m_coin_control, model, this);

            // show coin control stats
            ui->labelCoinControlAutomaticallySelected->hide();
            ui->widgetCoinControl->show();
        }
        else
        {
            // hide coin control stats
            ui->labelCoinControlAutomaticallySelected->show();
            ui->widgetCoinControl->hide();
            ui->labelCoinControlInsuffFunds->hide();
        }
        */
    }
}

impl SendConfirmationDialog {

    pub fn new(
        title:               &String,
        text:                &String,
        informative_text:    Option<&str>,
        detailed_text:       Option<&str>,
        sec_delay:           Option<i32>,
        confirm_button_text: Option<&str>,
        parent:              *mut QWidget) -> Self {

        let informative_text:     &str = informative_text.unwrap_or("");
        let detailed_text:        &str = detailed_text.unwrap_or("");
        let sec_delay:            i32  = sec_delay.unwrap_or(SEND_CONFIRM_DELAY.try_into().unwrap());
        let confirm_button_text:  &str = confirm_button_text.unwrap_or("");
    
        todo!();
        /*


            : QMessageBox(parent), secDelay(_secDelay), confirmButtonText(_confirmButtonText)

        setIcon(QMessageBox::Question);
        setWindowTitle(title); // On macOS, the window title is ignored (as required by the macOS Guidelines).
        setText(text);
        setInformativeText(informative_text);
        setDetailedText(detailed_text);
        setStandardButtons(QMessageBox::Yes | QMessageBox::Cancel);
        setDefaultButton(QMessageBox::Cancel);
        yesButton = button(QMessageBox::Yes);
        if (confirmButtonText.isEmpty()) {
            confirmButtonText = yesButton->text();
        }
        updateYesButton();
        connect(&countDownTimer, &QTimer::timeout, this, &SendConfirmationDialog::countDown);
        */
    }
    
    pub fn exec(&mut self) -> i32 {
        
        todo!();
        /*
            updateYesButton();
        countDownTimer.start(1000);
        return QMessageBox::exec();
        */
    }
    
    #[Q_SLOT]
    pub fn count_down(&mut self)  {
        
        todo!();
        /*
            secDelay--;
        updateYesButton();

        if(secDelay <= 0)
        {
            countDownTimer.stop();
        }
        */
    }
    
    #[Q_SLOT]
    pub fn update_yes_button(&mut self)  {
        
        todo!();
        /*
            if(secDelay > 0)
        {
            yesButton->setEnabled(false);
            yesButton->setText(confirmButtonText + " (" + QString::number(secDelay) + ")");
        }
        else
        {
            yesButton->setEnabled(true);
            yesButton->setText(confirmButtonText);
        }
        */
    }
}
