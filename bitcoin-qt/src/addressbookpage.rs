crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/addressbookpage.h]

/**
  | Widget that shows a list of sending or
  | receiving addresses.
  |
  */
#[Q_OBJECT]
pub struct AddressBookPage {
    base:                  QDialog,
    ui:                    *mut UiAddressBookPage,
    model:                 *mut AddressTableModel,
    mode:                  AddressBookPageMode,
    tab:                   AddressBookPageTabs,
    return_value:          String,
    proxy_model:           *mut AddressBookSortFilterProxyModel,
    context_menu:          *mut QMenu,
    new_address_to_select: String,
}

pub enum AddressBookPageTabs {
    SendingTab   = 0,
    ReceivingTab = 1
}

pub enum AddressBookPageMode {

    /**
      | Open address book to pick address
      |
      */
    ForSelection, 

    /**
      | Open address book for editing
      |
      */
    ForEditing  
}

impl AddressBookPage {

    pub fn get_return_value(&self) -> &String {
        
        todo!();
        /*
            return returnValue;
        */
    }

    #[Q_SIGNAL]
    pub fn send_coins(&mut self, addr: String)  {
        
        todo!();
        /*
        
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/addressbookpage.cpp]

pub struct AddressBookSortFilterProxyModel {
    base: QSortFilterProxyModel,
    ty:   String,
}

impl AddressBookSortFilterProxyModel {

    pub fn new(
        ty:     &String,
        parent: *mut QObject) -> Self {
    
        todo!();
        /*
        : q_sort_filter_proxy_model(parent),
        : ty(type),

            setDynamicSortFilter(true);
            setFilterCaseSensitivity(QtCaseInsensitive);
            setSortCaseSensitivity(QtCaseInsensitive);
        */
    }
    
    pub fn filter_accepts_row(&self, 
        row:    i32,
        parent: &QModelIndex) -> bool {
        
        todo!();
        /*
            auto model = sourceModel();
            auto label = model->index(row, AddressTableModel::Label, parent);

            if (model->data(label, AddressTableModel::TypeRole).toString() != m_type) {
                return false;
            }

            auto address = model->index(row, AddressTableModel::Address, parent);

            if (filterRegExp().indexIn(model->data(address).toString()) < 0 &&
                filterRegExp().indexIn(model->data(label).toString()) < 0) {
                return false;
            }

            return true;
        */
    }
}

impl Drop for AddressBookPage {

    fn drop(&mut self) {
        todo!();
        /*
            delete ui;
        */
    }
}

impl AddressBookPage {

    pub fn new(
        platform_style: *const PlatformStyle,
        mode:           AddressBookPageMode,
        tab:            AddressBookPageTabs,
        parent:         *mut QWidget) -> Self {
    
        todo!();
        /*


            :
        QDialog(parent, gui_util::dialog_flags),
        ui(new UiAddressBookPage),
        model(nullptr),
        mode(_mode),
        tab(_tab)
        ui->setupUi(this);

        if (!platformStyle->getImagesOnButtons()) {
            ui->newAddress->setIcon(QIcon());
            ui->copyAddress->setIcon(QIcon());
            ui->deleteAddress->setIcon(QIcon());
            ui->exportButton->setIcon(QIcon());
        } else {
            ui->newAddress->setIcon(platformStyle->SingleColorIcon(":/icons/add"));
            ui->copyAddress->setIcon(platformStyle->SingleColorIcon(":/icons/editcopy"));
            ui->deleteAddress->setIcon(platformStyle->SingleColorIcon(":/icons/remove"));
            ui->exportButton->setIcon(platformStyle->SingleColorIcon(":/icons/export"));
        }

        switch(mode)
        {
        case ForSelection:
            switch(tab)
            {
            case SendingTab: setWindowTitle(tr("Choose the address to send coins to")); break;
            case ReceivingTab: setWindowTitle(tr("Choose the address to receive coins with")); break;
            }
            connect(ui->tableView, &QTableView::doubleClicked, this, &QDialog::accept);
            ui->tableView->setEditTriggers(QAbstractItemView::NoEditTriggers);
            ui->tableView->setFocus();
            ui->closeButton->setText(tr("C&hoose"));
            ui->exportButton->hide();
            break;
        case ForEditing:
            switch(tab)
            {
            case SendingTab: setWindowTitle(tr("Sending addresses")); break;
            case ReceivingTab: setWindowTitle(tr("Receiving addresses")); break;
            }
            break;
        }
        switch(tab)
        {
        case SendingTab:
            ui->labelExplanation->setText(tr("These are your Bitcoin addresses for sending payments. Always check the amount and the receiving address before sending coins."));
            ui->deleteAddress->setVisible(true);
            ui->newAddress->setVisible(true);
            break;
        case ReceivingTab:
            ui->labelExplanation->setText(tr("These are your Bitcoin addresses for receiving payments. Use the 'Create new receiving address' button in the receive tab to create new addresses.\nSigning is only possible with addresses of the type 'legacy'."));
            ui->deleteAddress->setVisible(false);
            ui->newAddress->setVisible(false);
            break;
        }

        // Build context menu
        contextMenu = new QMenu(this);
        contextMenu->addAction(tr("&Copy Address"), this, &AddressBookPage::on_copyAddress_clicked);
        contextMenu->addAction(tr("Copy &Label"), this, &AddressBookPage::onCopyLabelAction);
        contextMenu->addAction(tr("&Edit"), this, &AddressBookPage::onEditAction);

        if (tab == SendingTab) {
            contextMenu->addAction(tr("&Delete"), this, &AddressBookPage::on_deleteAddress_clicked);
        }

        connect(ui->tableView, &QWidget::customContextMenuRequested, this, &AddressBookPage::contextualMenu);
        connect(ui->closeButton, &QPushButton::clicked, this, &QDialog::accept);

        gui_util::handleCloseWindowShortcut(this);
        */
    }
    
    pub fn set_model(&mut self, model: *mut AddressTableModel)  {
        
        todo!();
        /*
            this->model = _model;
        if(!_model)
            return;

        auto type = tab == ReceivingTab ? AddressTableModel::Receive : AddressTableModel::Send;
        proxyModel = new AddressBookSortFilterProxyModel(type, this);
        proxyModel->setSourceModel(_model);

        connect(ui->searchLineEdit, &QLineEdit::textChanged, proxyModel, &QSortFilterProxyModel::setFilterWildcard);

        ui->tableView->setModel(proxyModel);
        ui->tableView->sortByColumn(0, QtAscendingOrder);

        // Set column widths
        ui->tableView->horizontalHeader()->setSectionResizeMode(AddressTableModel::Label, QHeaderView::Stretch);
        ui->tableView->horizontalHeader()->setSectionResizeMode(AddressTableModel::Address, QHeaderView::ResizeToContents);

        connect(ui->tableView->selectionModel(), &QItemSelectionModel::selectionChanged,
            this, &AddressBookPage::selectionChanged);

        // Select row for newly created address
        connect(_model, &AddressTableModel::rowsInserted, this, &AddressBookPage::selectNewAddress);

        selectionChanged();
        */
    }
    
    /**
      | Copy address of currently selected
      | address entry to clipboard
      |
      */
    #[Q_SLOT]
    pub fn on_copy_address_clicked(&mut self)  {
        
        todo!();
        /*
            gui_util::copyEntryData(ui->tableView, AddressTableModel::Address);
        */
    }
    
    /**
      | Copy label of currently selected address
      | entry to clipboard (no button)
      |
      */
    #[Q_SLOT]
    pub fn on_copy_label_action(&mut self)  {
        
        todo!();
        /*
            gui_util::copyEntryData(ui->tableView, AddressTableModel::Label);
        */
    }
    
    /**
      | Edit currently selected address entry
      | (no button)
      |
      */
    #[Q_SLOT]
    pub fn on_edit_action(&mut self)  {
        
        todo!();
        /*
            if(!model)
            return;

        if(!ui->tableView->selectionModel())
            return;
        QModelIndexList indexes = ui->tableView->selectionModel()->selectedRows();
        if(indexes.isEmpty())
            return;

        auto dlg = new EditAddressDialog(
            tab == SendingTab ?
            EditAddressDialog::EditSendingAddress :
            EditAddressDialog::EditReceivingAddress, this);
        dlg->setModel(model);
        QModelIndex origIndex = proxyModel->mapToSource(indexes.at(0));
        dlg->loadRow(origIndex.row());
        gui_util::ShowModalDialogAndDeleteOnClose(dlg);
        */
    }
    
    /**
      | Create a new address for receiving coins
      | and / or add a new address book entry
      |
      */
    #[Q_SLOT]
    pub fn on_new_address_clicked(&mut self)  {
        
        todo!();
        /*
            if(!model)
            return;

        if (tab == ReceivingTab) {
            return;
        }

        EditAddressDialog dlg(EditAddressDialog::NewSendingAddress, this);
        dlg.setModel(model);
        if(dlg.exec())
        {
            newAddressToSelect = dlg.getAddress();
        }
        */
    }
    
    /**
      | Delete currently selected address
      | entry
      |
      */
    #[Q_SLOT]
    pub fn on_delete_address_clicked(&mut self)  {
        
        todo!();
        /*
            QTableView *table = ui->tableView;
        if(!table->selectionModel())
            return;

        QModelIndexList indexes = table->selectionModel()->selectedRows();
        if(!indexes.isEmpty())
        {
            table->model()->removeRow(indexes.at(0).row());
        }
        */
    }
    
    /**
      | Set button states based on selected
      | tab and selection
      |
      */
    #[Q_SLOT]
    pub fn selection_changed(&mut self)  {
        
        todo!();
        /*
            // Set button states based on selected tab and selection
        QTableView *table = ui->tableView;
        if(!table->selectionModel())
            return;

        if(table->selectionModel()->hasSelection())
        {
            switch(tab)
            {
            case SendingTab:
                // In sending tab, allow deletion of selection
                ui->deleteAddress->setEnabled(true);
                ui->deleteAddress->setVisible(true);
                break;
            case ReceivingTab:
                // Deleting receiving addresses, however, is not allowed
                ui->deleteAddress->setEnabled(false);
                ui->deleteAddress->setVisible(false);
                break;
            }
            ui->copyAddress->setEnabled(true);
        }
        else
        {
            ui->deleteAddress->setEnabled(false);
            ui->copyAddress->setEnabled(false);
        }
        */
    }
    
    #[Q_SLOT]
    pub fn done(&mut self, retval: i32)  {
        
        todo!();
        /*
            QTableView *table = ui->tableView;
        if(!table->selectionModel() || !table->model())
            return;

        // Figure out which address was selected, and return it
        QModelIndexList indexes = table->selectionModel()->selectedRows(AddressTableModel::Address);

        for (const QModelIndex& index : indexes) {
            QVariant address = table->model()->data(index);
            returnValue = address.toString();
        }

        if(returnValue.isEmpty())
        {
            // If no address entry selected, return rejected
            retval = Rejected;
        }

        QDialog::done(retval);
        */
    }
    
    /**
      | Export button clicked
      |
      */
    #[Q_SLOT]
    pub fn on_export_button_clicked(&mut self)  {
        
        todo!();
        /*
            // CSV is currently the only supported format
        QString filename = gui_util::getSaveFileName(this,
            tr("Export Address List"), QString(),
            /*: Expanded name of the CSV file format.
                See: https://en.wikipedia.org/wiki/Comma-separated_values. */
            tr("Comma separated file") + QLatin1String(" (*.csv)"), nullptr);

        if (filename.isNull())
            return;

        CSVModelWriter writer(filename);

        // name, column, role
        writer.setModel(proxyModel);
        writer.addColumn("Label", AddressTableModel::Label, QtEditRole);
        writer.addColumn("Address", AddressTableModel::Address, QtEditRole);

        if(!writer.write()) {
            QMessageBox::critical(this, tr("Exporting Failed"),
                /*: An error message. %1 is a stand-in argument for the name
                    of the file we attempted to save to. */
                tr("There was an error trying to save the address list to %1. Please try again.").arg(filename));
        }
        */
    }
    
    /**
      | Spawn contextual menu (right mouse
      | menu) for address book entry
      |
      */
    #[Q_SLOT]
    pub fn contextual_menu(&mut self, point: &QPoint)  {
        
        todo!();
        /*
            QModelIndex index = ui->tableView->indexAt(point);
        if(index.isValid())
        {
            contextMenu->exec(QCursor::pos());
        }
        */
    }
    
    /**
      | New entry/entries were added to address
      | table
      |
      */
    #[Q_SLOT]
    pub fn select_new_address(&mut self, 
        parent: &QModelIndex,
        begin:  i32,
        end:    i32)  {
        
        todo!();
        /*
            QModelIndex idx = proxyModel->mapFromSource(model->index(begin, AddressTableModel::Address, parent));
        if(idx.isValid() && (idx.data(QtEditRole).toString() == newAddressToSelect))
        {
            // Select row of newly created address, once
            ui->tableView->setFocus();
            ui->tableView->selectRow(idx.row());
            newAddressToSelect.clear();
        }
        */
    }
}
