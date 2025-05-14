// ---------------- [ File: bitcoin-qt/src/overviewpage.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/overviewpage.h]

/**
  | Overview ("home") page widget
  |
  */
#[Q_OBJECT]
pub struct OverviewPage {
    base:           QWidget,
    ui:             *mut UiOverviewPage,
    client_model:   *mut ClientModel,
    wallet_model:   *mut WalletModel,
    balances:       WalletBalances,
    privacy:        bool, // default = { false }
    platform_style: *const PlatformStyle,
    txdelegate:     *mut TxViewDelegate,
    filter:         Box<TransactionFilterProxy>,
}

impl Drop for OverviewPage {
    fn drop(&mut self) {
        todo!();
        /*
            delete ui;
        */
    }
}

impl OverviewPage {

    #[Q_SIGNAL]
    pub fn transaction_clicked(&mut self, index: &QModelIndex)  {
        
        todo!();
        /*
        
        */
    }
    
    #[Q_SIGNAL]
    pub fn out_of_sync_warning_clicked(&mut self)  {
        
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
        QWidget(parent),
        ui(new UiOverviewPage),
        clientModel(nullptr),
        walletModel(nullptr),
        m_platform_style{platformStyle},
        txdelegate(new TxViewDelegate(platformStyle, this))

        ui->setupUi(this);

        m_balances.balance = -1;

        // use a SingleColorIcon for the "out of sync warning" icon
        QIcon icon = m_platform_style->SingleColorIcon(QStringLiteral(":/icons/warning"));
        ui->labelTransactionsStatus->setIcon(icon);
        ui->labelWalletStatus->setIcon(icon);

        // Recent transactions
        ui->listTransactions->setItemDelegate(txdelegate);
        ui->listTransactions->setIconSize(QSize(DECORATION_SIZE, DECORATION_SIZE));
        ui->listTransactions->setMinimumHeight(NUM_ITEMS * (DECORATION_SIZE + 2));
        ui->listTransactions->setAttribute(QtWA_MacShowFocusRect, false);

        connect(ui->listTransactions, &TransactionOverviewWidget::clicked, this, &OverviewPage::handleTransactionClicked);

        // start with displaying the "out of sync" warnings
        showOutOfSyncWarning(true);
        connect(ui->labelWalletStatus, &QPushButton::clicked, this, &OverviewPage::outOfSyncWarningClicked);
        connect(ui->labelTransactionsStatus, &QPushButton::clicked, this, &OverviewPage::outOfSyncWarningClicked);
        */
    }
    
    #[Q_SLOT]
    pub fn handle_transaction_clicked(&mut self, index: &QModelIndex)  {
        
        todo!();
        /*
            if(filter)
            Q_EMIT transactionClicked(filter->mapToSource(index));
        */
    }
    
    #[Q_SLOT]
    pub fn set_privacy(&mut self, privacy: bool)  {
        
        todo!();
        /*
            m_privacy = privacy;
        if (m_balances.balance != -1) {
            setBalance(m_balances);
        }

        ui->listTransactions->setVisible(!m_privacy);

        const QString status_tip = m_privacy ? tr("Privacy mode activated for the Overview tab. To unmask the values, uncheck Settings->Mask values.") : "";
        setStatusTip(status_tip);
        QStatusTipEvent event(status_tip);
        QApplication::sendEvent(this, &event);
        */
    }
    
    #[Q_SLOT]
    pub fn set_balance(&mut self, balances: &WalletBalances)  {
        
        todo!();
        /*
            int unit = walletModel->getOptionsModel()->getDisplayUnit();
        m_balances = balances;
        if (walletModel->wallet().isLegacy()) {
            if (walletModel->wallet().privateKeysDisabled()) {
                ui->labelBalance->setText(BitcoinUnits::formatWithPrivacy(unit, balances.watch_only_balance, BitcoinUnits::SeparatorStyle::ALWAYS, m_privacy));
                ui->labelUnconfirmed->setText(BitcoinUnits::formatWithPrivacy(unit, balances.unconfirmed_watch_only_balance, BitcoinUnits::SeparatorStyle::ALWAYS, m_privacy));
                ui->labelImmature->setText(BitcoinUnits::formatWithPrivacy(unit, balances.immature_watch_only_balance, BitcoinUnits::SeparatorStyle::ALWAYS, m_privacy));
                ui->labelTotal->setText(BitcoinUnits::formatWithPrivacy(unit, balances.watch_only_balance + balances.unconfirmed_watch_only_balance + balances.immature_watch_only_balance, BitcoinUnits::SeparatorStyle::ALWAYS, m_privacy));
            } else {
                ui->labelBalance->setText(BitcoinUnits::formatWithPrivacy(unit, balances.balance, BitcoinUnits::SeparatorStyle::ALWAYS, m_privacy));
                ui->labelUnconfirmed->setText(BitcoinUnits::formatWithPrivacy(unit, balances.unconfirmed_balance, BitcoinUnits::SeparatorStyle::ALWAYS, m_privacy));
                ui->labelImmature->setText(BitcoinUnits::formatWithPrivacy(unit, balances.immature_balance, BitcoinUnits::SeparatorStyle::ALWAYS, m_privacy));
                ui->labelTotal->setText(BitcoinUnits::formatWithPrivacy(unit, balances.balance + balances.unconfirmed_balance + balances.immature_balance, BitcoinUnits::SeparatorStyle::ALWAYS, m_privacy));
                ui->labelWatchAvailable->setText(BitcoinUnits::formatWithPrivacy(unit, balances.watch_only_balance, BitcoinUnits::SeparatorStyle::ALWAYS, m_privacy));
                ui->labelWatchPending->setText(BitcoinUnits::formatWithPrivacy(unit, balances.unconfirmed_watch_only_balance, BitcoinUnits::SeparatorStyle::ALWAYS, m_privacy));
                ui->labelWatchImmature->setText(BitcoinUnits::formatWithPrivacy(unit, balances.immature_watch_only_balance, BitcoinUnits::SeparatorStyle::ALWAYS, m_privacy));
                ui->labelWatchTotal->setText(BitcoinUnits::formatWithPrivacy(unit, balances.watch_only_balance + balances.unconfirmed_watch_only_balance + balances.immature_watch_only_balance, BitcoinUnits::SeparatorStyle::ALWAYS, m_privacy));
            }
        } else {
            ui->labelBalance->setText(BitcoinUnits::formatWithPrivacy(unit, balances.balance, BitcoinUnits::SeparatorStyle::ALWAYS, m_privacy));
            ui->labelUnconfirmed->setText(BitcoinUnits::formatWithPrivacy(unit, balances.unconfirmed_balance, BitcoinUnits::SeparatorStyle::ALWAYS, m_privacy));
            ui->labelImmature->setText(BitcoinUnits::formatWithPrivacy(unit, balances.immature_balance, BitcoinUnits::SeparatorStyle::ALWAYS, m_privacy));
            ui->labelTotal->setText(BitcoinUnits::formatWithPrivacy(unit, balances.balance + balances.unconfirmed_balance + balances.immature_balance, BitcoinUnits::SeparatorStyle::ALWAYS, m_privacy));
        }
        // only show immature (newly mined) balance if it's non-zero, so as not to complicate things
        // for the non-mining users
        bool showImmature = balances.immature_balance != 0;
        bool showWatchOnlyImmature = balances.immature_watch_only_balance != 0;

        // for symmetry reasons also show immature label when the watch-only one is shown
        ui->labelImmature->setVisible(showImmature || showWatchOnlyImmature);
        ui->labelImmatureText->setVisible(showImmature || showWatchOnlyImmature);
        ui->labelWatchImmature->setVisible(!walletModel->wallet().privateKeysDisabled() && showWatchOnlyImmature); // show watch-only immature balance
        */
    }

    /**
      | show/hide watch-only labels
      |
      */
    #[Q_SLOT]
    pub fn update_watch_only_labels(&mut self, show_watch_only: bool)  {
        
        todo!();
        /*
            ui->labelSpendable->setVisible(showWatchOnly);      // show spendable label (only when watch-only is active)
        ui->labelWatchonly->setVisible(showWatchOnly);      // show watch-only label
        ui->lineWatchBalance->setVisible(showWatchOnly);    // show watch-only balance separator line
        ui->labelWatchAvailable->setVisible(showWatchOnly); // show watch-only available balance
        ui->labelWatchPending->setVisible(showWatchOnly);   // show watch-only pending balance
        ui->labelWatchTotal->setVisible(showWatchOnly);     // show watch-only total balance

        if (!showWatchOnly)
            ui->labelWatchImmature->hide();
        */
    }
    
    pub fn set_client_model(&mut self, model: *mut ClientModel)  {
        
        todo!();
        /*
            this->clientModel = model;
        if (model) {
            // Show warning, for example if this is a prerelease version
            connect(model, &ClientModel::alertsChanged, this, &OverviewPage::updateAlerts);
            updateAlerts(model->getStatusBarWarnings());

            connect(model->getOptionsModel(), &OptionsModel::useEmbeddedMonospacedFontChanged, this, &OverviewPage::setMonospacedFont);
            setMonospacedFont(model->getOptionsModel()->getUseEmbeddedMonospacedFont());
        }
        */
    }
    
    pub fn set_wallet_model(&mut self, model: *mut WalletModel)  {
        
        todo!();
        /*
            this->walletModel = model;
        if(model && model->getOptionsModel())
        {
            // Set up transaction list
            filter.reset(new TransactionFilterProxy());
            filter->setSourceModel(model->getTransactionTableModel());
            filter->setLimit(NUM_ITEMS);
            filter->setDynamicSortFilter(true);
            filter->setSortRole(QtEditRole);
            filter->setShowInactive(false);
            filter->sort(TransactionTableModel::Date, QtDescendingOrder);

            ui->listTransactions->setModel(filter.get());
            ui->listTransactions->setModelColumn(TransactionTableModel::ToAddress);

            // Keep up to date with wallet
            typename interfaces::Wallet& wallet = model->wallet();
            typename interfaces::WalletBalances balances = wallet.getBalances();
            setBalance(balances);
            connect(model, &WalletModel::balanceChanged, this, &OverviewPage::setBalance);

            connect(model->getOptionsModel(), &OptionsModel::displayUnitChanged, this, &OverviewPage::updateDisplayUnit);

            updateWatchOnlyLabels(wallet.haveWatchOnly() && !model->wallet().privateKeysDisabled());
            connect(model, &WalletModel::notifyWatchonlyChanged, [this](bool showWatchOnly) {
                updateWatchOnlyLabels(showWatchOnly && !walletModel->wallet().privateKeysDisabled());
            });
        }

        // update the display unit, to not use the default ("BTC")
        updateDisplayUnit();
        */
    }
    
    pub fn change_event(&mut self, e: *mut QEvent)  {
        
        todo!();
        /*
            if (e->type() == QEvent::PaletteChange) {
            QIcon icon = m_platform_style->SingleColorIcon(QStringLiteral(":/icons/warning"));
            ui->labelTransactionsStatus->setIcon(icon);
            ui->labelWalletStatus->setIcon(icon);
        }

        QWidget::changeEvent(e);
        */
    }
    
    #[Q_SLOT]
    pub fn update_display_unit(&mut self)  {
        
        todo!();
        /*
            if(walletModel && walletModel->getOptionsModel())
        {
            if (m_balances.balance != -1) {
                setBalance(m_balances);
            }

            // Update txdelegate->unit with the current unit
            txdelegate->unit = walletModel->getOptionsModel()->getDisplayUnit();

            ui->listTransactions->update();
        }
        */
    }
    
    #[Q_SLOT]
    pub fn update_alerts(&mut self, warnings: &String)  {
        
        todo!();
        /*
            this->ui->labelAlerts->setVisible(!warnings.isEmpty());
        this->ui->labelAlerts->setText(warnings);
        */
    }
    
    pub fn show_out_of_sync_warning(&mut self, show: bool)  {
        
        todo!();
        /*
            ui->labelWalletStatus->setVisible(fShow);
        ui->labelTransactionsStatus->setVisible(fShow);
        */
    }
    
    #[Q_SLOT]
    pub fn set_monospaced_font(&mut self, use_embedded_font: bool)  {
        
        todo!();
        /*
            QFont f = typename gui_util::fixedPitchFont(use_embedded_font);
        f.setWeight(QFont::Bold);
        ui->labelBalance->setFont(f);
        ui->labelUnconfirmed->setFont(f);
        ui->labelImmature->setFont(f);
        ui->labelTotal->setFont(f);
        ui->labelWatchAvailable->setFont(f);
        ui->labelWatchPending->setFont(f);
        ui->labelWatchImmature->setFont(f);
        ui->labelWatchTotal->setFont(f);
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/overviewpage.cpp]

pub const DECORATION_SIZE: usize = 54;
pub const NUM_ITEMS:       usize = 5;

#[Q_OBJECT]
pub struct TxViewDelegate {
    base:           QAbstractItemDelegate,
    unit:           i32,
    platform_style: *const PlatformStyle,
    minimum_width:  RefCell<HashMap<i32,i32>>,
}

impl TxViewDelegate {
    
    pub fn new(
        platform_style: &PlatformStyle,
        parent:         Option<&mut QObject>) -> Self {

        todo!();
        /*
            :
            QAbstractItemDelegate(parent), unit(BitcoinUnits::BTC),
            platformStyle(_platformStyle)

            connect(this, &TxViewDelegate::width_changed, this, &TxViewDelegate::sizeHintChanged);
        */
    }
    
    #[inline] pub fn paint(&self, 
        painter: *mut QPainter,
        option:  &QStyleOptionViewItem,
        index:   &QModelIndex)  {
        
        todo!();
        /*
            painter->save();

            QIcon icon = qvariant_cast<QIcon>(index.data(TransactionTableModel::RawDecorationRole));
            QRect mainRect = option.rect;
            QRect decorationRect(mainRect.topLeft(), QSize(DECORATION_SIZE, DECORATION_SIZE));
            int xspace = DECORATION_SIZE + 8;
            int ypad = 6;
            int halfheight = (mainRect.height() - 2*ypad)/2;
            QRect amountRect(mainRect.left() + xspace, mainRect.top()+ypad, mainRect.width() - xspace, halfheight);
            QRect addressRect(mainRect.left() + xspace, mainRect.top()+ypad+halfheight, mainRect.width() - xspace, halfheight);
            icon = platformStyle->SingleColorIcon(icon);
            icon.paint(painter, decorationRect);

            QDateTime date = index.data(TransactionTableModel::DateRole).toDateTime();
            QString address = index.data(QtDisplayRole).toString();
            i64 amount = index.data(TransactionTableModel::AmountRole).toLongLong();
            bool confirmed = index.data(TransactionTableModel::ConfirmedRole).toBool();
            QVariant value = index.data(QtForegroundRole);
            QColor foreground = option.palette.color(QPalette::Text);
            if(value.canConvert<QBrush>())
            {
                QBrush brush = qvariant_cast<QBrush>(value);
                foreground = brush.color();
            }

            if (index.data(TransactionTableModel::WatchonlyRole).toBool()) {
                QIcon iconWatchonly = qvariant_cast<QIcon>(index.data(TransactionTableModel::WatchonlyDecorationRole));
                QRect watchonlyRect(addressRect.left(), addressRect.top(), 16, addressRect.height());
                iconWatchonly = platformStyle->TextColorIcon(iconWatchonly);
                iconWatchonly.paint(painter, watchonlyRect);
                addressRect.setLeft(addressRect.left() + watchonlyRect.width() + 5);
            }

            painter->setPen(foreground);
            QRect boundingRect;
            painter->drawText(addressRect, QtAlignLeft | QtAlignVCenter, address, &boundingRect);

            if(amount < 0)
            {
                foreground = COLOR_NEGATIVE;
            }
            else if(!confirmed)
            {
                foreground = COLOR_UNCONFIRMED;
            }
            else
            {
                foreground = option.palette.color(QPalette::Text);
            }
            painter->setPen(foreground);
            QString amountText = BitcoinUnits::formatWithUnit(unit, amount, true, BitcoinUnits::SeparatorStyle::ALWAYS);
            if(!confirmed)
            {
                amountText = QString("[") + amountText + QString("]");
            }

            QRect amount_bounding_rect;
            painter->drawText(amountRect, QtAlignRight | QtAlignVCenter, amountText, &amount_bounding_rect);

            painter->setPen(option.palette.color(QPalette::Text));
            QRect date_bounding_rect;
            painter->drawText(amountRect, QtAlignLeft | QtAlignVCenter, typename gui_util::dateTimeStr(date), &date_bounding_rect);

            // 0.4*date_bounding_rect.width() is used to visually distinguish a date from an amount.
            const int minimum_width = 1.4 * date_bounding_rect.width() + amount_bounding_rect.width();
            const auto search = m_minimum_width.find(index.row());
            if (search == m_minimum_width.end() || search->second != minimum_width) {
                m_minimum_width[index.row()] = minimum_width;
                Q_EMIT width_changed(index);
            }

            painter->restore();
        */
    }
    
    #[inline] pub fn size_hint(&self, 
        option: &QStyleOptionViewItem,
        index:  &QModelIndex) -> QSize {
        
        todo!();
        /*
            const auto search = m_minimum_width.find(index.row());
            const int minimum_text_width = search == m_minimum_width.end() ? 0 : search->second;
            return {DECORATION_SIZE + 8 + minimum_text_width, DECORATION_SIZE};
        */
    }

    /**
      | An intermediate signal for emitting
      | from the `paint() const` member function.
      |
      */
    #[Q_SIGNAL]
    pub fn width_changed(&self, index: &QModelIndex)  {
        
        todo!();
        /*
        
        */
    }
}
