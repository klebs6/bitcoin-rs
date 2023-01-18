crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/recentrequeststablemodel.h]

pub struct RecentRequestEntry {
    n_version: i32,
    id:        i64,
    date:      QDateTime,
    recipient: SendCoinsRecipient,
}

pub mod recent_request_entry {
    pub const CURRENT_VERSION: i32 = 1;
}

impl Default for RecentRequestEntry {
    
    fn default() -> Self {
        todo!();
        /*
        : n_version(RecentRequestEntry::CURRENT_VERSION),
        : id(0),

        
        */
    }
}

lazy_static!{
    /*
    SERIALIZE_METHODS(RecentRequestEntry, obj) {
            unsigned int date_timet;
            SER_WRITE(obj, date_timet = obj.date.toSecsSinceEpoch());
            READWRITE(obj.nVersion, obj.id, date_timet, obj.recipient);
            SER_READ(obj, obj.date = QDateTime::fromSecsSinceEpoch(date_timet));
        }
    */
}

///---------------------
pub struct RecentRequestEntryLessThan {
    column: i32,
    order:  QtSortOrder,
}

impl RecentRequestEntryLessThan {
    
    pub fn new(
        n_column: i32,
        order:    QtSortOrder) -> Self {
    
        todo!();
        /*
        : column(nColumn),
        : order(fOrder),

        
        */
    }
    
    pub fn invoke(&self, 
        left:  &RecentRequestEntry,
        right: &RecentRequestEntry) -> bool {
        
        todo!();
        /*
            const RecentRequestEntry* pLeft = &left;
        const RecentRequestEntry* pRight = &right;
        if (order == QtDescendingOrder)
            std::swap(pLeft, pRight);

        switch(column)
        {
        case RecentRequestsTableModel::Date:
            return pLeft->date.toSecsSinceEpoch() < pRight->date.toSecsSinceEpoch();
        case RecentRequestsTableModel::Label:
            return pLeft->recipient.label < pRight->recipient.label;
        case RecentRequestsTableModel::Message:
            return pLeft->recipient.message < pRight->recipient.message;
        case RecentRequestsTableModel::Amount:
            return pLeft->recipient.amount < pRight->recipient.amount;
        default:
            return pLeft->id < pRight->id;
        }
        */
    }
}

/**
  | Model for list of recently generated
  | payment requests / bitcoin: URIs.
  | 
  | Part of wallet model.
  |
  */
#[Q_OBJECT]
pub struct RecentRequestsTableModel {
    base:                      QAbstractTableModel,
    wallet_model:              *mut WalletModel,
    columns:                   QStringList,
    list:                      QList<RecentRequestEntry>,
    n_receive_requests_max_id: i64, // default = { 0 }
}

pub mod recent_requests_table_model {

    pub enum ColumnIndex {
        Date     = 0,
        Label    = 1,
        Message  = 2,
        Amount   = 3,
        NUMBER_OF_COLUMNS
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/recentrequeststablemodel.cpp]
impl RecentRequestsTableModel {
    
    pub fn entry(&self, row: i32) -> &RecentRequestEntry {
        
        todo!();
        /*
            return list[row];
        */
    }
    
    pub fn new(parent: *mut WalletModel) -> Self {
    
        todo!();
        /*
        : q_abstract_table_model(parent),
        : wallet_model(parent),

            // Load entries from wallet
        for (const std::string& request : parent->wallet().getAddressReceiveRequests()) {
            addNewRequest(request);
        }

        /* These columns must match the indices in the ColumnIndex enumeration */
        columns << tr("Date") << tr("Label") << tr("Message") << getAmountTitle();

        connect(walletModel->getOptionsModel(), &OptionsModel::displayUnitChanged, this, &RecentRequestsTableModel::updateDisplayUnit);
        */
    }
    
    /**
      | @name Methods overridden from QAbstractTableModel
      |
      */
    pub fn row_count(&self, parent: &QModelIndex) -> i32 {
        
        todo!();
        /*
            if (parent.isValid()) {
            return 0;
        }
        return list.length();
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
    
    pub fn data(&self, 
        index: &QModelIndex,
        role:  i32) -> QVariant {
        
        todo!();
        /*
            if(!index.isValid() || index.row() >= list.length())
            return QVariant();

        if(role == QtDisplayRole || role == QtEditRole)
        {
            const RecentRequestEntry *rec = &list[index.row()];
            switch(index.column())
            {
            case Date:
                return typename gui_util::dateTimeStr(rec->date);
            case Label:
                if(rec->recipient.label.isEmpty() && role == QtDisplayRole)
                {
                    return tr("(no label)");
                }
                else
                {
                    return rec->recipient.label;
                }
            case Message:
                if(rec->recipient.message.isEmpty() && role == QtDisplayRole)
                {
                    return tr("(no message)");
                }
                else
                {
                    return rec->recipient.message;
                }
            case Amount:
                if (rec->recipient.amount == 0 && role == QtDisplayRole)
                    return tr("(no amount requested)");
                else if (role == QtEditRole)
                    return BitcoinUnits::format(walletModel->getOptionsModel()->getDisplayUnit(), rec->recipient.amount, false, BitcoinUnits::SeparatorStyle::NEVER);
                else
                    return BitcoinUnits::format(walletModel->getOptionsModel()->getDisplayUnit(), rec->recipient.amount);
            }
        }
        else if (role == QtTextAlignmentRole)
        {
            if (index.column() == Amount)
                return (int)(QtAlignRight|QtAlignVCenter);
        }
        return QVariant();
        */
    }
    
    pub fn set_data(&mut self, 
        index: &QModelIndex,
        value: &QVariant,
        role:  i32) -> bool {
        
        todo!();
        /*
            return true;
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
            if(role == QtDisplayRole && section < columns.size())
            {
                return columns[section];
            }
        }
        return QVariant();
        */
    }

    /**
      | Updates the column title to "Amount
      | (DisplayUnit)" and emits headerDataChanged()
      | signal for table headers to react.
      |
      */
    pub fn update_amount_column_title(&mut self)  {
        
        todo!();
        /*
            columns[Amount] = getAmountTitle();
        Q_EMIT headerDataChanged(QtHorizontal,Amount,Amount);
        */
    }

    /**
      | Gets title for amount column including
      | current display unit if optionsModel
      | reference available.
      |
      */
    pub fn get_amount_title(&mut self) -> String {
        
        todo!();
        /*
            if (!walletModel->getOptionsModel()) return {};
        return tr("Requested") +
               QLatin1String(" (") +
               BitcoinUnits::shortName(this->walletModel->getOptionsModel()->getDisplayUnit()) +
               QLatin1Char(')');
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

        return createIndex(row, column);
        */
    }
    
    pub fn remove_rows(&mut self, 
        row:    i32,
        count:  i32,
        parent: Option<&QModelIndex>) -> bool {

        let parent: &QModelIndex = unsafe { parent.unwrap_or(&QModelIndex::new()) };
        
        todo!();
        /*
            Q_UNUSED(parent);

        if(count > 0 && row >= 0 && (row+count) <= list.size())
        {
            for (int i = 0; i < count; ++i)
            {
                const RecentRequestEntry* rec = &list[row+i];
                if (!walletModel->wallet().setAddressReceiveRequest(DecodeDestination(rec->recipient.address.toStdString()), ToString(rec->id), ""))
                    return false;
            }

            beginRemoveRows(parent, row, row + count - 1);
            list.erase(list.begin() + row, list.begin() + row + count);
            endRemoveRows();
            return true;
        } else {
            return false;
        }
        */
    }
    
    pub fn flags(&self, index: &QModelIndex) -> QtItemFlags {
        
        todo!();
        /*
            return QtItemIsSelectable | QtItemIsEnabled;
        */
    }

    /**
      | called when adding a request from the
      | GUI
      |
      */
    pub fn add_new_request_with_send_coins_recipient(&mut self, recipient: &SendCoinsRecipient)  {
        
        todo!();
        /*
            RecentRequestEntry newEntry;
        newEntry.id = ++nReceiveRequestsMaxId;
        newEntry.date = QDateTime::currentDateTime();
        newEntry.recipient = recipient;

        DataStream ss(SER_DISK, CLIENT_VERSION);
        ss << newEntry;

        if (!walletModel->wallet().setAddressReceiveRequest(DecodeDestination(recipient.address.toStdString()), ToString(newEntry.id), ss.str()))
            return;

        addNewRequest(newEntry);
        */
    }

    /**
      | called from ctor when loading from wallet
      |
      */
    pub fn add_new_request_with_string(&mut self, recipient: &String)  {
        
        todo!();
        /*
            std::vector<uint8_t> data(recipient.begin(), recipient.end());
        DataStream ss(data, SER_DISK, CLIENT_VERSION);

        RecentRequestEntry entry;
        ss >> entry;

        if (entry.id == 0) // should not happen
            return;

        if (entry.id > nReceiveRequestsMaxId)
            nReceiveRequestsMaxId = entry.id;

        addNewRequest(entry);
        */
    }

    /**
      | actually add to table in GUI
      |
      */
    pub fn add_new_request_with_recent_request_entry(&mut self, recipient: &mut RecentRequestEntry)  {
        
        todo!();
        /*
            beginInsertRows(QModelIndex(), 0, 0);
        list.prepend(recipient);
        endInsertRows();
        */
    }
    
    pub fn sort(&mut self, 
        column: i32,
        order:  Option<QtSortOrder>)  {

        let order: QtSortOrder = order.unwrap_or(QtAscendingOrder);
        
        todo!();
        /*
            std::sort(list.begin(), list.end(), RecentRequestEntryLessThan(column, order));
        Q_EMIT dataChanged(index(0, 0, QModelIndex()), index(list.size() - 1, NUMBER_OF_COLUMNS - 1, QModelIndex()));
        */
    }
    
    #[Q_SLOT]
    pub fn update_display_unit(&mut self)  {
        
        todo!();
        /*
            updateAmountColumnTitle();
        */
    }
}
