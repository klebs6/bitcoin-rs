// ---------------- [ File: bitcoin-qt/src/psbtoperationsdialog.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/psbtoperationsdialog.h]

/**
  | Dialog showing transaction details.
  |
  */
#[Q_OBJECT]
pub struct PSBTOperationsDialog {
    base:             QDialog,
    ui:               *mut UiPSBTOperationsDialog,
    transaction_data: PartiallySignedTransaction,
    wallet_model:     *mut WalletModel,
    client_model:     *mut ClientModel,
}

pub mod psbt_operations_dialog {

    pub enum StatusLevel {
        INFO,
        WARN,
        ERR
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/psbtoperationsdialog.cpp]
impl Drop for PSBTOperationsDialog {
    fn drop(&mut self) {
        todo!();
        /*
            delete m_ui;
        */
    }
}

impl PSBTOperationsDialog {

    pub fn new(
        parent:       *mut QWidget,
        wallet_model: *mut WalletModel,
        client_model: *mut ClientModel) -> Self {
    
        todo!();
        /*


            : QDialog(parent, typename gui_util::dialog_flags),
                                                                                 m_ui(new UiPSBTOperationsDialog),
                                                                                 m_wallet_model(wallet_model),
                                                                                 m_client_model(client_model)
        m_ui->setupUi(this);
        setWindowTitle("PSBT Operations");

        connect(m_ui->signTransactionButton, &QPushButton::clicked, this, &PSBTOperationsDialog::signTransaction);
        connect(m_ui->broadcastTransactionButton, &QPushButton::clicked, this, &PSBTOperationsDialog::broadcastTransaction);
        connect(m_ui->copyToClipboardButton, &QPushButton::clicked, this, &PSBTOperationsDialog::copyToClipboard);
        connect(m_ui->saveButton, &QPushButton::clicked, this, &PSBTOperationsDialog::saveTransaction);

        connect(m_ui->closeButton, &QPushButton::clicked, this, &PSBTOperationsDialog::close);

        m_ui->signTransactionButton->setEnabled(false);
        m_ui->broadcastTransactionButton->setEnabled(false);
        */
    }
    
    pub fn open_withpsbt(&mut self, psbtx: PartiallySignedTransaction)  {
        
        todo!();
        /*
            m_transaction_data = psbtx;

        bool complete = FinalizePSBT(psbtx); // Make sure all existing signatures are fully combined before checking for completeness.
        if (m_wallet_model) {
            size_t n_could_sign;
            TransactionError err = m_wallet_model->wallet().fillPSBT(SIGHASH_ALL, false /* sign */, true /* bip32derivs */, &n_could_sign, m_transaction_data, complete);
            if (err != TransactionError::OK) {
                showStatus(tr("Failed to load transaction: %1")
                               .arg(QString::fromStdString(TransactionErrorString(err).translated)),
                           StatusLevel::ERR);
                return;
            }
            m_ui->signTransactionButton->setEnabled(!complete && !m_wallet_model->wallet().privateKeysDisabled() && n_could_sign > 0);
        } else {
            m_ui->signTransactionButton->setEnabled(false);
        }

        m_ui->broadcastTransactionButton->setEnabled(complete);

        updateTransactionDisplay();
        */
    }
    
    #[Q_SLOT]
    pub fn sign_transaction(&mut self)  {
        
        todo!();
        /*
            bool complete;
        size_t n_signed;

        WalletModel::UnlockContext ctx(m_wallet_model->requestUnlock());

        TransactionError err = m_wallet_model->wallet().fillPSBT(SIGHASH_ALL, true /* sign */, true /* bip32derivs */, &n_signed, m_transaction_data, complete);

        if (err != TransactionError::OK) {
            showStatus(tr("Failed to sign transaction: %1")
                .arg(QString::fromStdString(TransactionErrorString(err).translated)), StatusLevel::ERR);
            return;
        }

        updateTransactionDisplay();

        if (!complete && !ctx.isValid()) {
            showStatus(tr("Cannot sign inputs while wallet is locked."), StatusLevel::WARN);
        } else if (!complete && n_signed < 1) {
            showStatus(tr("Could not sign any more inputs."), StatusLevel::WARN);
        } else if (!complete) {
            showStatus(tr("Signed %1 inputs, but more signatures are still required.").arg(n_signed),
                StatusLevel::INFO);
        } else {
            showStatus(tr("Signed transaction successfully. Transaction is ready to broadcast."),
                StatusLevel::INFO);
            m_ui->broadcastTransactionButton->setEnabled(true);
        }
        */
    }
    
    #[Q_SLOT]
    pub fn broadcast_transaction(&mut self)  {
        
        todo!();
        /*
            CMutableTransaction mtx;
        if (!FinalizeAndExtractPSBT(m_transaction_data, mtx)) {
            // This is never expected to fail unless we were given a malformed PSBT
            // (e.g. with an invalid signature.)
            showStatus(tr("Unknown error processing transaction."), StatusLevel::ERR);
            return;
        }

        CTransactionRef tx = MakeTransactionRef(mtx);
        std::string err_string;
        TransactionError error = BroadcastTransaction(
            *m_client_model->node().context(), tx, err_string, DEFAULT_MAX_RAW_TX_FEE_RATE.GetFeePerK(), /* relay */ true, /* await_callback */ false);

        if (error == TransactionError::OK) {
            showStatus(tr("Transaction broadcast successfully! Transaction ID: %1")
                .arg(QString::fromStdString(tx->GetHash().GetHex())), StatusLevel::INFO);
        } else {
            showStatus(tr("Transaction broadcast failed: %1")
                .arg(QString::fromStdString(TransactionErrorString(error).translated)), StatusLevel::ERR);
        }
        */
    }
    
    #[Q_SLOT]
    pub fn copy_to_clipboard(&mut self)  {
        
        todo!();
        /*
            DataStream ssTx(SER_NETWORK, PROTOCOL_VERSION);
        ssTx << m_transaction_data;
        typename gui_util::setClipboard(EncodeBase64(ssTx.str()).c_str());
        showStatus(tr("PSBT copied to clipboard."), StatusLevel::INFO);
        */
    }
    
    #[Q_SLOT]
    pub fn save_transaction(&mut self)  {
        
        todo!();
        /*
            DataStream ssTx(SER_NETWORK, PROTOCOL_VERSION);
        ssTx << m_transaction_data;

        QString selected_filter;
        QString filename_suggestion = "";
        bool first = true;
        for (const CTxOut& out : m_transaction_data.tx->vout) {
            if (!first) {
                filename_suggestion.append("-");
            }
            TxDestination address;
            ExtractDestination(out.scriptPubKey, address);
            QString amount = BitcoinUnits::format(m_client_model->getOptionsModel()->getDisplayUnit(), out.nValue);
            QString address_str = QString::fromStdString(EncodeDestination(address));
            filename_suggestion.append(address_str + "-" + amount);
            first = false;
        }
        filename_suggestion.append(".psbt");
        QString filename = typename gui_util::getSaveFileName(this,
            tr("Save Transaction Data"), filename_suggestion,
            //: Expanded name of the binary PSBT file format. See: BIP 174.
            tr("Partially Signed Transaction (Binary)") + QLatin1String(" (*.psbt)"), &selected_filter);
        if (filename.isEmpty()) {
            return;
        }
        std::ofstream out(filename.toLocal8Bit().data(), std::ofstream::out | std::ofstream::binary);
        out << ssTx.str();
        out.close();
        showStatus(tr("PSBT saved to disk."), StatusLevel::INFO);
        */
    }
    
    pub fn update_transaction_display(&mut self)  {
        
        todo!();
        /*
            m_ui->transactionDescription->setText(QString::fromStdString(renderTransaction(m_transaction_data)));
        showTransactionStatus(m_transaction_data);
        */
    }
    
    pub fn render_transaction(&mut self, psbtx: &PartiallySignedTransaction) -> String {
        
        todo!();
        /*
            QString tx_description = "";
        CAmount totalAmount = 0;
        for (const CTxOut& out : psbtx.tx->vout) {
            TxDestination address;
            ExtractDestination(out.scriptPubKey, address);
            totalAmount += out.nValue;
            tx_description.append(tr(" * Sends %1 to %2")
                .arg(BitcoinUnits::formatWithUnit(BitcoinUnits::BTC, out.nValue))
                .arg(QString::fromStdString(EncodeDestination(address))));
            tx_description.append("<br>");
        }

        PSBTAnalysis analysis = AnalyzePSBT(psbtx);
        tx_description.append(" * ");
        if (!*analysis.fee) {
            // This happens if the transaction is missing input UTXO information.
            tx_description.append(tr("Unable to calculate transaction fee or total transaction amount."));
        } else {
            tx_description.append(tr("Pays transaction fee: "));
            tx_description.append(BitcoinUnits::formatWithUnit(BitcoinUnits::BTC, *analysis.fee));

            // add total amount in all subdivision units
            tx_description.append("<hr />");
            QStringList alternativeUnits;
            for (const BitcoinUnits::Unit u : BitcoinUnits::availableUnits())
            {
                if(u != m_client_model->getOptionsModel()->getDisplayUnit()) {
                    alternativeUnits.append(BitcoinUnits::formatHtmlWithUnit(u, totalAmount));
                }
            }
            tx_description.append(QString("<b>%1</b>: <b>%2</b>").arg(tr("Total Amount"))
                .arg(BitcoinUnits::formatHtmlWithUnit(m_client_model->getOptionsModel()->getDisplayUnit(), totalAmount)));
            tx_description.append(QString("<br /><span style='font-size:10pt; font-weight:normal;'>(=%1)</span>")
                .arg(alternativeUnits.join(" " + tr("or") + " ")));
        }

        size_t num_unsigned = CountPSBTUnsignedInputs(psbtx);
        if (num_unsigned > 0) {
            tx_description.append("<br><br>");
            tx_description.append(tr("Transaction has %1 unsigned inputs.").arg(QString::number(num_unsigned)));
        }

        return tx_description.toStdString();
        */
    }
    
    pub fn show_status(&mut self, 
        msg:   &String,
        level: psbt_operations_dialog::StatusLevel)  {
        
        todo!();
        /*
            m_ui->statusBar->setText(msg);
        switch (level) {
            case StatusLevel::INFO: {
                m_ui->statusBar->setStyleSheet("QLabel { background-color : lightgreen }");
                break;
            }
            case StatusLevel::WARN: {
                m_ui->statusBar->setStyleSheet("QLabel { background-color : orange }");
                break;
            }
            case StatusLevel::ERR: {
                m_ui->statusBar->setStyleSheet("QLabel { background-color : red }");
                break;
            }
        }
        m_ui->statusBar->show();
        */
    }
    
    pub fn could_sign_inputs(&mut self, psbtx: &PartiallySignedTransaction) -> usize {
        
        todo!();
        /*
            if (!m_wallet_model) {
            return 0;
        }

        size_t n_signed;
        bool complete;
        TransactionError err = m_wallet_model->wallet().fillPSBT(SIGHASH_ALL, false /* sign */, false /* bip32derivs */, &n_signed, m_transaction_data, complete);

        if (err != TransactionError::OK) {
            return 0;
        }
        return n_signed;
        */
    }
    
    pub fn show_transaction_status(&mut self, psbtx: &PartiallySignedTransaction)  {
        
        todo!();
        /*
            PSBTAnalysis analysis = AnalyzePSBT(psbtx);
        size_t n_could_sign = couldSignInputs(psbtx);

        switch (analysis.next) {
            case PSBTRole::UPDATER: {
                showStatus(tr("Transaction is missing some information about inputs."), StatusLevel::WARN);
                break;
            }
            case PSBTRole::SIGNER: {
                QString need_sig_text = tr("Transaction still needs signature(s).");
                StatusLevel level = StatusLevel::INFO;
                if (!m_wallet_model) {
                    need_sig_text += " " + tr("(But no wallet is loaded.)");
                    level = StatusLevel::WARN;
                } else if (m_wallet_model->wallet().privateKeysDisabled()) {
                    need_sig_text += " " + tr("(But this wallet cannot sign transactions.)");
                    level = StatusLevel::WARN;
                } else if (n_could_sign < 1) {
                    need_sig_text += " " + tr("(But this wallet does not have the right keys.)"); // XXX wording
                    level = StatusLevel::WARN;
                }
                showStatus(need_sig_text, level);
                break;
            }
            case PSBTRole::FINALIZER:
            case PSBTRole::EXTRACTOR: {
                showStatus(tr("Transaction is fully signed and ready for broadcast."), StatusLevel::INFO);
                break;
            }
            default: {
                showStatus(tr("Transaction status is unknown."), StatusLevel::ERR);
                break;
            }
        }
        */
    }
}
