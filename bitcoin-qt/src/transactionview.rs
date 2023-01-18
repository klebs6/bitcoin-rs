crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/transactionview.h]

/**
  | Widget showing the transaction list
  | for a wallet, including a filter row.
  | 
  | Using the filter row, the user can view
  | or export a subset of the transactions.
  |
  */
#[Q_OBJECT]
pub struct TransactionView {
    base:                    QWidget,
    model:                   *mut WalletModel, // default = { nullptr }
    transaction_proxy_model: *mut TransactionFilterProxy, // default = { nullptr }
    transaction_view:        *mut QTableView, // default = { nullptr }
    date_widget:             *mut QComboBox,
    type_widget:             *mut QComboBox,
    watch_only_widget:       *mut QComboBox,
    search_widget:           *mut QLineEdit,
    amount_widget:           *mut QLineEdit,
    context_menu:            *mut QMenu,
    date_range_widget:       *mut QFrame,
    date_from:               *mut QDateTimeEdit,
    date_to:                 *mut QDateTimeEdit,
    abandon_action:          *mut QAction, // default = { nullptr }
    bump_fee_action:         *mut QAction, // default = { nullptr }
    copy_address_action:     *mut QAction, // default = { nullptr }
    copy_label_action:       *mut QAction, // default = { nullptr }
    platform_style:          *const PlatformStyle,
}

pub mod transaction_view {
    use super::*;

    /**
      | Date ranges for filter
      |
      */
    pub enum DateEnum
    {
        All,
        Today,
        ThisWeek,
        ThisMonth,
        LastMonth,
        ThisYear,
        Range
    }

    bitflags!{
        pub struct ColumnWidths: u32 {
            const STATUS_COLUMN_WIDTH         = 30;
            const WATCHONLY_COLUMN_WIDTH      = 23;
            const DATE_COLUMN_WIDTH           = 120;
            const TYPE_COLUMN_WIDTH           = 113;
            const AMOUNT_MINIMUM_COLUMN_WIDTH = 120;
            const MINIMUM_COLUMN_WIDTH        = 23;
        }
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/transactionview.cpp]
impl Drop for TransactionView {
    fn drop(&mut self) {
        todo!();
        /*
            QSettings settings;
        settings.setValue("TransactionViewHeaderState", transactionView->horizontalHeader()->saveState());
        */
    }
}

impl TransactionView {

    #[Q_SIGNAL]
    pub fn double_clicked(&mut self, _0: &QModelIndex)  {
        
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
        title:   &String,
        message: &String,
        style:   u32)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn bumped_fee(&mut self, txid: &u256)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn new(
        platform_style: *const PlatformStyle,
        parent:         *mut QWidget) -> Self {
    
        todo!();
        /*


            : QWidget(parent), m_platform_style{platformStyle}

        // Build filter row
        setContentsMargins(0,0,0,0);

        QHBoxLayout *hlayout = new QHBoxLayout();
        hlayout->setContentsMargins(0,0,0,0);

        if (platformStyle->getUseExtraSpacing()) {
            hlayout->setSpacing(5);
            hlayout->addSpacing(26);
        } else {
            hlayout->setSpacing(0);
            hlayout->addSpacing(23);
        }

        watchOnlyWidget = new QComboBox(this);
        watchOnlyWidget->setFixedWidth(24);
        watchOnlyWidget->addItem("", TransactionFilterProxy::WatchOnlyFilter_All);
        watchOnlyWidget->addItem(platformStyle->SingleColorIcon(":/icons/eye_plus"), "", TransactionFilterProxy::WatchOnlyFilter_Yes);
        watchOnlyWidget->addItem(platformStyle->SingleColorIcon(":/icons/eye_minus"), "", TransactionFilterProxy::WatchOnlyFilter_No);
        hlayout->addWidget(watchOnlyWidget);

        dateWidget = new QComboBox(this);
        if (platformStyle->getUseExtraSpacing()) {
            dateWidget->setFixedWidth(121);
        } else {
            dateWidget->setFixedWidth(120);
        }
        dateWidget->addItem(tr("All"), All);
        dateWidget->addItem(tr("Today"), Today);
        dateWidget->addItem(tr("This week"), ThisWeek);
        dateWidget->addItem(tr("This month"), ThisMonth);
        dateWidget->addItem(tr("Last month"), LastMonth);
        dateWidget->addItem(tr("This year"), ThisYear);
        dateWidget->addItem(tr("Range…"), Range);
        hlayout->addWidget(dateWidget);

        typeWidget = new QComboBox(this);
        if (platformStyle->getUseExtraSpacing()) {
            typeWidget->setFixedWidth(121);
        } else {
            typeWidget->setFixedWidth(120);
        }

        typeWidget->addItem(tr("All"), TransactionFilterProxy::ALL_TYPES);
        typeWidget->addItem(tr("Received with"), TransactionFilterProxy::TYPE(TransactionRecord::RecvWithAddress) |
                                            TransactionFilterProxy::TYPE(TransactionRecord::RecvFromOther));
        typeWidget->addItem(tr("Sent to"), TransactionFilterProxy::TYPE(TransactionRecord::SendToAddress) |
                                      TransactionFilterProxy::TYPE(TransactionRecord::SendToOther));
        typeWidget->addItem(tr("To yourself"), TransactionFilterProxy::TYPE(TransactionRecord::SendToSelf));
        typeWidget->addItem(tr("Mined"), TransactionFilterProxy::TYPE(TransactionRecord::Generated));
        typeWidget->addItem(tr("Other"), TransactionFilterProxy::TYPE(TransactionRecord::Other));

        hlayout->addWidget(typeWidget);

        search_widget = new QLineEdit(this);
        search_widget->setPlaceholderText(tr("Enter address, transaction id, or label to search"));
        hlayout->addWidget(search_widget);

        amountWidget = new QLineEdit(this);
        amountWidget->setPlaceholderText(tr("Min amount"));
        if (platformStyle->getUseExtraSpacing()) {
            amountWidget->setFixedWidth(97);
        } else {
            amountWidget->setFixedWidth(100);
        }
        QDoubleValidator *amountValidator = new QDoubleValidator(0, 1e20, 8, this);
        QLocale amountLocale(QLocale::C);
        amountLocale.setNumberOptions(QLocale::RejectGroupSeparator);
        amountValidator->setLocale(amountLocale);
        amountWidget->setValidator(amountValidator);
        hlayout->addWidget(amountWidget);

        // Delay before filtering transactions in ms
        static const int input_filter_delay = 200;

        QTimer* amount_typing_delay = new QTimer(this);
        amount_typing_delay->setSingleShot(true);
        amount_typing_delay->setInterval(input_filter_delay);

        QTimer* prefix_typing_delay = new QTimer(this);
        prefix_typing_delay->setSingleShot(true);
        prefix_typing_delay->setInterval(input_filter_delay);

        QVBoxLayout *vlayout = new QVBoxLayout(this);
        vlayout->setContentsMargins(0,0,0,0);
        vlayout->setSpacing(0);

        transactionView = new QTableView(this);
        transactionView->setObjectName("transactionView");
        vlayout->addLayout(hlayout);
        vlayout->addWidget(createDateRangeWidget());
        vlayout->addWidget(transactionView);
        vlayout->setSpacing(0);
        int width = transactionView->verticalScrollBar()->sizeHint().width();
        // Cover scroll bar width with spacing
        if (platformStyle->getUseExtraSpacing()) {
            hlayout->addSpacing(width+2);
        } else {
            hlayout->addSpacing(width);
        }
        transactionView->setVerticalScrollBarPolicy(QtScrollBarAlwaysOn);
        transactionView->setTabKeyNavigation(false);
        transactionView->setContextMenuPolicy(QtCustomContextMenu);
        transactionView->installEventFilter(this);
        transactionView->setAlternatingRowColors(true);
        transactionView->setSelectionBehavior(QAbstractItemView::SelectRows);
        transactionView->setSelectionMode(QAbstractItemView::ExtendedSelection);
        transactionView->setSortingEnabled(true);
        transactionView->verticalHeader()->hide();

        QSettings settings;
        if (!transactionView->horizontalHeader()->restoreState(settings.value("TransactionViewHeaderState").toByteArray())) {
            transactionView->setColumnWidth(TransactionTableModel::Status, STATUS_COLUMN_WIDTH);
            transactionView->setColumnWidth(TransactionTableModel::Watchonly, WATCHONLY_COLUMN_WIDTH);
            transactionView->setColumnWidth(TransactionTableModel::Date, DATE_COLUMN_WIDTH);
            transactionView->setColumnWidth(TransactionTableModel::Type, TYPE_COLUMN_WIDTH);
            transactionView->setColumnWidth(TransactionTableModel::Amount, AMOUNT_MINIMUM_COLUMN_WIDTH);
            transactionView->horizontalHeader()->setMinimumSectionSize(MINIMUM_COLUMN_WIDTH);
            transactionView->horizontalHeader()->setStretchLastSection(true);
        }

        contextMenu = new QMenu(this);
        contextMenu->setObjectName("contextMenu");
        copyAddressAction = contextMenu->addAction(tr("&Copy address"), this, &TransactionView::copyAddress);
        copyLabelAction = contextMenu->addAction(tr("Copy &label"), this, &TransactionView::copyLabel);
        contextMenu->addAction(tr("Copy &amount"), this, &TransactionView::copyAmount);
        contextMenu->addAction(tr("Copy transaction &ID"), this, &TransactionView::copyTxID);
        contextMenu->addAction(tr("Copy &raw transaction"), this, &TransactionView::copyTxHex);
        contextMenu->addAction(tr("Copy full transaction &details"), this, &TransactionView::copyTxPlainText);
        contextMenu->addAction(tr("&Show transaction details"), this, &TransactionView::showDetails);
        contextMenu->addSeparator();
        bumpFeeAction = contextMenu->addAction(tr("Increase transaction &fee"));
        typename gui_util::ExceptionSafeConnect(bumpFeeAction, &QAction::triggered, this, &TransactionView::bumpFee);
        bumpFeeAction->setObjectName("bumpFeeAction");
        abandonAction = contextMenu->addAction(tr("A&bandon transaction"), this, &TransactionView::abandonTx);
        contextMenu->addAction(tr("&Edit address label"), this, &TransactionView::editLabel);

        connect(dateWidget, qOverload<int>(&QComboBox::activated), this, &TransactionView::chooseDate);
        connect(typeWidget, qOverload<int>(&QComboBox::activated), this, &TransactionView::chooseType);
        connect(watchOnlyWidget, qOverload<int>(&QComboBox::activated), this, &TransactionView::chooseWatchonly);
        connect(amountWidget, &QLineEdit::textChanged, amount_typing_delay, qOverload<>(&QTimer::start));
        connect(amount_typing_delay, &QTimer::timeout, this, &TransactionView::changedAmount);
        connect(search_widget, &QLineEdit::textChanged, prefix_typing_delay, qOverload<>(&QTimer::start));
        connect(prefix_typing_delay, &QTimer::timeout, this, &TransactionView::changedSearch);

        connect(transactionView, &QTableView::doubleClicked, this, &TransactionView::doubleClicked);
        connect(transactionView, &QTableView::customContextMenuRequested, this, &TransactionView::contextualMenu);

        // Double-clicking on a transaction on the transaction history page shows details
        connect(this, &TransactionView::doubleClicked, this, &TransactionView::showDetails);
        // Highlight transaction after fee bump
        connect(this, &TransactionView::bumpedFee, [this](const uint256& txid) {
          focusTransaction(txid);
        });
        */
    }
    
    pub fn set_model(&mut self, model: *mut WalletModel)  {
        
        todo!();
        /*
            this->model = _model;
        if(_model)
        {
            transactionProxyModel = new TransactionFilterProxy(this);
            transactionProxyModel->setSourceModel(_model->getTransactionTableModel());
            transactionProxyModel->setDynamicSortFilter(true);
            transactionProxyModel->setSortCaseSensitivity(QtCaseInsensitive);
            transactionProxyModel->setFilterCaseSensitivity(QtCaseInsensitive);
            transactionProxyModel->setSortRole(QtEditRole);
            transactionView->setModel(transactionProxyModel);
            transactionView->sortByColumn(TransactionTableModel::Date, QtDescendingOrder);

            if (_model->getOptionsModel())
            {
                // Add third party transaction URLs to context menu
                QStringList listUrls = typename gui_util::SplitSkipEmptyParts(_model->getOptionsModel()->getThirdPartyTxUrls(), "|");
                bool actions_created = false;
                for (int i = 0; i < listUrls.size(); ++i)
                {
                    QString url = listUrls[i].trimmed();
                    QString host = QUrl(url, QUrl::StrictMode).host();
                    if (!host.isEmpty())
                    {
                        if (!actions_created) {
                            contextMenu->addSeparator();
                            actions_created = true;
                        }
                        /*: Transactions table context menu action to show the
                            selected transaction in a third-party block explorer.
                            %1 is a stand-in argument for the URL of the explorer. */
                        contextMenu->addAction(tr("Show in %1").arg(host), [this, url] { openThirdPartyTxUrl(url); });
                    }
                }
            }

            // show/hide column Watch-only
            updateWatchOnlyColumn(_model->wallet().haveWatchOnly());

            // Watch-only signal
            connect(_model, &WalletModel::notifyWatchonlyChanged, this, &TransactionView::updateWatchOnlyColumn);
        }
        */
    }
    
    pub fn change_event(&mut self, e: *mut QEvent)  {
        
        todo!();
        /*
            if (e->type() == QEvent::PaletteChange) {
            watchOnlyWidget->setItemIcon(
                TransactionFilterProxy::WatchOnlyFilter_Yes,
                m_platform_style->SingleColorIcon(QStringLiteral(":/icons/eye_plus")));
            watchOnlyWidget->setItemIcon(
                TransactionFilterProxy::WatchOnlyFilter_No,
                m_platform_style->SingleColorIcon(QStringLiteral(":/icons/eye_minus")));
        }

        QWidget::changeEvent(e);
        */
    }
    
    #[Q_SLOT]
    pub fn choose_date(&mut self, idx: i32)  {
        
        todo!();
        /*
            if (!transactionProxyModel) return;
        QDate current = QDate::currentDate();
        dateRangeWidget->setVisible(false);
        switch(dateWidget->itemData(idx).toInt())
        {
        case All:
            transactionProxyModel->setDateRange(
                    std::nullopt,
                    std::nullopt);
            break;
        case Today:
            transactionProxyModel->setDateRange(
                    typename gui_util::StartOfDay(current),
                    std::nullopt);
            break;
        case ThisWeek: {
            // Find last Monday
            QDate startOfWeek = current.addDays(-(current.dayOfWeek()-1));
            transactionProxyModel->setDateRange(
                    typename gui_util::StartOfDay(startOfWeek),
                    std::nullopt);

            } break;
        case ThisMonth:
            transactionProxyModel->setDateRange(
                    typename gui_util::StartOfDay(QDate(current.year(), current.month(), 1)),
                    std::nullopt);
            break;
        case LastMonth:
            transactionProxyModel->setDateRange(
                    typename gui_util::StartOfDay(QDate(current.year(), current.month(), 1).addMonths(-1)),
                    typename gui_util::StartOfDay(QDate(current.year(), current.month(), 1)));
            break;
        case ThisYear:
            transactionProxyModel->setDateRange(
                    typename gui_util::StartOfDay(QDate(current.year(), 1, 1)),
                    std::nullopt);
            break;
        case Range:
            dateRangeWidget->setVisible(true);
            dateRangeChanged();
            break;
        }
        */
    }
    
    #[Q_SLOT]
    pub fn choose_type(&mut self, idx: i32)  {
        
        todo!();
        /*
            if(!transactionProxyModel)
            return;
        transactionProxyModel->setTypeFilter(
            typeWidget->itemData(idx).toInt());
        */
    }
    
    #[Q_SLOT]
    pub fn choose_watchonly(&mut self, idx: i32)  {
        
        todo!();
        /*
            if(!transactionProxyModel)
            return;
        transactionProxyModel->setWatchOnlyFilter(
            static_cast<TransactionFilterProxy::WatchOnlyFilter>(watchOnlyWidget->itemData(idx).toInt()));
        */
    }
    
    #[Q_SLOT]
    pub fn changed_search(&mut self)  {
        
        todo!();
        /*
            if(!transactionProxyModel)
            return;
        transactionProxyModel->setSearchString(search_widget->text());
        */
    }
    
    #[Q_SLOT]
    pub fn changed_amount(&mut self)  {
        
        todo!();
        /*
            if(!transactionProxyModel)
            return;
        CAmount amount_parsed = 0;
        if (BitcoinUnits::parse(model->getOptionsModel()->getDisplayUnit(), amountWidget->text(), &amount_parsed)) {
            transactionProxyModel->setMinAmount(amount_parsed);
        }
        else
        {
            transactionProxyModel->setMinAmount(0);
        }
        */
    }
    
    #[Q_SLOT]
    pub fn export_clicked(&mut self)  {
        
        todo!();
        /*
            if (!model || !model->getOptionsModel()) {
            return;
        }

        // CSV is currently the only supported format
        QString filename = typename gui_util::getSaveFileName(this,
            tr("Export Transaction History"), QString(),
            /*: Expanded name of the CSV file format.
                See: https://en.wikipedia.org/wiki/Comma-separated_values. */
            tr("Comma separated file") + QLatin1String(" (*.csv)"), nullptr);

        if (filename.isNull())
            return;

        CSVModelWriter writer(filename);

        // name, column, role
        writer.setModel(transactionProxyModel);
        writer.addColumn(tr("Confirmed"), 0, TransactionTableModel::ConfirmedRole);
        if (model->wallet().haveWatchOnly())
            writer.addColumn(tr("Watch-only"), TransactionTableModel::Watchonly);
        writer.addColumn(tr("Date"), 0, TransactionTableModel::DateRole);
        writer.addColumn(tr("Type"), TransactionTableModel::Type, QtEditRole);
        writer.addColumn(tr("Label"), 0, TransactionTableModel::LabelRole);
        writer.addColumn(tr("Address"), 0, TransactionTableModel::AddressRole);
        writer.addColumn(BitcoinUnits::getAmountColumnTitle(model->getOptionsModel()->getDisplayUnit()), 0, TransactionTableModel::FormattedAmountRole);
        writer.addColumn(tr("ID"), 0, TransactionTableModel::TxHashRole);

        if(!writer.write()) {
            Q_EMIT message(tr("Exporting Failed"), tr("There was an error trying to save the transaction history to %1.").arg(filename),
                CClientUIInterface::MSG_ERROR);
        }
        else {
            Q_EMIT message(tr("Exporting Successful"), tr("The transaction history was successfully saved to %1.").arg(filename),
                CClientUIInterface::MSG_INFORMATION);
        }
        */
    }
    
    #[Q_SLOT]
    pub fn contextual_menu(&mut self, point: &QPoint)  {
        
        todo!();
        /*
            QModelIndex index = transactionView->indexAt(point);
        QModelIndexList selection = transactionView->selectionModel()->selectedRows(0);
        if (selection.empty())
            return;

        // check if transaction can be abandoned, disable context menu action in case it doesn't
        uint256 hash;
        hash.SetHex(selection.at(0).data(TransactionTableModel::TxHashRole).toString().toStdString());
        abandonAction->setEnabled(model->wallet().transactionCanBeAbandoned(hash));
        bumpFeeAction->setEnabled(model->wallet().transactionCanBeBumped(hash));
        copyAddressAction->setEnabled(typename gui_util::hasEntryData(transactionView, 0, TransactionTableModel::AddressRole));
        copyLabelAction->setEnabled(typename gui_util::hasEntryData(transactionView, 0, TransactionTableModel::LabelRole));

        if (index.isValid()) {
            typename gui_util::PopupMenu(contextMenu, transactionView->viewport()->mapToGlobal(point));
        }
        */
    }
    
    #[Q_SLOT]
    pub fn abandon_tx(&mut self)  {
        
        todo!();
        /*
            if(!transactionView || !transactionView->selectionModel())
            return;
        QModelIndexList selection = transactionView->selectionModel()->selectedRows(0);

        // get the hash from the TxHashRole (QVariant / QString)
        uint256 hash;
        QString hashQStr = selection.at(0).data(TransactionTableModel::TxHashRole).toString();
        hash.SetHex(hashQStr.toStdString());

        // Abandon the wallet transaction over the walletModel
        model->wallet().abandonTransaction(hash);

        // Update the table
        model->getTransactionTableModel()->updateTransaction(hashQStr, CT_UPDATED, false);
        */
    }
    
    #[Q_SLOT]
    pub fn bump_fee(&mut self, checked: bool)  {
        
        todo!();
        /*
            if(!transactionView || !transactionView->selectionModel())
            return;
        QModelIndexList selection = transactionView->selectionModel()->selectedRows(0);

        // get the hash from the TxHashRole (QVariant / QString)
        uint256 hash;
        QString hashQStr = selection.at(0).data(TransactionTableModel::TxHashRole).toString();
        hash.SetHex(hashQStr.toStdString());

        // Bump tx fee over the walletModel
        uint256 newHash;
        if (model->bumpFee(hash, newHash)) {
            // Update the table
            transactionView->selectionModel()->clearSelection();
            model->getTransactionTableModel()->updateTransaction(hashQStr, CT_UPDATED, true);

            qApp->processEvents();
            Q_EMIT bumpedFee(newHash);
        }
        */
    }
    
    #[Q_SLOT]
    pub fn copy_address(&mut self)  {
        
        todo!();
        /*
            typename gui_util::copyEntryData(transactionView, 0, TransactionTableModel::AddressRole);
        */
    }
    
    #[Q_SLOT]
    pub fn copy_label(&mut self)  {
        
        todo!();
        /*
            typename gui_util::copyEntryData(transactionView, 0, TransactionTableModel::LabelRole);
        */
    }
    
    #[Q_SLOT]
    pub fn copy_amount(&mut self)  {
        
        todo!();
        /*
            typename gui_util::copyEntryData(transactionView, 0, TransactionTableModel::FormattedAmountRole);
        */
    }
    
    #[Q_SLOT]
    pub fn copy_txid(&mut self)  {
        
        todo!();
        /*
            typename gui_util::copyEntryData(transactionView, 0, TransactionTableModel::TxHashRole);
        */
    }
    
    #[Q_SLOT]
    pub fn copy_tx_hex(&mut self)  {
        
        todo!();
        /*
            typename gui_util::copyEntryData(transactionView, 0, TransactionTableModel::TxHexRole);
        */
    }
    
    #[Q_SLOT]
    pub fn copy_tx_plain_text(&mut self)  {
        
        todo!();
        /*
            typename gui_util::copyEntryData(transactionView, 0, TransactionTableModel::TxPlainTextRole);
        */
    }
    
    #[Q_SLOT]
    pub fn edit_label(&mut self)  {
        
        todo!();
        /*
            if(!transactionView->selectionModel() ||!model)
            return;
        QModelIndexList selection = transactionView->selectionModel()->selectedRows();
        if(!selection.isEmpty())
        {
            AddressTableModel *addressBook = model->getAddressTableModel();
            if(!addressBook)
                return;
            QString address = selection.at(0).data(TransactionTableModel::AddressRole).toString();
            if(address.isEmpty())
            {
                // If this transaction has no associated address, exit
                return;
            }
            // Is address in address book? Address book can miss address when a transaction is
            // sent from outside the UI.
            int idx = addressBook->lookupAddress(address);
            if(idx != -1)
            {
                // Edit sending / receiving address
                QModelIndex modelIdx = addressBook->index(idx, 0, QModelIndex());
                // Determine type of address, launch appropriate editor dialog type
                QString type = modelIdx.data(AddressTableModel::TypeRole).toString();

                auto dlg = new EditAddressDialog(
                    type == AddressTableModel::Receive
                    ? EditAddressDialog::EditReceivingAddress
                    : EditAddressDialog::EditSendingAddress, this);
                dlg->setModel(addressBook);
                dlg->loadRow(idx);
                typename gui_util::ShowModalDialogAndDeleteOnClose(dlg);
            }
            else
            {
                // Add sending address
                auto dlg = new EditAddressDialog(EditAddressDialog::NewSendingAddress,
                    this);
                dlg->setModel(addressBook);
                dlg->setAddress(address);
                typename gui_util::ShowModalDialogAndDeleteOnClose(dlg);
            }
        }
        */
    }
    
    #[Q_SLOT]
    pub fn show_details(&mut self)  {
        
        todo!();
        /*
            if(!transactionView->selectionModel())
            return;
        QModelIndexList selection = transactionView->selectionModel()->selectedRows();
        if(!selection.isEmpty())
        {
            TransactionDescDialog *dlg = new TransactionDescDialog(selection.at(0));
            dlg->setAttribute(QtWA_DeleteOnClose);
            dlg->show();
        }
        */
    }
    
    #[Q_SLOT]
    pub fn open_third_party_tx_url(&mut self, url: String)  {
        
        todo!();
        /*
            if(!transactionView || !transactionView->selectionModel())
            return;
        QModelIndexList selection = transactionView->selectionModel()->selectedRows(0);
        if(!selection.isEmpty())
             QDesktopServices::openUrl(QUrl::fromUserInput(url.replace("%s", selection.at(0).data(TransactionTableModel::TxHashRole).toString())));
        */
    }
    
    pub fn create_date_range_widget(&mut self) -> *mut QWidget {
        
        todo!();
        /*
            dateRangeWidget = new QFrame();
        dateRangeWidget->setFrameStyle(QFrame::Panel | QFrame::Raised);
        dateRangeWidget->setContentsMargins(1,1,1,1);
        QHBoxLayout *layout = new QHBoxLayout(dateRangeWidget);
        layout->setContentsMargins(0,0,0,0);
        layout->addSpacing(23);
        layout->addWidget(new QLabel(tr("Range:")));

        dateFrom = new QDateTimeEdit(this);
        dateFrom->setDisplayFormat("dd/MM/yy");
        dateFrom->setCalendarPopup(true);
        dateFrom->setMinimumWidth(100);
        dateFrom->setDate(QDate::currentDate().addDays(-7));
        layout->addWidget(dateFrom);
        layout->addWidget(new QLabel(tr("to")));

        dateTo = new QDateTimeEdit(this);
        dateTo->setDisplayFormat("dd/MM/yy");
        dateTo->setCalendarPopup(true);
        dateTo->setMinimumWidth(100);
        dateTo->setDate(QDate::currentDate());
        layout->addWidget(dateTo);
        layout->addStretch();

        // Hide by default
        dateRangeWidget->setVisible(false);

        // Notify on change
        connect(dateFrom, &QDateTimeEdit::dateChanged, this, &TransactionView::dateRangeChanged);
        connect(dateTo, &QDateTimeEdit::dateChanged, this, &TransactionView::dateRangeChanged);

        return dateRangeWidget;
        */
    }
    
    #[Q_SLOT]
    pub fn date_range_changed(&mut self)  {
        
        todo!();
        /*
            if(!transactionProxyModel)
            return;
        transactionProxyModel->setDateRange(
                typename gui_util::StartOfDay(dateFrom->date()),
                typename gui_util::StartOfDay(dateTo->date()).addDays(1));
        */
    }
    
    #[Q_SLOT]
    pub fn focus_transaction_with_qmodel_index(&mut self, idx: &QModelIndex)  {
        
        todo!();
        /*
            if(!transactionProxyModel)
            return;
        QModelIndex targetIdx = transactionProxyModel->mapFromSource(idx);
        transactionView->scrollTo(targetIdx);
        transactionView->setCurrentIndex(targetIdx);
        transactionView->setFocus();
        */
    }
    
    #[Q_SLOT]
    pub fn focus_transaction(&mut self, txid: &u256)  {
        
        todo!();
        /*
            if (!transactionProxyModel)
            return;

        const QModelIndexList results = this->model->getTransactionTableModel()->match(
            this->model->getTransactionTableModel()->index(0,0),
            TransactionTableModel::TxHashRole,
            QString::fromStdString(txid.ToString()), -1);

        transactionView->setFocus();
        transactionView->selectionModel()->clearSelection();
        for (const QModelIndex& index : results) {
            const QModelIndex targetIndex = transactionProxyModel->mapFromSource(index);
            transactionView->selectionModel()->select(
                targetIndex,
                QItemSelectionModel::Rows | QItemSelectionModel::Select);
            // Called once per destination to ensure all results are in view, unless
            // transactions are not ordered by (ascending or descending) date.
            transactionView->scrollTo(targetIndex);
            // scrollTo() does not scroll far enough the first time when transactions
            // are ordered by ascending date.
            if (index == results[0]) transactionView->scrollTo(targetIndex);
        }
        */
    }

    /**
      | Need to override default Ctrl+C action
      | for amount as default behaviour is just
      | to copy
      | 
      | DisplayRole text
      |
      */
    pub fn event_filter(&mut self, 
        obj:   *mut QObject,
        event: *mut QEvent) -> bool {
        
        todo!();
        /*
            if (event->type() == QEvent::KeyPress)
        {
            QKeyEvent *ke = static_cast<QKeyEvent *>(event);
            if (ke->key() == QtKey_C && ke->modifiers().testFlag(QtControlModifier))
            {
                 typename gui_util::copyEntryData(transactionView, 0, TransactionTableModel::TxPlainTextRole);
                 return true;
            }
        }
        return QWidget::eventFilter(obj, event);
        */
    }

    /**
      | show/hide column Watch-only
      |
      */
    #[Q_SLOT]
    pub fn update_watch_only_column(&mut self, have_watch_only: bool)  {
        
        todo!();
        /*
            watchOnlyWidget->setVisible(fHaveWatchOnly);
        transactionView->setColumnHidden(TransactionTableModel::Watchonly, !fHaveWatchOnly);
        */
    }
}
