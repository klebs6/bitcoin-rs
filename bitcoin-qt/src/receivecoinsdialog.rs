// ---------------- [ File: bitcoin-qt/src/receivecoinsdialog.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/receivecoinsdialog.h]

/**
  | Dialog for requesting payment of bitcoins
  |
  */
#[Q_OBJECT]
pub struct ReceiveCoinsDialog {
    base:                QDialog,

    ui:                  *mut UiReceiveCoinsDialog,
    model:               *mut WalletModel,
    context_menu:        *mut QMenu,
    copy_label_action:   *mut QAction,
    copy_message_action: *mut QAction,
    copy_amount_action:  *mut QAction,
    platform_style:      *const PlatformStyle,
}

bitflags!{
    pub struct ReceiveCoinsDialogColumnWidths: u32 {
        const DATE_COLUMN_WIDTH           = 130;
        const LABEL_COLUMN_WIDTH          = 120;
        const AMOUNT_MINIMUM_COLUMN_WIDTH = 180;
        const MINIMUM_COLUMN_WIDTH        = 130;
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/receivecoinsdialog.cpp]
impl Drop for ReceiveCoinsDialog {
    fn drop(&mut self) {
        todo!();
        /*
            QSettings settings;
        settings.setValue("RecentRequestsViewHeaderState", ui->recentRequestsView->horizontalHeader()->saveState());
        delete ui;
        */
    }
}

impl ReceiveCoinsDialog {

    pub fn new(
        platform_style: *const PlatformStyle,
        parent:         *mut QWidget) -> Self {
    
        todo!();
        /*
            :
        QDialog(parent, typename gui_util::dialog_flags),
        ui(new UiReceiveCoinsDialog),
        model(nullptr),
        platformStyle(_platformStyle)

        ui->setupUi(this);

        if (!_platformStyle->getImagesOnButtons()) {
            ui->clearButton->setIcon(QIcon());
            ui->receiveButton->setIcon(QIcon());
            ui->showRequestButton->setIcon(QIcon());
            ui->removeRequestButton->setIcon(QIcon());
        } else {
            ui->clearButton->setIcon(_platformStyle->SingleColorIcon(":/icons/remove"));
            ui->receiveButton->setIcon(_platformStyle->SingleColorIcon(":/icons/receiving_addresses"));
            ui->showRequestButton->setIcon(_platformStyle->SingleColorIcon(":/icons/edit"));
            ui->removeRequestButton->setIcon(_platformStyle->SingleColorIcon(":/icons/remove"));
        }

        // context menu
        contextMenu = new QMenu(this);
        contextMenu->addAction(tr("Copy &URI"), this, &ReceiveCoinsDialog::copyURI);
        contextMenu->addAction(tr("&Copy address"), this, &ReceiveCoinsDialog::copyAddress);
        copyLabelAction = contextMenu->addAction(tr("Copy &label"), this, &ReceiveCoinsDialog::copyLabel);
        copyMessageAction = contextMenu->addAction(tr("Copy &message"), this, &ReceiveCoinsDialog::copyMessage);
        copyAmountAction = contextMenu->addAction(tr("Copy &amount"), this, &ReceiveCoinsDialog::copyAmount);
        connect(ui->recentRequestsView, &QWidget::customContextMenuRequested, this, &ReceiveCoinsDialog::showMenu);

        connect(ui->clearButton, &QPushButton::clicked, this, &ReceiveCoinsDialog::clear);

        QTableView* tableView = ui->recentRequestsView;
        tableView->verticalHeader()->hide();
        tableView->setAlternatingRowColors(true);
        tableView->setSelectionBehavior(QAbstractItemView::SelectRows);
        tableView->setSelectionMode(QAbstractItemView::ContiguousSelection);

        QSettings settings;
        if (!tableView->horizontalHeader()->restoreState(settings.value("RecentRequestsViewHeaderState").toByteArray())) {
            tableView->setColumnWidth(RecentRequestsTableModel::Date, DATE_COLUMN_WIDTH);
            tableView->setColumnWidth(RecentRequestsTableModel::Label, LABEL_COLUMN_WIDTH);
            tableView->setColumnWidth(RecentRequestsTableModel::Amount, AMOUNT_MINIMUM_COLUMN_WIDTH);
            tableView->horizontalHeader()->setMinimumSectionSize(MINIMUM_COLUMN_WIDTH);
            tableView->horizontalHeader()->setStretchLastSection(true);
        }
        */
    }
    
    pub fn set_model(&mut self, model: *mut WalletModel)  {
        
        todo!();
        /*
            this->model = _model;

        if(_model && _model->getOptionsModel())
        {
            _model->getRecentRequestsTableModel()->sort(RecentRequestsTableModel::Date, QtDescendingOrder);
            connect(_model->getOptionsModel(), &OptionsModel::displayUnitChanged, this, &ReceiveCoinsDialog::updateDisplayUnit);
            updateDisplayUnit();

            QTableView* tableView = ui->recentRequestsView;
            tableView->setModel(_model->getRecentRequestsTableModel());
            tableView->sortByColumn(RecentRequestsTableModel::Date, QtDescendingOrder);

            connect(tableView->selectionModel(),
                &QItemSelectionModel::selectionChanged, this,
                &ReceiveCoinsDialog::recentRequestsView_selectionChanged);

            if (model->wallet().getDefaultAddressType() == OutputType::BECH32) {
                ui->useBech32->setCheckState(QtChecked);
            } else {
                ui->useBech32->setCheckState(QtUnchecked);
            }

            // Set the button to be enabled or disabled based on whether the wallet can give out new addresses.
            ui->receiveButton->setEnabled(model->wallet().canGetAddresses());

            // Enable/disable the receive button if the wallet is now able/unable to give out new addresses.
            connect(model, &WalletModel::canGetAddressesChanged, [this] {
                ui->receiveButton->setEnabled(model->wallet().canGetAddresses());
            });
        }
        */
    }
    
    #[Q_SLOT]
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            ui->reqAmount->clear();
        ui->reqLabel->setText("");
        ui->reqMessage->setText("");
        updateDisplayUnit();
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
    pub fn update_display_unit(&mut self)  {
        
        todo!();
        /*
            if(model && model->getOptionsModel())
        {
            ui->reqAmount->setDisplayUnit(model->getOptionsModel()->getDisplayUnit());
        }
        */
    }
    
    #[Q_SLOT]
    pub fn on_receive_button_clicked(&mut self)  {
        
        todo!();
        /*
            if(!model || !model->getOptionsModel() || !model->getAddressTableModel() || !model->getRecentRequestsTableModel())
            return;

        QString address;
        QString label = ui->reqLabel->text();
        /* Generate new receiving address */
        OutputType address_type;
        if (ui->useBech32->isChecked()) {
            address_type = OutputType::BECH32;
        } else {
            address_type = model->wallet().getDefaultAddressType();
            if (address_type == OutputType::BECH32) {
                address_type = OutputType::P2SH_SEGWIT;
            }
        }
        address = model->getAddressTableModel()->addRow(AddressTableModel::Receive, label, "", address_type);

        switch(model->getAddressTableModel()->getEditStatus())
        {
        case AddressTableModel::EditStatus::OK: {
            // Success
            SendCoinsRecipient info(address, label,
                ui->reqAmount->value(), ui->reqMessage->text());
            ReceiveRequestDialog *dialog = new ReceiveRequestDialog(this);
            dialog->setAttribute(QtWA_DeleteOnClose);
            dialog->setModel(model);
            dialog->setInfo(info);
            dialog->show();

            /* Store request for later reference */
            model->getRecentRequestsTableModel()->addNewRequest(info);
            break;
        }
        case AddressTableModel::EditStatus::WALLET_UNLOCK_FAILURE:
            QMessageBox::critical(this, windowTitle(),
                tr("Could not unlock wallet."),
                QMessageBox::Ok, QMessageBox::Ok);
            break;
        case AddressTableModel::EditStatus::KEY_GENERATION_FAILURE:
            QMessageBox::critical(this, windowTitle(),
                tr("Could not generate new %1 address").arg(QString::fromStdString(FormatOutputType(address_type))),
                QMessageBox::Ok, QMessageBox::Ok);
            break;
        // These aren't valid return values for our action
        case AddressTableModel::EditStatus::INVALID_ADDRESS:
        case AddressTableModel::EditStatus::DUPLICATE_ADDRESS:
        case AddressTableModel::EditStatus::NO_CHANGES:
            assert(false);
        }
        clear();
        */
    }
    
    #[Q_SLOT]
    pub fn on_recent_requests_view_double_clicked(&mut self, index: &QModelIndex)  {
        
        todo!();
        /*
            const RecentRequestsTableModel *submodel = model->getRecentRequestsTableModel();
        ReceiveRequestDialog *dialog = new ReceiveRequestDialog(this);
        dialog->setModel(model);
        dialog->setInfo(submodel->entry(index.row()).recipient);
        dialog->setAttribute(QtWA_DeleteOnClose);
        dialog->show();
        */
    }
    
    #[Q_SLOT]
    pub fn recent_requests_view_selection_changed(&mut self, 
        selected:   &QItemSelection,
        deselected: &QItemSelection)  {
        
        todo!();
        /*
            // Enable Show/Remove buttons only if anything is selected.
        bool enable = !ui->recentRequestsView->selectionModel()->selectedRows().isEmpty();
        ui->showRequestButton->setEnabled(enable);
        ui->removeRequestButton->setEnabled(enable);
        */
    }
    
    #[Q_SLOT]
    pub fn on_show_request_button_clicked(&mut self)  {
        
        todo!();
        /*
            if(!model || !model->getRecentRequestsTableModel() || !ui->recentRequestsView->selectionModel())
            return;
        QModelIndexList selection = ui->recentRequestsView->selectionModel()->selectedRows();

        for (const QModelIndex& index : selection) {
            on_recentRequestsView_doubleClicked(index);
        }
        */
    }
    
    #[Q_SLOT]
    pub fn on_remove_request_button_clicked(&mut self)  {
        
        todo!();
        /*
            if(!model || !model->getRecentRequestsTableModel() || !ui->recentRequestsView->selectionModel())
            return;
        QModelIndexList selection = ui->recentRequestsView->selectionModel()->selectedRows();
        if(selection.empty())
            return;
        // correct for selection mode ContiguousSelection
        QModelIndex firstIndex = selection.at(0);
        model->getRecentRequestsTableModel()->removeRows(firstIndex.row(), selection.length(), firstIndex.parent());
        */
    }
    
    #[Q_SLOT]
    pub fn selected_row(&mut self) -> QModelIndex {
        
        todo!();
        /*
            if(!model || !model->getRecentRequestsTableModel() || !ui->recentRequestsView->selectionModel())
            return QModelIndex();
        QModelIndexList selection = ui->recentRequestsView->selectionModel()->selectedRows();
        if(selection.empty())
            return QModelIndex();
        // correct for selection mode ContiguousSelection
        QModelIndex firstIndex = selection.at(0);
        return firstIndex;
        */
    }

    /**
      | copy column of selected row to clipboard
      |
      */
    #[Q_SLOT]
    pub fn copy_column_to_clipboard(&mut self, column: i32)  {
        
        todo!();
        /*
            QModelIndex firstIndex = selectedRow();
        if (!firstIndex.isValid()) {
            return;
        }
        typename gui_util::setClipboard(model->getRecentRequestsTableModel()->index(firstIndex.row(), column).data(QtEditRole).toString());
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
            const QModelIndex sel = selectedRow();
        if (!sel.isValid()) {
            return;
        }

        // disable context menu actions when appropriate
        const RecentRequestsTableModel* const submodel = model->getRecentRequestsTableModel();
        const RecentRequestEntry& req = submodel->entry(sel.row());
        copyLabelAction->setDisabled(req.recipient.label.isEmpty());
        copyMessageAction->setDisabled(req.recipient.message.isEmpty());
        copyAmountAction->setDisabled(req.recipient.amount == 0);

        contextMenu->exec(QCursor::pos());
        */
    }

    /**
      | context menu action: copy URI
      |
      */
    #[Q_SLOT]
    pub fn copyuri(&mut self)  {
        
        todo!();
        /*
            QModelIndex sel = selectedRow();
        if (!sel.isValid()) {
            return;
        }

        const RecentRequestsTableModel * const submodel = model->getRecentRequestsTableModel();
        const QString uri = typename gui_util::formatBitcoinURI(submodel->entry(sel.row()).recipient);
        typename gui_util::setClipboard(uri);
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
            const QModelIndex sel = selectedRow();
        if (!sel.isValid()) {
            return;
        }

        const RecentRequestsTableModel* const submodel = model->getRecentRequestsTableModel();
        const QString address = submodel->entry(sel.row()).recipient.address;
        typename gui_util::setClipboard(address);
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
            copyColumnToClipboard(RecentRequestsTableModel::Label);
        */
    }

    /**
      | context menu action: copy message
      |
      */
    #[Q_SLOT]
    pub fn copy_message(&mut self)  {
        
        todo!();
        /*
            copyColumnToClipboard(RecentRequestsTableModel::Message);
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
            copyColumnToClipboard(RecentRequestsTableModel::Amount);
        */
    }
}
