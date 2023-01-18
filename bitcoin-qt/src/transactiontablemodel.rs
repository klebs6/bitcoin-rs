crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/transactiontablemodel.h]

/**
  | UI model for the transaction table of
  | a wallet.
  |
  */
#[Q_OBJECT]
pub struct TransactionTableModel {
    base:                           QAbstractTableModel,
    wallet_model:                   *mut WalletModel,
    handler_transaction_changed:    Box<dyn Handler>,
    handler_show_progress:          Box<dyn Handler>,
    columns:                        QStringList,
    priv_:                          *mut TransactionTablePriv,
    processing_queued_transactions: bool,
    platform_style:                 *const PlatformStyle,
}

pub mod transaction_table_model {

    use super::*;

    pub enum ColumnIndex {
        Status    = 0,
        Watchonly = 1,
        Date      = 2,
        Type      = 3,
        ToAddress = 4,
        Amount    = 5
    }

    /**
      | Roles to get specific information from
      | a transaction row.
      | 
      | These are independent of column.
      |
      */
    #[repr(i32)]
    pub enum RoleIndex {

        /**
          | Type of transaction
          |
          */
        TypeRole = USER_ROLE,

        /**
          | Date and time this transaction was created
          |
          */
        DateRole,

        /**
          | Watch-only boolean
          |
          */
        WatchonlyRole,

        /**
          | Watch-only icon
          |
          */
        WatchonlyDecorationRole,

        /**
          | Long description (HTML format)
          |
          */
        LongDescriptionRole,

        /**
          | Address of transaction
          |
          */
        AddressRole,

        /**
          | Label of address related to transaction
          |
          */
        LabelRole,

        /**
          | Net amount of transaction
          |
          */
        AmountRole,

        /**
          | Transaction hash
          |
          */
        TxHashRole,

        /**
          | Transaction data, hex-encoded
          |
          */
        TxHexRole,

        /**
          | Whole transaction as plain text
          |
          */
        TxPlainTextRole,

        /**
          | Is transaction confirmed?
          |
          */
        ConfirmedRole,

        /**
          | Formatted amount, without brackets
          | when unconfirmed
          |
          */
        FormattedAmountRole,

        /**
          | Transaction status (TransactionRecord::Status)
          |
          */
        StatusRole,

        /**
          | Unprocessed icon
          |
          */
        RawDecorationRole,
    }
}

impl Drop for TransactionTableModel {
    fn drop(&mut self) {
        todo!();
        /*
            unsubscribeFromCoreSignals();
        delete priv;
        */
    }
}

impl TransactionTableModel {
    
    pub fn processing_queued_transactions(&self) -> bool {
        
        todo!();
        /*
            return fProcessingQueuedTransactions;
        */
    }
    
    /**
      | Needed to update fProcessingQueuedTransactions
      | through a QueuedConnection
      |
      */
    #[Q_SLOT]
    pub fn set_processing_queued_transactions(&mut self, value: bool)  {
        
        todo!();
        /*
            fProcessingQueuedTransactions = value;
        */
    }

    pub fn new(
        platform_style: *const PlatformStyle,
        parent:         *mut WalletModel) -> Self {
    
        todo!();
        /*


            :
            QAbstractTableModel(parent),
            walletModel(parent),
            priv(new TransactionTablePriv(this)),
            fProcessingQueuedTransactions(false),
            platformStyle(_platformStyle)
        subscribeToCoreSignals();

        columns << QString() << QString() << tr("Date") << tr("Type") << tr("Label") << BitcoinUnits::getAmountColumnTitle(walletModel->getOptionsModel()->getDisplayUnit());
        priv->refreshWallet(walletModel->wallet());

        connect(walletModel->getOptionsModel(), &OptionsModel::displayUnitChanged, this, &TransactionTableModel::updateDisplayUnit);
        */
    }

    /**
      | Updates the column title to "Amount
      | (DisplayUnit)" and emits headerDataChanged()
      | signal for table headers to react.
      |
      */
    #[Q_SLOT]
    pub fn update_amount_column_title(&mut self)  {
        
        todo!();
        /*
            columns[Amount] = BitcoinUnits::getAmountColumnTitle(walletModel->getOptionsModel()->getDisplayUnit());
        Q_EMIT headerDataChanged(QtHorizontal,Amount,Amount);
        */
    }
    
    /**
      | New transaction, or transaction changed
      | status
      |
      */
    #[Q_SLOT]
    pub fn update_transaction(&mut self, 
        hash:             &String,
        status:           i32,
        show_transaction: bool)  {
        
        todo!();
        /*
            uint256 updated;
        updated.SetHex(hash.toStdString());

        priv->updateWallet(walletModel->wallet(), updated, status, showTransaction);
        */
    }
    
    #[Q_SLOT]
    pub fn update_confirmations(&mut self)  {
        
        todo!();
        /*
            // Blocks came in since last poll.
        // Invalidate status (number of confirmations) and (possibly) description
        //  for all rows. Qt is smart enough to only actually request the data for the
        //  visible rows.
        Q_EMIT dataChanged(index(0, Status), index(priv->size()-1, Status));
        Q_EMIT dataChanged(index(0, ToAddress), index(priv->size()-1, ToAddress));
        */
    }
    
    pub fn row_count(&self, parent: &QModelIndex) -> i32 {
        
        todo!();
        /*
            if (parent.isValid()) {
            return 0;
        }
        return priv->size();
        */
    }
    
    pub fn column_count(&self, parent: &QModelIndex) -> i32 {
        
        todo!();
        /*
            if (parent.isValid()) {
            return 0;
        }
        return columns.length();
        */
    }
    
    pub fn format_tx_status(&self, wtx: *const TransactionRecord) -> String {
        
        todo!();
        /*
            QString status;

        switch(wtx->status.status)
        {
        case TransactionStatus::OpenUntilBlock:
            status = tr("Open for %n more block(s)","",wtx->status.open_for);
            break;
        case TransactionStatus::OpenUntilDate:
            status = tr("Open until %1").arg(typename gui_util::dateTimeStr(wtx->status.open_for));
            break;
        case TransactionStatus::Unconfirmed:
            status = tr("Unconfirmed");
            break;
        case TransactionStatus::Abandoned:
            status = tr("Abandoned");
            break;
        case TransactionStatus::Confirming:
            status = tr("Confirming (%1 of %2 recommended confirmations)").arg(wtx->status.depth).arg(TransactionRecord::RecommendedNumConfirmations);
            break;
        case TransactionStatus::Confirmed:
            status = tr("Confirmed (%1 confirmations)").arg(wtx->status.depth);
            break;
        case TransactionStatus::Conflicted:
            status = tr("Conflicted");
            break;
        case TransactionStatus::Immature:
            status = tr("Immature (%1 confirmations, will be available after %2)").arg(wtx->status.depth).arg(wtx->status.depth + wtx->status.matures_in);
            break;
        case TransactionStatus::NotAccepted:
            status = tr("Generated but not accepted");
            break;
        }

        return status;
        */
    }
    
    pub fn format_tx_date(&self, wtx: *const TransactionRecord) -> String {
        
        todo!();
        /*
            if(wtx->time)
        {
            return typename gui_util::dateTimeStr(wtx->time);
        }
        return QString();
        */
    }

    /**
      | Look up address in address book, if found
      | return label (address) otherwise just
      | return (address)
      |
      */
    pub fn lookup_address(&self, 
        address: &String,
        tooltip: bool) -> String {
        
        todo!();
        /*
            QString label = walletModel->getAddressTableModel()->labelForAddress(QString::fromStdString(address));
        QString description;
        if(!label.isEmpty())
        {
            description += label;
        }
        if(label.isEmpty() || tooltip)
        {
            description += QString(" (") + QString::fromStdString(address) + QString(")");
        }
        return description;
        */
    }
    
    pub fn format_tx_type(&self, wtx: *const TransactionRecord) -> String {
        
        todo!();
        /*
            switch(wtx->type)
        {
        case TransactionRecord::RecvWithAddress:
            return tr("Received with");
        case TransactionRecord::RecvFromOther:
            return tr("Received from");
        case TransactionRecord::SendToAddress:
        case TransactionRecord::SendToOther:
            return tr("Sent to");
        case TransactionRecord::SendToSelf:
            return tr("Payment to yourself");
        case TransactionRecord::Generated:
            return tr("Mined");
        default:
            return QString();
        }
        */
    }
    
    pub fn tx_address_decoration(&self, wtx: *const TransactionRecord) -> QVariant {
        
        todo!();
        /*
            switch(wtx->type)
        {
        case TransactionRecord::Generated:
            return QIcon(":/icons/tx_mined");
        case TransactionRecord::RecvWithAddress:
        case TransactionRecord::RecvFromOther:
            return QIcon(":/icons/tx_input");
        case TransactionRecord::SendToAddress:
        case TransactionRecord::SendToOther:
            return QIcon(":/icons/tx_output");
        default:
            return QIcon(":/icons/tx_inout");
        }
        */
    }
    
    pub fn format_tx_to_address(&self, 
        wtx:     *const TransactionRecord,
        tooltip: bool) -> String {
        
        todo!();
        /*
            QString watchAddress;
        if (tooltip && wtx->involvesWatchAddress) {
            // Mark transactions involving watch-only addresses by adding " (watch-only)"
            watchAddress = QLatin1String(" (") + tr("watch-only") + QLatin1Char(')');
        }

        switch(wtx->type)
        {
        case TransactionRecord::RecvFromOther:
            return QString::fromStdString(wtx->address) + watchAddress;
        case TransactionRecord::RecvWithAddress:
        case TransactionRecord::SendToAddress:
        case TransactionRecord::Generated:
            return lookupAddress(wtx->address, tooltip) + watchAddress;
        case TransactionRecord::SendToOther:
            return QString::fromStdString(wtx->address) + watchAddress;
        case TransactionRecord::SendToSelf:
            return lookupAddress(wtx->address, tooltip) + watchAddress;
        default:
            return tr("(n/a)") + watchAddress;
        }
        */
    }
    
    pub fn address_color(&self, wtx: *const TransactionRecord) -> QVariant {
        
        todo!();
        /*
            // Show addresses without label in a less visible color
        switch(wtx->type)
        {
        case TransactionRecord::RecvWithAddress:
        case TransactionRecord::SendToAddress:
        case TransactionRecord::Generated:
            {
            QString label = walletModel->getAddressTableModel()->labelForAddress(QString::fromStdString(wtx->address));
            if(label.isEmpty())
                return COLOR_BAREADDRESS;
            } break;
        case TransactionRecord::SendToSelf:
            return COLOR_BAREADDRESS;
        default:
            break;
        }
        return QVariant();
        */
    }
    
    pub fn format_tx_amount(&self, 
        wtx:              *const TransactionRecord,
        show_unconfirmed: Option<bool>,
        separators:       Option<bitcoin_units::SeparatorStyle>) -> String {

        let show_unconfirmed: bool = show_unconfirmed.unwrap_or(true);
        let separators:       bitcoin_units::SeparatorStyle = separators.unwrap_or(bitcoin_units::SeparatorStyle::STANDARD);
        
        todo!();
        /*
            QString str = BitcoinUnits::format(walletModel->getOptionsModel()->getDisplayUnit(), wtx->credit + wtx->debit, false, separators);
        if(showUnconfirmed)
        {
            if(!wtx->status.countsForBalance)
            {
                str = QString("[") + str + QString("]");
            }
        }
        return QString(str);
        */
    }
    
    pub fn tx_status_decoration(&self, wtx: *const TransactionRecord) -> QVariant {
        
        todo!();
        /*
            switch(wtx->status.status)
        {
        case TransactionStatus::OpenUntilBlock:
        case TransactionStatus::OpenUntilDate:
            return COLOR_TX_STATUS_OPENUNTILDATE;
        case TransactionStatus::Unconfirmed:
            return QIcon(":/icons/transaction_0");
        case TransactionStatus::Abandoned:
            return QIcon(":/icons/transaction_abandoned");
        case TransactionStatus::Confirming:
            switch(wtx->status.depth)
            {
            case 1: return QIcon(":/icons/transaction_1");
            case 2: return QIcon(":/icons/transaction_2");
            case 3: return QIcon(":/icons/transaction_3");
            case 4: return QIcon(":/icons/transaction_4");
            default: return QIcon(":/icons/transaction_5");
            };
        case TransactionStatus::Confirmed:
            return QIcon(":/icons/transaction_confirmed");
        case TransactionStatus::Conflicted:
            return QIcon(":/icons/transaction_conflicted");
        case TransactionStatus::Immature: {
            int total = wtx->status.depth + wtx->status.matures_in;
            int part = (wtx->status.depth * 4 / total) + 1;
            return QIcon(QString(":/icons/transaction_%1").arg(part));
            }
        case TransactionStatus::NotAccepted:
            return QIcon(":/icons/transaction_0");
        default:
            return COLOR_BLACK;
        }
        */
    }
    
    pub fn tx_watchonly_decoration(&self, wtx: *const TransactionRecord) -> QVariant {
        
        todo!();
        /*
            if (wtx->involvesWatchAddress)
            return QIcon(":/icons/eye");
        else
            return QVariant();
        */
    }
    
    pub fn format_tooltip(&self, rec: *const TransactionRecord) -> String {
        
        todo!();
        /*
            QString tooltip = formatTxStatus(rec) + QString("\n") + formatTxType(rec);
        if(rec->type==TransactionRecord::RecvFromOther || rec->type==TransactionRecord::SendToOther ||
           rec->type==TransactionRecord::SendToAddress || rec->type==TransactionRecord::RecvWithAddress)
        {
            tooltip += QString(" ") + formatTxToAddress(rec, true);
        }
        return tooltip;
        */
    }
    
    pub fn data(&self, 
        index: &QModelIndex,
        role:  i32) -> QVariant {
        
        todo!();
        /*
            if(!index.isValid())
            return QVariant();
        TransactionRecord *rec = static_cast<TransactionRecord*>(index.internalPointer());

        const auto column = static_cast<ColumnIndex>(index.column());
        switch (role) {
        case RawDecorationRole:
            switch (column) {
            case Status:
                return txStatusDecoration(rec);
            case Watchonly:
                return txWatchonlyDecoration(rec);
            case Date: return {};
            case Type: return {};
            case ToAddress:
                return txAddressDecoration(rec);
            case Amount: return {};
            } // no default case, so the compiler can warn about missing cases
            assert(false);
        case QtDecorationRole:
        {
            QIcon icon = qvariant_cast<QIcon>(index.data(RawDecorationRole));
            return platformStyle->TextColorIcon(icon);
        }
        case QtDisplayRole:
            switch (column) {
            case Status: return {};
            case Watchonly: return {};
            case Date:
                return formatTxDate(rec);
            case Type:
                return formatTxType(rec);
            case ToAddress:
                return formatTxToAddress(rec, false);
            case Amount:
                return formatTxAmount(rec, true, BitcoinUnits::SeparatorStyle::ALWAYS);
            } // no default case, so the compiler can warn about missing cases
            assert(false);
        case QtEditRole:
            // Edit role is used for sorting, so return the unformatted values
            switch (column) {
            case Status:
                return QString::fromStdString(rec->status.sortKey);
            case Date:
                return rec->time;
            case Type:
                return formatTxType(rec);
            case Watchonly:
                return (rec->involvesWatchAddress ? 1 : 0);
            case ToAddress:
                return formatTxToAddress(rec, true);
            case Amount:
                return i64(rec->credit + rec->debit);
            } // no default case, so the compiler can warn about missing cases
            assert(false);
        case QtToolTipRole:
            return formatTooltip(rec);
        case QtTextAlignmentRole:
            return column_alignments[index.column()];
        case QtForegroundRole:
            // Use the "danger" color for abandoned transactions
            if(rec->status.status == TransactionStatus::Abandoned)
            {
                return COLOR_TX_STATUS_DANGER;
            }
            // Non-confirmed (but not immature) as transactions are grey
            if(!rec->status.countsForBalance && rec->status.status != TransactionStatus::Immature)
            {
                return COLOR_UNCONFIRMED;
            }
            if(index.column() == Amount && (rec->credit+rec->debit) < 0)
            {
                return COLOR_NEGATIVE;
            }
            if(index.column() == ToAddress)
            {
                return addressColor(rec);
            }
            break;
        case TypeRole:
            return rec->type;
        case DateRole:
            return QDateTime::fromSecsSinceEpoch(rec->time);
        case WatchonlyRole:
            return rec->involvesWatchAddress;
        case WatchonlyDecorationRole:
            return txWatchonlyDecoration(rec);
        case LongDescriptionRole:
            return priv->describe(walletModel->node(), walletModel->wallet(), rec, walletModel->getOptionsModel()->getDisplayUnit());
        case AddressRole:
            return QString::fromStdString(rec->address);
        case LabelRole:
            return walletModel->getAddressTableModel()->labelForAddress(QString::fromStdString(rec->address));
        case AmountRole:
            return i64(rec->credit + rec->debit);
        case TxHashRole:
            return rec->getTxHash();
        case TxHexRole:
            return priv->getTxHex(walletModel->wallet(), rec);
        case TxPlainTextRole:
            {
                QString details;
                QDateTime date = QDateTime::fromSecsSinceEpoch(rec->time);
                QString txLabel = walletModel->getAddressTableModel()->labelForAddress(QString::fromStdString(rec->address));

                details.append(date.toString("M/d/yy HH:mm"));
                details.append(" ");
                details.append(formatTxStatus(rec));
                details.append(". ");
                if(!formatTxType(rec).isEmpty()) {
                    details.append(formatTxType(rec));
                    details.append(" ");
                }
                if(!rec->address.empty()) {
                    if(txLabel.isEmpty())
                        details.append(tr("(no label)") + " ");
                    else {
                        details.append("(");
                        details.append(txLabel);
                        details.append(") ");
                    }
                    details.append(QString::fromStdString(rec->address));
                    details.append(" ");
                }
                details.append(formatTxAmount(rec, false, BitcoinUnits::SeparatorStyle::NEVER));
                return details;
            }
        case ConfirmedRole:
            return rec->status.status == TransactionStatus::Status::Confirming || rec->status.status == TransactionStatus::Status::Confirmed;
        case FormattedAmountRole:
            // Used for copy/export, so don't include separators
            return formatTxAmount(rec, false, BitcoinUnits::SeparatorStyle::NEVER);
        case StatusRole:
            return rec->status.status;
        }
        return QVariant();
        */
    }
    
    pub fn header_data(&self, 
        section:     i32,
        orientation: QtOrientation,
        role:        i32) -> QVariant {
        
        todo!();
        /*
            if(orientation == QtHorizontal)
        {
            if(role == QtDisplayRole)
            {
                return columns[section];
            }
            else if (role == QtTextAlignmentRole)
            {
                return column_alignments[section];
            } else if (role == QtToolTipRole)
            {
                switch(section)
                {
                case Status:
                    return tr("Transaction status. Hover over this field to show number of confirmations.");
                case Date:
                    return tr("Date and time that the transaction was received.");
                case Type:
                    return tr("Type of transaction.");
                case Watchonly:
                    return tr("Whether or not a watch-only address is involved in this transaction.");
                case ToAddress:
                    return tr("User-defined intent/purpose of the transaction.");
                case Amount:
                    return tr("Amount removed from or added to balance.");
                }
            }
        }
        return QVariant();
        */
    }
    
    pub fn index(&self, 
        row:    i32,
        column: i32,
        parent: Option<&QModelIndex>) -> QModelIndex {

        let parent: &QModelIndex = unsafe { parent.unwrap_or(&QModelIndex::new()) };
        
        todo!();
        /*
            Q_UNUSED(parent);
        TransactionRecord* data = priv->index(walletModel->wallet(), walletModel->getLastBlockProcessed(), row);
        if(data)
        {
            return createIndex(row, column, data);
        }
        return QModelIndex();
        */
    }
    
    #[Q_SLOT]
    pub fn update_display_unit(&mut self)  {
        
        todo!();
        /*
            // emit dataChanged to update Amount column with the current unit
        updateAmountColumnTitle();
        Q_EMIT dataChanged(index(0, Amount), index(priv->size()-1, Amount));
        */
    }
    
    pub fn subscribe_to_core_signals(&mut self)  {
        
        todo!();
        /*
            // Connect signals to wallet
        m_handler_transaction_changed = walletModel->wallet().handleTransactionChanged(std::bind(&TransactionTablePriv::NotifyTransactionChanged, priv, std::placeholders::_1, std::placeholders::_2));
        m_handler_show_progress = walletModel->wallet().handleShowProgress([this](const std::string&, int progress) {
            priv->m_loading = progress < 100;
            priv->DispatchNotifications();
        });
        */
    }
    
    pub fn unsubscribe_from_core_signals(&mut self)  {
        
        todo!();
        /*
            // Disconnect signals from wallet
        m_handler_transaction_changed->disconnect();
        m_handler_show_progress->disconnect();
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/transactiontablemodel.cpp]

/**
  | Amount column is right-aligned it contains
  | numbers
  |
  */
lazy_static!{
    /*
    static int column_alignments[] = {
            QtAlignLeft|QtAlignVCenter, /* status */
            QtAlignLeft|QtAlignVCenter, /* watchonly */
            QtAlignLeft|QtAlignVCenter, /* date */
            QtAlignLeft|QtAlignVCenter, /* type */
            QtAlignLeft|QtAlignVCenter, /* address */
            QtAlignRight|QtAlignVCenter /* amount */
        };
    */
}

/**
  | Comparison operator for sort/binary
  | search of model tx list
  |
  */
pub struct TxLessThan {

}

impl TxLessThan {

    pub fn invoke<T,S>(&self, 
        a: &T,
        b: &S) -> bool {
        
        todo!();
        /*
        pub fn invoke(&self, 
            a: &TransactionRecord,
            b: &TransactionRecord) -> bool {
            
            todo!();
            /*
                return a.hash < b.hash;
            */
        }
        
        pub fn invoke(&self, 
            a: &TransactionRecord,
            b: &u256) -> bool {
            
            todo!();
            /*
                return a.hash < b;
            */
        }
        
        pub fn invoke(&self, 
            a: &u256,
            b: &TransactionRecord) -> bool {
            
            todo!();
            /*
                return a < b.hash;
            */
        }
        */
    }
}

/**
  | queue notifications to show a non freezing
  | progress dialog e.g. for rescan
  |
  */
#[derive(Default)]
pub struct TransactionNotification {
    hash:             u256,
    status:           ChangeType,
    show_transaction: bool,
}

impl TransactionNotification {

    pub fn new(
        hash:             u256,
        status:           ChangeType,
        show_transaction: bool) -> Self {
    
        todo!();
        /*
        : hash(_hash),
        : status(_status),
        : show_transaction(_showTransaction),

        
        */
    }
    
    pub fn invoke(&mut self, ttm: *mut QObject)  {
        
        todo!();
        /*
            QString strHash = QString::fromStdString(hash.GetHex());
            qDebug() << "NotifyTransactionChanged: " + strHash + " status= " + QString::number(status);
            bool invoked = QMetaObject::invokeMethod(ttm, "updateTransaction", QtQueuedConnection,
                                      Q_ARG(QString, strHash),
                                      Q_ARG(int, status),
                                      Q_ARG(bool, showTransaction));
            assert(invoked);
        */
    }
}

/**
  | Private implementation
  |
  */
pub struct TransactionTablePriv {

    parent:              *mut TransactionTableModel,

    /**
      | Local cache of wallet.
      | 
      | As it is in the same order as the CWallet,
      | by definition this is sorted by sha256.
      |
      */
    cached_wallet:       QList<TransactionRecord>,


    /**
      | True when model finishes loading all
      | wallet transactions on start
      |
      */
    loaded:              bool, // default = false

    /**
      | True when transactions are being notified,
      | for instance when scanning
      |
      */
    loading:             bool, // default = false

    queue_notifications: Vec<TransactionNotification>,
}

impl TransactionTablePriv {

    pub fn new(parent: *mut TransactionTableModel) -> Self {
    
        todo!();
        /*
        : parent(_parent),

        
        */
    }
    
    /**
      | Query entire wallet anew from core.
      |
      */
    pub fn refresh_wallet(&mut self, wallet: Rc<RefCell<dyn WalletInterface>>)  {
        
        todo!();
        /*
            assert(!m_loaded);
            {
                for (const auto& wtx : wallet.getWalletTxs()) {
                    if (TransactionRecord::showTransaction()) {
                        cachedWallet.append(TransactionRecord::decomposeTransaction(wtx));
                    }
                }
            }
            m_loaded = true;
            DispatchNotifications();
        */
    }

    /**
      | Update our model of the wallet incrementally,
      | to synchronize our model of the wallet
      | with that of the core.
      | 
      | Call with transaction that was added,
      | removed or changed.
      |
      */
    pub fn update_wallet(&mut self, 
        wallet:           Rc<RefCell<dyn WalletInterface>>,
        hash:             &u256,
        status:           i32,
        show_transaction: bool)  {
        
        todo!();
        /*
            qDebug() << "TransactionTablePriv::updateWallet: " + QString::fromStdString(hash.ToString()) + " " + QString::number(status);

            // Find bounds of this transaction in model
            QList<TransactionRecord>::iterator lower = std::lower_bound(
                cachedWallet.begin(), cachedWallet.end(), hash, TxLessThan());
            QList<TransactionRecord>::iterator upper = std::upper_bound(
                cachedWallet.begin(), cachedWallet.end(), hash, TxLessThan());
            int lowerIndex = (lower - cachedWallet.begin());
            int upperIndex = (upper - cachedWallet.begin());
            bool inModel = (lower != upper);

            if(status == CT_UPDATED)
            {
                if(showTransaction && !inModel)
                    status = CT_NEW; /* Not in model, but want to show, treat as new */
                if(!showTransaction && inModel)
                    status = CT_DELETED; /* In model, but want to hide, treat as deleted */
            }

            qDebug() << "    inModel=" + QString::number(inModel) +
                        " Index=" + QString::number(lowerIndex) + "-" + QString::number(upperIndex) +
                        " showTransaction=" + QString::number(showTransaction) + " derivedStatus=" + QString::number(status);

            switch(status)
            {
            case CT_NEW:
                if(inModel)
                {
                    qWarning() << "TransactionTablePriv::updateWallet: Warning: Got CT_NEW, but transaction is already in model";
                    break;
                }
                if(showTransaction)
                {
                    // Find transaction in wallet
                    typename interfaces::WalletTx wtx = wallet.getWalletTx(hash);
                    if(!wtx.tx)
                    {
                        qWarning() << "TransactionTablePriv::updateWallet: Warning: Got CT_NEW, but transaction is not in wallet";
                        break;
                    }
                    // Added -- insert at the right position
                    QList<TransactionRecord> toInsert =
                            TransactionRecord::decomposeTransaction(wtx);
                    if(!toInsert.isEmpty()) /* only if something to insert */
                    {
                        parent->beginInsertRows(QModelIndex(), lowerIndex, lowerIndex+toInsert.size()-1);
                        int insert_idx = lowerIndex;
                        for (const TransactionRecord &rec : toInsert)
                        {
                            cachedWallet.insert(insert_idx, rec);
                            insert_idx += 1;
                        }
                        parent->endInsertRows();
                    }
                }
                break;
            case CT_DELETED:
                if(!inModel)
                {
                    qWarning() << "TransactionTablePriv::updateWallet: Warning: Got CT_DELETED, but transaction is not in model";
                    break;
                }
                // Removed -- remove entire transaction from table
                parent->beginRemoveRows(QModelIndex(), lowerIndex, upperIndex-1);
                cachedWallet.erase(lower, upper);
                parent->endRemoveRows();
                break;
            case CT_UPDATED:
                // Miscellaneous updates -- nothing to do, status update will take care of this, and is only computed for
                // visible transactions.
                for (int i = lowerIndex; i < upperIndex; i++) {
                    TransactionRecord *rec = &cachedWallet[i];
                    rec->status.needsUpdate = true;
                }
                break;
            }
        */
    }
    
    pub fn size(&mut self) -> i32 {
        
        todo!();
        /*
            return cachedWallet.size();
        */
    }
    
    pub fn index(&mut self, 
        wallet:         Rc<RefCell<dyn WalletInterface>>,
        cur_block_hash: &u256,
        idx:            i32) -> *mut TransactionRecord {
        
        todo!();
        /*
            if (idx >= 0 && idx < cachedWallet.size()) {
                TransactionRecord *rec = &cachedWallet[idx];

                // If a status update is needed (blocks came in since last check),
                // try to update the status of this transaction from the wallet.
                // Otherwise, simply re-use the cached status.
                typename interfaces::WalletTxStatus wtx;
                int numBlocks;
                int64_t block_time;
                if (!cur_block_hash.IsNull() && rec->statusUpdateNeeded(cur_block_hash) && wallet.tryGetTxStatus(rec->hash, wtx, numBlocks, block_time)) {
                    rec->updateStatus(wtx, cur_block_hash, numBlocks, block_time);
                }
                return rec;
            }
            return nullptr;
        */
    }
    
    pub fn describe(&mut self, 
        node:   Rc<RefCell<dyn NodeInterface>>,
        wallet: Rc<RefCell<dyn WalletInterface>>,
        rec:    *mut TransactionRecord,
        unit:   i32) -> String {
        
        todo!();
        /*
            return TransactionDesc::toHTML(node, wallet, rec, unit);
        */
    }
    
    pub fn get_tx_hex(&mut self, 
        wallet: Rc<RefCell<dyn WalletInterface>>,
        rec:    *mut TransactionRecord) -> String {
        
        todo!();
        /*
            auto tx = wallet.getTx(rec->hash);
            if (tx) {
                std::string strHex = EncodeHexTx(*tx);
                return QString::fromStdString(strHex);
            }
            return QString();
        */
    }
    
    pub fn notify_transaction_changed(&mut self, 
        hash:   &u256,
        status: ChangeType)  {
        
        todo!();
        /*
            // Find transaction in wallet
        // Determine whether to show transaction or not (determine this here so that no relocking is needed in GUI thread)
        bool showTransaction = TransactionRecord::showTransaction();

        TransactionNotification notification(hash, status, showTransaction);

        if (!m_loaded || m_loading)
        {
            vQueueNotifications.push_back(notification);
            return;
        }
        notification.invoke(parent);
        */
    }
    
    pub fn dispatch_notifications(&mut self)  {
        
        todo!();
        /*
            if (!m_loaded || m_loading) return;

        if (vQueueNotifications.size() > 10) { // prevent balloon spam, show maximum 10 balloons
            bool invoked = QMetaObject::invokeMethod(parent, "setProcessingQueuedTransactions", QtQueuedConnection, Q_ARG(bool, true));
            assert(invoked);
        }
        for (unsigned int i = 0; i < vQueueNotifications.size(); ++i)
        {
            if (vQueueNotifications.size() - i <= 10) {
                bool invoked = QMetaObject::invokeMethod(parent, "setProcessingQueuedTransactions", QtQueuedConnection, Q_ARG(bool, false));
                assert(invoked);
            }

            vQueueNotifications[i].invoke(parent);
        }
        vQueueNotifications.clear();
        */
    }
}
