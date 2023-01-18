crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/coincontroldialog.h]

pub const ASYMP_UTF8: &[u8] = &[0xE2, 0x89, 0x88];

pub struct CoinControlWidgetItem {
    base: QTreeWidgetItem,
}

impl CoinControlWidgetItem {

    pub fn new(
        parent: *mut QTreeWidget,
        ty:     Option<i32>) -> Self {
        let ty: i32 = ty.unwrap_or(0);
        todo!();
        /*
        : q_tree_widget_item(parent, type),
        */
    }
    
    pub fn new_with_widget_item(
        parent: *mut QTreeWidgetItem,
        ty:     Option<i32>) -> Self {
        let ty: i32 = ty.unwrap_or(0);
        todo!();
        /*
        : q_tree_widget_item(parent, type),
        */
    }
}

///--------------------------
#[Q_OBJECT]
pub struct CoinControlDialog {
    base:                             QDialog,
    ui:                               Rc<RefCell<UiCoinControlDialog>>,
    coin_control:                     Rc<RefCell<CoinControl>>,
    model:                            Rc<RefCell<WalletModel>>,
    sort_column:                      i32,
    sort_order:                       QSortOrder,
    context_menu:                     Rc<RefCell<QMenu>>,
    context_menu_item:                Rc<RefCell<QTreeWidgetItem>>,
    copy_transaction_outpoint_action: Rc<RefCell<QAction>>,
    lock_action:                      Rc<RefCell<QAction>>,
    unlock_action:                    Rc<RefCell<QAction>>,
    platform_style:                   Rc<PlatformStyle>,
}

pub mod coin_control_dialog {
    use super::*;

    lazy_static!{
        /*
        static QList<CAmount> payAmounts;
            static bool fSubtractFeeFromAmount;
        */
    }

    pub const COLUMN_CHECKBOX:      usize = 0;
    pub const COLUMN_AMOUNT:        usize = 1;
    pub const COLUMN_LABEL:         usize = 2;
    pub const COLUMN_ADDRESS:       usize = 3;
    pub const COLUMN_DATE:          usize = 4;
    pub const COLUMN_CONFIRMATIONS: usize = 5;

    pub const TxHashRole: i32 = USER_ROLE;
    pub const VOutRole:   i32 = USER_ROLE + 1;
}

//-------------------------------------------[.cpp/bitcoin/src/qt/coincontroldialog.cpp]

lazy_static!{
    /*
    QList<CAmount> CoinControlDialog::payAmounts;
    bool CoinControlDialog::fSubtractFeeFromAmount = false;
    */
}

impl PartialEq<QTreeWidgetItem> for CoinControlWidgetItem {
    #[inline] fn eq(&self, other: &QTreeWidgetItem) -> bool {
        todo!();
        /*
            int column = treeWidget()->sortColumn();
        if (column == CoinControlDialog::COLUMN_AMOUNT || column == CoinControlDialog::COLUMN_DATE || column == CoinControlDialog::COLUMN_CONFIRMATIONS)
            return data(column, USER_ROLE).toLongLong() < other.data(column, USER_ROLE).toLongLong();
        return QTreeWidgetItem::operator<(other);
        */
    }
}

impl PartialOrd<QTreeWidgetItem> for CoinControlWidgetItem {
    #[inline] fn partial_cmp(&self, other: &QTreeWidgetItem) -> Option<Ordering> {
        /*
        #[inline] fn cmp(&self, other: &QTreeWidgetItem) -> Ordering {
        }
        */
        todo!();
        //Some(self.cmp(other))
    }
}

///----------------------------
impl Drop for CoinControlDialog {
    fn drop(&mut self) {
        todo!();
        /*
            QSettings settings;
        settings.setValue("nCoinControlMode", ui->radioListMode->isChecked());
        settings.setValue("nCoinControlSortColumn", sortColumn);
        settings.setValue("nCoinControlSortOrder", (int)sortOrder);

        delete ui;
        */
    }
}

impl CoinControlDialog {

    pub fn new(
        coin_control:   &mut CoinControl,
        model:          *mut WalletModel,
        platform_style: *const PlatformStyle,
        parent:         *mut QWidget) -> Self {
    
        todo!();
        /*


            :
        QDialog(parent, typename gui_util::dialog_flags),
        ui(new UiCoinControlDialog),
        m_coin_control(coin_control),
        model(_model),
        platformStyle(_platformStyle)

        ui->setupUi(this);

        // context menu
        contextMenu = new QMenu(this);
        contextMenu->addAction(tr("&Copy address"), this, &CoinControlDialog::copyAddress);
        contextMenu->addAction(tr("Copy &label"), this, &CoinControlDialog::copyLabel);
        contextMenu->addAction(tr("Copy &amount"), this, &CoinControlDialog::copyAmount);
        m_copy_transaction_outpoint_action = contextMenu->addAction(tr("Copy transaction &ID and output index"), this, &CoinControlDialog::copyTransactionOutpoint);
        contextMenu->addSeparator();
        lockAction = contextMenu->addAction(tr("L&ock unspent"), this, &CoinControlDialog::lockCoin);
        unlockAction = contextMenu->addAction(tr("&Unlock unspent"), this, &CoinControlDialog::unlockCoin);
        connect(ui->treeWidget, &QWidget::customContextMenuRequested, this, &CoinControlDialog::showMenu);

        // clipboard actions
        QAction *clipboardQuantityAction = new QAction(tr("Copy quantity"), this);
        QAction *clipboardAmountAction = new QAction(tr("Copy amount"), this);
        QAction *clipboardFeeAction = new QAction(tr("Copy fee"), this);
        QAction *clipboardAfterFeeAction = new QAction(tr("Copy after fee"), this);
        QAction *clipboardBytesAction = new QAction(tr("Copy bytes"), this);
        QAction *clipboardLowOutputAction = new QAction(tr("Copy dust"), this);
        QAction *clipboardChangeAction = new QAction(tr("Copy change"), this);

        connect(clipboardQuantityAction, &QAction::triggered, this, &CoinControlDialog::clipboardQuantity);
        connect(clipboardAmountAction, &QAction::triggered, this, &CoinControlDialog::clipboardAmount);
        connect(clipboardFeeAction, &QAction::triggered, this, &CoinControlDialog::clipboardFee);
        connect(clipboardAfterFeeAction, &QAction::triggered, this, &CoinControlDialog::clipboardAfterFee);
        connect(clipboardBytesAction, &QAction::triggered, this, &CoinControlDialog::clipboardBytes);
        connect(clipboardLowOutputAction, &QAction::triggered, this, &CoinControlDialog::clipboardLowOutput);
        connect(clipboardChangeAction, &QAction::triggered, this, &CoinControlDialog::clipboardChange);

        ui->labelCoinControlQuantity->addAction(clipboardQuantityAction);
        ui->labelCoinControlAmount->addAction(clipboardAmountAction);
        ui->labelCoinControlFee->addAction(clipboardFeeAction);
        ui->labelCoinControlAfterFee->addAction(clipboardAfterFeeAction);
        ui->labelCoinControlBytes->addAction(clipboardBytesAction);
        ui->labelCoinControlLowOutput->addAction(clipboardLowOutputAction);
        ui->labelCoinControlChange->addAction(clipboardChangeAction);

        // toggle tree/list mode
        connect(ui->radioTreeMode, &QRadioButton::toggled, this, &CoinControlDialog::radioTreeMode);
        connect(ui->radioListMode, &QRadioButton::toggled, this, &CoinControlDialog::radioListMode);

        // click on checkbox
        connect(ui->treeWidget, &QTreeWidget::itemChanged, this, &CoinControlDialog::viewItemChanged);

        // click on header
        ui->treeWidget->header()->setSectionsClickable(true);
        connect(ui->treeWidget->header(), &QHeaderView::sectionClicked, this, &CoinControlDialog::headerSectionClicked);

        // ok button
        connect(ui->buttonBox, &QDialogButtonBox::clicked, this, &CoinControlDialog::buttonBoxClicked);

        // (un)select all
        connect(ui->pushButtonSelectAll, &QPushButton::clicked, this, &CoinControlDialog::buttonSelectAllClicked);

        ui->treeWidget->setColumnWidth(COLUMN_CHECKBOX, 84);
        ui->treeWidget->setColumnWidth(COLUMN_AMOUNT, 110);
        ui->treeWidget->setColumnWidth(COLUMN_LABEL, 190);
        ui->treeWidget->setColumnWidth(COLUMN_ADDRESS, 320);
        ui->treeWidget->setColumnWidth(COLUMN_DATE, 130);
        ui->treeWidget->setColumnWidth(COLUMN_CONFIRMATIONS, 110);

        // default view is sorted by amount desc
        sortView(COLUMN_AMOUNT, QtDescendingOrder);

        // restore list mode and sortorder as a convenience feature
        QSettings settings;
        if (settings.contains("nCoinControlMode") && !settings.value("nCoinControlMode").toBool())
            ui->radioTreeMode->click();
        if (settings.contains("nCoinControlSortColumn") && settings.contains("nCoinControlSortOrder"))
            sortView(settings.value("nCoinControlSortColumn").toInt(), (static_cast<QtSortOrder>(settings.value("nCoinControlSortOrder").toInt())));

        typename gui_util::handleCloseWindowShortcut(this);

        if(_model->getOptionsModel() && _model->getAddressTableModel())
        {
            updateView();
            updateLabelLocked();
            CoinControlDialog::updateLabels(m_coin_control, _model, this);
        }
        */
    }

    /**
      | ok button
      |
      */
    #[Q_SLOT]
    pub fn button_box_clicked(&mut self, button: *mut QAbstractButton)  {
        
        todo!();
        /*
            if (ui->buttonBox->buttonRole(button) == QDialogButtonBox::AcceptRole)
            done(QDialog::Accepted); // closes the dialog
        */
    }

    /**
      | (un)select all
      |
      */
    #[Q_SLOT]
    pub fn button_select_all_clicked(&mut self)  {
        
        todo!();
        /*
            QtCheckState state = QtChecked;
        for (int i = 0; i < ui->treeWidget->topLevelItemCount(); i++)
        {
            if (ui->treeWidget->topLevelItem(i)->checkState(COLUMN_CHECKBOX) != QtUnchecked)
            {
                state = QtUnchecked;
                break;
            }
        }
        ui->treeWidget->setEnabled(false);
        for (int i = 0; i < ui->treeWidget->topLevelItemCount(); i++)
                if (ui->treeWidget->topLevelItem(i)->checkState(COLUMN_CHECKBOX) != state)
                    ui->treeWidget->topLevelItem(i)->setCheckState(COLUMN_CHECKBOX, state);
        ui->treeWidget->setEnabled(true);
        if (state == QtUnchecked)
            m_coin_control.UnSelectAll(); // just to be sure
        CoinControlDialog::updateLabels(m_coin_control, model, this);
        */
    }

    /**
      | context menu
      |
      */
    #[Q_SLOT]
    pub fn show_menu(&mut self, point: &QPoint)  {
        
        todo!();
        /*
            QTreeWidgetItem *item = ui->treeWidget->itemAt(point);
        if(item)
        {
            contextMenuItem = item;

            // disable some items (like Copy Transaction ID, lock, unlock) for tree roots in context menu
            if (item->data(COLUMN_ADDRESS, TxHashRole).toString().length() == 64) // transaction hash is 64 characters (this means it is a child node, so it is not a parent node in tree mode)
            {
                m_copy_transaction_outpoint_action->setEnabled(true);
                if (model->wallet().isLockedCoin(OutPoint(uint256S(item->data(COLUMN_ADDRESS, TxHashRole).toString().toStdString()), item->data(COLUMN_ADDRESS, VOutRole).toUInt())))
                {
                    lockAction->setEnabled(false);
                    unlockAction->setEnabled(true);
                }
                else
                {
                    lockAction->setEnabled(true);
                    unlockAction->setEnabled(false);
                }
            }
            else // this means click on parent node in tree mode -> disable all
            {
                m_copy_transaction_outpoint_action->setEnabled(false);
                lockAction->setEnabled(false);
                unlockAction->setEnabled(false);
            }

            // show context menu
            contextMenu->exec(QCursor::pos());
        }
        */
    }

    /**
      | context menu action: copy amount
      |
      */
    #[Q_SLOT]
    pub fn copy_amount(&mut self)  {
        
        todo!();
        /*
            typename gui_util::setClipboard(BitcoinUnits::removeSpaces(contextMenuItem->text(COLUMN_AMOUNT)));
        */
    }

    /**
      | context menu action: copy label
      |
      */
    #[Q_SLOT]
    pub fn copy_label(&mut self)  {
        
        todo!();
        /*
            if (ui->radioTreeMode->isChecked() && contextMenuItem->text(COLUMN_LABEL).length() == 0 && contextMenuItem->parent())
            typename gui_util::setClipboard(contextMenuItem->parent()->text(COLUMN_LABEL));
        else
            typename gui_util::setClipboard(contextMenuItem->text(COLUMN_LABEL));
        */
    }

    /**
      | context menu action: copy address
      |
      */
    #[Q_SLOT]
    pub fn copy_address(&mut self)  {
        
        todo!();
        /*
            if (ui->radioTreeMode->isChecked() && contextMenuItem->text(COLUMN_ADDRESS).length() == 0 && contextMenuItem->parent())
            typename gui_util::setClipboard(contextMenuItem->parent()->text(COLUMN_ADDRESS));
        else
            typename gui_util::setClipboard(contextMenuItem->text(COLUMN_ADDRESS));
        */
    }

    /**
      | context menu action: copy transaction
      | id and vout index
      |
      */
    #[Q_SLOT]
    pub fn copy_transaction_outpoint(&mut self)  {
        
        todo!();
        /*
            const QString address = contextMenuItem->data(COLUMN_ADDRESS, TxHashRole).toString();
        const QString vout = contextMenuItem->data(COLUMN_ADDRESS, VOutRole).toString();
        const QString outpoint = QString("%1:%2").arg(address).arg(vout);

        typename gui_util::setClipboard(outpoint);
        */
    }

    /**
      | context menu action: lock coin
      |
      */
    #[Q_SLOT]
    pub fn lock_coin(&mut self)  {
        
        todo!();
        /*
            if (contextMenuItem->checkState(COLUMN_CHECKBOX) == QtChecked)
            contextMenuItem->setCheckState(COLUMN_CHECKBOX, QtUnchecked);

        OutPoint outpt(uint256S(contextMenuItem->data(COLUMN_ADDRESS, TxHashRole).toString().toStdString()), contextMenuItem->data(COLUMN_ADDRESS, VOutRole).toUInt());
        model->wallet().lockCoin(outpt, /* write_to_db = */ true);
        contextMenuItem->setDisabled(true);
        contextMenuItem->setIcon(COLUMN_CHECKBOX, platformStyle->SingleColorIcon(":/icons/lock_closed"));
        updateLabelLocked();
        */
    }

    /**
      | context menu action: unlock coin
      |
      */
    #[Q_SLOT]
    pub fn unlock_coin(&mut self)  {
        
        todo!();
        /*
            OutPoint outpt(uint256S(contextMenuItem->data(COLUMN_ADDRESS, TxHashRole).toString().toStdString()), contextMenuItem->data(COLUMN_ADDRESS, VOutRole).toUInt());
        model->wallet().unlockCoin(outpt);
        contextMenuItem->setDisabled(false);
        contextMenuItem->setIcon(COLUMN_CHECKBOX, QIcon());
        updateLabelLocked();
        */
    }

    /**
      | copy label "Quantity" to clipboard
      |
      */
    #[Q_SLOT]
    pub fn clipboard_quantity(&mut self)  {
        
        todo!();
        /*
            typename gui_util::setClipboard(ui->labelCoinControlQuantity->text());
        */
    }

    /**
      | copy label "Amount" to clipboard
      |
      */
    #[Q_SLOT]
    pub fn clipboard_amount(&mut self)  {
        
        todo!();
        /*
            typename gui_util::setClipboard(ui->labelCoinControlAmount->text().left(ui->labelCoinControlAmount->text().indexOf(" ")));
        */
    }

    /**
      | copy label "Fee" to clipboard
      |
      */
    #[Q_SLOT]
    pub fn clipboard_fee(&mut self)  {
        
        todo!();
        /*
            typename gui_util::setClipboard(ui->labelCoinControlFee->text().left(ui->labelCoinControlFee->text().indexOf(" ")).replace(ASYMP_UTF8, ""));
        */
    }

    /**
      | copy label "After fee" to clipboard
      |
      */
    #[Q_SLOT]
    pub fn clipboard_after_fee(&mut self)  {
        
        todo!();
        /*
            typename gui_util::setClipboard(ui->labelCoinControlAfterFee->text().left(ui->labelCoinControlAfterFee->text().indexOf(" ")).replace(ASYMP_UTF8, ""));
        */
    }

    /**
      | copy label "Bytes" to clipboard
      |
      */
    #[Q_SLOT]
    pub fn clipboard_bytes(&mut self)  {
        
        todo!();
        /*
            typename gui_util::setClipboard(ui->labelCoinControlBytes->text().replace(ASYMP_UTF8, ""));
        */
    }

    /**
      | copy label "Dust" to clipboard
      |
      */
    #[Q_SLOT]
    pub fn clipboard_low_output(&mut self)  {
        
        todo!();
        /*
            typename gui_util::setClipboard(ui->labelCoinControlLowOutput->text());
        */
    }

    /**
      | copy label "Change" to clipboard
      |
      */
    #[Q_SLOT]
    pub fn clipboard_change(&mut self)  {
        
        todo!();
        /*
            typename gui_util::setClipboard(ui->labelCoinControlChange->text().left(ui->labelCoinControlChange->text().indexOf(" ")).replace(ASYMP_UTF8, ""));
        */
    }

    /**
      | treeview: sort
      |
      */
    pub fn sort_view(&mut self, 
        column: i32,
        order:  QSortOrder)  {
        
        todo!();
        /*
            sortColumn = column;
        sortOrder = order;
        ui->treeWidget->sortItems(column, order);
        ui->treeWidget->header()->setSortIndicator(sortColumn, sortOrder);
        */
    }

    /**
      | treeview: clicked on header
      |
      */
    #[Q_SLOT]
    pub fn header_section_clicked(&mut self, logical_index: i32)  {
        
        todo!();
        /*
            if (logicalIndex == COLUMN_CHECKBOX) // click on most left column -> do nothing
        {
            ui->treeWidget->header()->setSortIndicator(sortColumn, sortOrder);
        }
        else
        {
            if (sortColumn == logicalIndex)
                sortOrder = ((sortOrder == QtAscendingOrder) ? QtDescendingOrder : QtAscendingOrder);
            else
            {
                sortColumn = logicalIndex;
                sortOrder = ((sortColumn == COLUMN_LABEL || sortColumn == COLUMN_ADDRESS) ? QtAscendingOrder : QtDescendingOrder); // if label or address then default => asc, else default => desc
            }

            sortView(sortColumn, sortOrder);
        }
        */
    }

    /**
      | toggle tree mode
      |
      */
    #[Q_SLOT]
    pub fn radio_tree_mode(&mut self, checked: bool)  {
        
        todo!();
        /*
            if (checked && model)
            updateView();
        */
    }

    /**
      | toggle list mode
      |
      */
    #[Q_SLOT]
    pub fn radio_list_mode(&mut self, checked: bool)  {
        
        todo!();
        /*
            if (checked && model)
            updateView();
        */
    }

    /**
      | checkbox clicked by user
      |
      */
    #[Q_SLOT]
    pub fn view_item_changed(&mut self, 
        item:   *mut QTreeWidgetItem,
        column: i32)  {
        
        todo!();
        /*
            if (column == COLUMN_CHECKBOX && item->data(COLUMN_ADDRESS, TxHashRole).toString().length() == 64) // transaction hash is 64 characters (this means it is a child node, so it is not a parent node in tree mode)
        {
            OutPoint outpt(uint256S(item->data(COLUMN_ADDRESS, TxHashRole).toString().toStdString()), item->data(COLUMN_ADDRESS, VOutRole).toUInt());

            if (item->checkState(COLUMN_CHECKBOX) == QtUnchecked)
                m_coin_control.UnSelect(outpt);
            else if (item->isDisabled()) // locked (this happens if "check all" through parent node)
                item->setCheckState(COLUMN_CHECKBOX, QtUnchecked);
            else
                m_coin_control.Select(outpt);

            // selection changed -> update labels
            if (ui->treeWidget->isEnabled()) // do not update on every click for (un)select all
                CoinControlDialog::updateLabels(m_coin_control, model, this);
        }
        */
    }

    /**
      | shows count of locked unspent outputs
      |
      */
    #[Q_SLOT]
    pub fn update_label_locked(&mut self)  {
        
        todo!();
        /*
            std::vector<OutPoint> vOutpts;
        model->wallet().listLockedCoins(vOutpts);
        if (vOutpts.size() > 0)
        {
           ui->labelLocked->setText(tr("(%1 locked)").arg(vOutpts.size()));
           ui->labelLocked->setVisible(true);
        }
        else ui->labelLocked->setVisible(false);
        */
    }
    
    /**
      | static because also called from sendcoinsdialog
      |
      */
    pub fn update_labels(&mut self, 
        coin_control: &mut CoinControl,
        model:        *mut WalletModel,
        dialog:       *mut QDialog)  {
        
        todo!();
        /*
            if (!model)
            return;

        // nPayAmount
        CAmount nPayAmount = 0;
        bool fDust = false;
        for (const CAmount &amount : CoinControlDialog::payAmounts)
        {
            nPayAmount += amount;

            if (amount > 0)
            {
                // Assumes a p2pkh script size
                CTxOut txout(amount, CScript() << std::vector<unsigned char>(24, 0));
                fDust |= IsDust(txout, model->node().getDustRelayFee());
            }
        }

        CAmount nAmount             = 0;
        CAmount nPayFee             = 0;
        CAmount nAfterFee           = 0;
        CAmount nChange             = 0;
        unsigned int nBytes         = 0;
        unsigned int nBytesInputs   = 0;
        unsigned int nQuantity      = 0;
        bool fWitness               = false;

        std::vector<OutPoint> vCoinControl;
        m_coin_control.ListSelected(vCoinControl);

        size_t i = 0;
        for (const auto& out : model->wallet().getCoins(vCoinControl)) {
            if (out.depth_in_main_chain < 0) continue;

            // unselect already spent, very unlikely scenario, this could happen
            // when selected are spent elsewhere, like rpc or another computer
            const OutPoint& outpt = vCoinControl[i++];
            if (out.is_spent)
            {
                m_coin_control.UnSelect(outpt);
                continue;
            }

            // Quantity
            nQuantity++;

            // Amount
            nAmount += out.txout.nValue;

            // Bytes
            TxDestination address;
            int witnessversion = 0;
            std::vector<unsigned char> witnessprogram;
            if (out.txout.scriptPubKey.IsWitnessProgram(witnessversion, witnessprogram))
            {
                nBytesInputs += (32 + 4 + 1 + (107 / WITNESS_SCALE_FACTOR) + 4);
                fWitness = true;
            }
            else if(ExtractDestination(out.txout.scriptPubKey, address))
            {
                CPubKey pubkey;
                PKHash* pkhash = std::get_if<PKHash>(&address);
                if (pkhash && model->wallet().getPubKey(out.txout.scriptPubKey, ToKeyID(*pkhash), pubkey))
                {
                    nBytesInputs += (pubkey.IsCompressed() ? 148 : 180);
                }
                else
                    nBytesInputs += 148; // in all error cases, simply assume 148 here
            }
            else nBytesInputs += 148;
        }

        // calculation
        if (nQuantity > 0)
        {
            // Bytes
            nBytes = nBytesInputs + ((CoinControlDialog::payAmounts.size() > 0 ? CoinControlDialog::payAmounts.size() + 1 : 2) * 34) + 10; // always assume +1 output for change here
            if (fWitness)
            {
                // there is some fudging in these numbers related to the actual virtual transaction size calculation that will keep this estimate from being exact.
                // usually, the result will be an overestimate within a couple of satoshis so that the confirmation dialog ends up displaying a slightly smaller fee.
                // also, the witness stack size value is a variable sized integer. usually, the number of stack items will be well under the single byte var int limit.
                nBytes += 2; // account for the serialized marker and flag bytes
                nBytes += nQuantity; // account for the witness byte that holds the number of stack items for each input.
            }

            // in the subtract fee from amount case, we can tell if zero change already and subtract the bytes, so that fee calculation afterwards is accurate
            if (CoinControlDialog::fSubtractFeeFromAmount)
                if (nAmount - nPayAmount == 0)
                    nBytes -= 34;

            // Fee
            nPayFee = model->wallet().getMinimumFee(nBytes, m_coin_control, nullptr /* returned_target */, nullptr /* reason */);

            if (nPayAmount > 0)
            {
                nChange = nAmount - nPayAmount;
                if (!CoinControlDialog::fSubtractFeeFromAmount)
                    nChange -= nPayFee;

                // Never create dust outputs; if we would, just add the dust to the fee.
                if (nChange > 0 && nChange < MIN_CHANGE)
                {
                    // Assumes a p2pkh script size
                    CTxOut txout(nChange, CScript() << std::vector<unsigned char>(24, 0));
                    if (IsDust(txout, model->node().getDustRelayFee()))
                    {
                        nPayFee += nChange;
                        nChange = 0;
                        if (CoinControlDialog::fSubtractFeeFromAmount)
                            nBytes -= 34; // we didn't detect lack of change above
                    }
                }

                if (nChange == 0 && !CoinControlDialog::fSubtractFeeFromAmount)
                    nBytes -= 34;
            }

            // after fee
            nAfterFee = std::max<CAmount>(nAmount - nPayFee, 0);
        }

        // actually update labels
        int nDisplayUnit = BitcoinUnits::BTC;
        if (model && model->getOptionsModel())
            nDisplayUnit = model->getOptionsModel()->getDisplayUnit();

        QLabel *l1 = dialog->findChild<QLabel *>("labelCoinControlQuantity");
        QLabel *l2 = dialog->findChild<QLabel *>("labelCoinControlAmount");
        QLabel *l3 = dialog->findChild<QLabel *>("labelCoinControlFee");
        QLabel *l4 = dialog->findChild<QLabel *>("labelCoinControlAfterFee");
        QLabel *l5 = dialog->findChild<QLabel *>("labelCoinControlBytes");
        QLabel *l7 = dialog->findChild<QLabel *>("labelCoinControlLowOutput");
        QLabel *l8 = dialog->findChild<QLabel *>("labelCoinControlChange");

        // enable/disable "dust" and "change"
        dialog->findChild<QLabel *>("labelCoinControlLowOutputText")->setEnabled(nPayAmount > 0);
        dialog->findChild<QLabel *>("labelCoinControlLowOutput")    ->setEnabled(nPayAmount > 0);
        dialog->findChild<QLabel *>("labelCoinControlChangeText")   ->setEnabled(nPayAmount > 0);
        dialog->findChild<QLabel *>("labelCoinControlChange")       ->setEnabled(nPayAmount > 0);

        // stats
        l1->setText(QString::number(nQuantity));                                 // Quantity
        l2->setText(BitcoinUnits::formatWithUnit(nDisplayUnit, nAmount));        // Amount
        l3->setText(BitcoinUnits::formatWithUnit(nDisplayUnit, nPayFee));        // Fee
        l4->setText(BitcoinUnits::formatWithUnit(nDisplayUnit, nAfterFee));      // After Fee
        l5->setText(((nBytes > 0) ? ASYMP_UTF8 : "") + QString::number(nBytes));        // Bytes
        l7->setText(fDust ? tr("yes") : tr("no"));                               // Dust
        l8->setText(BitcoinUnits::formatWithUnit(nDisplayUnit, nChange));        // Change
        if (nPayFee > 0)
        {
            l3->setText(ASYMP_UTF8 + l3->text());
            l4->setText(ASYMP_UTF8 + l4->text());
            if (nChange > 0 && !CoinControlDialog::fSubtractFeeFromAmount)
                l8->setText(ASYMP_UTF8 + l8->text());
        }

        // turn label red when dust
        l7->setStyleSheet((fDust) ? "color:red;" : "");

        // tool tips
        QString toolTipDust = tr("This label turns red if any recipient receives an amount smaller than the current dust threshold.");

        // how many satoshis the estimated fee can vary per byte we guess wrong
        double dFeeVary = (nBytes != 0) ? (double)nPayFee / nBytes : 0;

        QString toolTip4 = tr("Can vary +/- %1 satoshi(s) per input.").arg(dFeeVary);

        l3->setToolTip(toolTip4);
        l4->setToolTip(toolTip4);
        l7->setToolTip(toolTipDust);
        l8->setToolTip(toolTip4);
        dialog->findChild<QLabel *>("labelCoinControlFeeText")      ->setToolTip(l3->toolTip());
        dialog->findChild<QLabel *>("labelCoinControlAfterFeeText") ->setToolTip(l4->toolTip());
        dialog->findChild<QLabel *>("labelCoinControlBytesText")    ->setToolTip(l5->toolTip());
        dialog->findChild<QLabel *>("labelCoinControlLowOutputText")->setToolTip(l7->toolTip());
        dialog->findChild<QLabel *>("labelCoinControlChangeText")   ->setToolTip(l8->toolTip());

        // Insufficient funds
        QLabel *label = dialog->findChild<QLabel *>("labelCoinControlInsuffFunds");
        if (label)
            label->setVisible(nChange < 0);
        */
    }
    
    pub fn change_event(&mut self, e: *mut QEvent)  {
        
        todo!();
        /*
            if (e->type() == QEvent::PaletteChange) {
            updateView();
        }

        QDialog::changeEvent(e);
        */
    }
    
    pub fn update_view(&mut self)  {
        
        todo!();
        /*
            if (!model || !model->getOptionsModel() || !model->getAddressTableModel())
            return;

        bool treeMode = ui->radioTreeMode->isChecked();

        ui->treeWidget->clear();
        ui->treeWidget->setEnabled(false); // performance, otherwise updateLabels would be called for every checked checkbox
        ui->treeWidget->setAlternatingRowColors(!treeMode);
        QFlags<QtItemFlag> flgCheckbox = QtItemIsSelectable | QtItemIsEnabled | QtItemIsUserCheckable;
        QFlags<QtItemFlag> flgTristate = QtItemIsSelectable | QtItemIsEnabled | QtItemIsUserCheckable | QtItemIsTristate;

        int nDisplayUnit = model->getOptionsModel()->getDisplayUnit();

        for (const auto& coins : model->wallet().listCoins()) {
            CCoinControlWidgetItem* itemWalletAddress{nullptr};
            QString sWalletAddress = QString::fromStdString(EncodeDestination(coins.first));
            QString sWalletLabel = model->getAddressTableModel()->labelForAddress(sWalletAddress);
            if (sWalletLabel.isEmpty())
                sWalletLabel = tr("(no label)");

            if (treeMode)
            {
                // wallet address
                itemWalletAddress = new CCoinControlWidgetItem(ui->treeWidget);

                itemWalletAddress->setFlags(flgTristate);
                itemWalletAddress->setCheckState(COLUMN_CHECKBOX, QtUnchecked);

                // label
                itemWalletAddress->setText(COLUMN_LABEL, sWalletLabel);

                // address
                itemWalletAddress->setText(COLUMN_ADDRESS, sWalletAddress);
            }

            CAmount nSum = 0;
            int nChildren = 0;
            for (const auto& outpair : coins.second) {
                const OutPoint& output = std::get<0>(outpair);
                const typename interfaces::WalletTxOut& out = std::get<1>(outpair);
                nSum += out.txout.nValue;
                nChildren++;

                CCoinControlWidgetItem *itemOutput;
                if (treeMode)    itemOutput = new CCoinControlWidgetItem(itemWalletAddress);
                else             itemOutput = new CCoinControlWidgetItem(ui->treeWidget);
                itemOutput->setFlags(flgCheckbox);
                itemOutput->setCheckState(COLUMN_CHECKBOX,QtUnchecked);

                // address
                TxDestination outputAddress;
                QString sAddress = "";
                if(ExtractDestination(out.txout.scriptPubKey, outputAddress))
                {
                    sAddress = QString::fromStdString(EncodeDestination(outputAddress));

                    // if listMode or change => show bitcoin address. In tree mode, address is not shown again for direct wallet address outputs
                    if (!treeMode || (!(sAddress == sWalletAddress)))
                        itemOutput->setText(COLUMN_ADDRESS, sAddress);
                }

                // label
                if (!(sAddress == sWalletAddress)) // change
                {
                    // tooltip from where the change comes from
                    itemOutput->setToolTip(COLUMN_LABEL, tr("change from %1 (%2)").arg(sWalletLabel).arg(sWalletAddress));
                    itemOutput->setText(COLUMN_LABEL, tr("(change)"));
                }
                else if (!treeMode)
                {
                    QString sLabel = model->getAddressTableModel()->labelForAddress(sAddress);
                    if (sLabel.isEmpty())
                        sLabel = tr("(no label)");
                    itemOutput->setText(COLUMN_LABEL, sLabel);
                }

                // amount
                itemOutput->setText(COLUMN_AMOUNT, BitcoinUnits::format(nDisplayUnit, out.txout.nValue));
                itemOutput->setData(COLUMN_AMOUNT, QtUserRole, QVariant((qlonglong)out.txout.nValue)); // padding so that sorting works correctly

                // date
                itemOutput->setText(COLUMN_DATE, typename gui_util::dateTimeStr(out.time));
                itemOutput->setData(COLUMN_DATE, QtUserRole, QVariant((qlonglong)out.time));

                // confirmations
                itemOutput->setText(COLUMN_CONFIRMATIONS, QString::number(out.depth_in_main_chain));
                itemOutput->setData(COLUMN_CONFIRMATIONS, QtUserRole, QVariant((qlonglong)out.depth_in_main_chain));

                // transaction hash
                itemOutput->setData(COLUMN_ADDRESS, TxHashRole, QString::fromStdString(output.hash.GetHex()));

                // vout index
                itemOutput->setData(COLUMN_ADDRESS, VOutRole, output.n);

                 // disable locked coins
                if (model->wallet().isLockedCoin(output))
                {
                    m_coin_control.UnSelect(output); // just to be sure
                    itemOutput->setDisabled(true);
                    itemOutput->setIcon(COLUMN_CHECKBOX, platformStyle->SingleColorIcon(":/icons/lock_closed"));
                }

                // set checkbox
                if (m_coin_control.IsSelected(output))
                    itemOutput->setCheckState(COLUMN_CHECKBOX, QtChecked);
            }

            // amount
            if (treeMode)
            {
                itemWalletAddress->setText(COLUMN_CHECKBOX, "(" + QString::number(nChildren) + ")");
                itemWalletAddress->setText(COLUMN_AMOUNT, BitcoinUnits::format(nDisplayUnit, nSum));
                itemWalletAddress->setData(COLUMN_AMOUNT, QtUserRole, QVariant((qlonglong)nSum));
            }
        }

        // expand all partially selected
        if (treeMode)
        {
            for (int i = 0; i < ui->treeWidget->topLevelItemCount(); i++)
                if (ui->treeWidget->topLevelItem(i)->checkState(COLUMN_CHECKBOX) == QtPartiallyChecked)
                    ui->treeWidget->topLevelItem(i)->setExpanded(true);
        }

        // sort view
        sortView(sortColumn, sortOrder);
        ui->treeWidget->setEnabled(true);
        */
    }
}
