// ---------------- [ File: bitcoin-qt/src/addresstablemodel.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/addresstablemodel.h]

/**
  | Qt model of the address book in the core.
  | This allows views to access and modify
  | the address book.
  |
  */
#[Q_OBJECT]
pub struct AddressTableModel {
    base:         QAbstractTableModel,
    wallet_model: *const WalletModel,
    priv_:        *mut AddressTablePriv, // default = nullptr
    columns:      QStringList,
    edit_status:  AddressTableModelEditStatus, // default = OK
}

pub mod address_table_model {

    /**
      | Specifies send address
      |
      */
    pub const SEND:    &'static str = "S";

    /**
      | Specifies receive address
      |
      */
    pub const RECEIVE: &'static str = "R";
}

pub enum AddressTableModelColumnIndex {

    /**
      | User specified label
      |
      */
    Label   = 0,   

    /**
      | Bitcoin address
      |
      */
    Address = 1  
}

#[repr(i32)]
pub enum AddressTableModelRoleIndex {

    /**
      | Type of address (#Send or #Receive)
      |
      */
    TypeRole = USER_ROLE 
}

/**
  | Return status of edit/insert operation
  |
  */
pub enum AddressTableModelEditStatus {

    /**
      | Everything ok
      |
      */
    OK,                     

    /**
      | No changes were made during edit operation
      |
      */
    NO_CHANGES,             

    /**
      | Unparseable address
      |
      */
    INVALID_ADDRESS,        

    /**
      | Address already in address book
      |
      */
    DUPLICATE_ADDRESS,      

    /**
      | Wallet could not be unlocked to create
      | new receiving address
      |
      */
    WALLET_UNLOCK_FAILURE,  

    /**
      | Generating a new public key for a receiving
      | address failed
      |
      */
    KEY_GENERATION_FAILURE,  
}

impl AddressTableModel {

    pub fn get_edit_status(&self) -> AddressTableEntryEditStatus {
        
        todo!();
        /*
            return editStatus;
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/addresstablemodel.cpp]

#[derive(Default)]
pub struct AddressTableEntry {
    ty:      AddressTableEntryType,
    label:   String,
    address: String,
}

pub enum AddressTableEntryType {
    Sending,
    Receiving,
    Hidden /* QSortFilterProxyModel will filter these out */
}

impl Default for AddressTableEntryType {
    fn default() -> Self {
        AddressTableEntryType::Sending
    }
}

impl AddressTableEntry {

    pub fn new(
        ty:      AddressTableEntryType,
        label:   &String,
        address: &String) -> Self {
    
        todo!();
        /*
        : ty(_type),
        : label(_label),
        : address(_address),

        
        */
    }
}

///-----------------------------
pub struct AddressTableEntryLessThan {

}

impl AddressTableEntryLessThan {

    pub fn invoke<T,S>(&self, 
        a: &T,
        b: &S) -> bool {

        todo!();

        /*
        pub fn invoke(&self, 
            a: &AddressTableEntry,
            b: &AddressTableEntry) -> bool {
            
            todo!();
            /*
                return a.address < b.address;
            */
        }
        
        pub fn invoke(&self, 
            a: &AddressTableEntry,
            b: &String) -> bool {
            
            todo!();
            /*
                return a.address < b;
            */
        }
        
        pub fn invoke(&self, 
            a: &String,
            b: &AddressTableEntry) -> bool {
            
            todo!();
            /*
                return a < b.address;
            */
        }
        */
    }
}

/**
  | Determine address type from address
  | purpose
  |
  */
pub fn translate_transaction_type(
        str_purpose: &String,
        is_mine:     bool) -> AddressTableEntryType {
    
    todo!();
        /*
            AddressTableEntry::Type addressType = AddressTableEntry::Hidden;
        // "refund" addresses aren't shown, and change addresses aren't returned by getAddresses at all.
        if (strPurpose == "send")
            addressType = AddressTableEntry::Sending;
        else if (strPurpose == "receive")
            addressType = AddressTableEntry::Receiving;
        else if (strPurpose == "unknown" || strPurpose == "") // if purpose not set, guess
            addressType = (isMine ? AddressTableEntry::Receiving : AddressTableEntry::Sending);
        return addressType;
        */
}

/**
  | Private implementation
  |
  */
pub struct AddressTablePriv {
    cached_address_table: QList<AddressTableEntry>,
    parent:               *mut AddressTableModel,
}

impl AddressTablePriv {

    pub fn new(parent: *mut AddressTableModel) -> Self {
    
        todo!();
        /*
        : parent(_parent),

        
        */
    }
    
    pub fn refresh_address_table(&mut self, 
        wallet:       Rc<RefCell<dyn WalletInterface>>,
        pk_hash_only: Option<bool>)  
    {
        let pk_hash_only: bool = pk_hash_only.unwrap_or(false);

        todo!();
        /*
            cachedAddressTable.clear();
            {
                for (const auto& address : wallet.getAddresses())
                {
                    if (pk_hash_only && !std::holds_alternative<PKHash>(address.dest)) {
                        continue;
                    }
                    AddressTableEntry::Type addressType = translateTransactionType(
                            QString::fromStdString(address.purpose), address.is_mine);
                    cachedAddressTable.append(AddressTableEntry(addressType,
                                      QString::fromStdString(address.name),
                                      QString::fromStdString(EncodeDestination(address.dest))));
                }
            }
            // std::lower_bound() and std::upper_bound() require our cachedAddressTable list to be sorted in asc order
            // Even though the map is already sorted this re-sorting step is needed because the originating map
            // is sorted by binary address, not by base58() address.
            std::sort(cachedAddressTable.begin(), cachedAddressTable.end(), AddressTableEntryLessThan());
        */
    }
    
    pub fn update_entry(&mut self, 
        address: &String,
        label:   &String,
        is_mine: bool,
        purpose: &String,
        status:  i32) 
    {
        todo!();

        /*
            // Find address / label in model
            QList<AddressTableEntry>::iterator lower = std::lower_bound(
                cachedAddressTable.begin(), cachedAddressTable.end(), address, AddressTableEntryLessThan());
            QList<AddressTableEntry>::iterator upper = std::upper_bound(
                cachedAddressTable.begin(), cachedAddressTable.end(), address, AddressTableEntryLessThan());
            int lowerIndex = (lower - cachedAddressTable.begin());
            int upperIndex = (upper - cachedAddressTable.begin());
            bool inModel = (lower != upper);
            AddressTableEntry::Type newEntryType = translateTransactionType(purpose, isMine);

            switch(status)
            {
            case CT_NEW:
                if(inModel)
                {
                    qWarning() << "AddressTablePriv::updateEntry: Warning: Got CT_NEW, but entry is already in model";
                    break;
                }
                parent->beginInsertRows(QModelIndex(), lowerIndex, lowerIndex);
                cachedAddressTable.insert(lowerIndex, AddressTableEntry(newEntryType, label, address));
                parent->endInsertRows();
                break;
            case CT_UPDATED:
                if(!inModel)
                {
                    qWarning() << "AddressTablePriv::updateEntry: Warning: Got CT_UPDATED, but entry is not in model";
                    break;
                }
                lower->type = newEntryType;
                lower->label = label;
                parent->emitDataChanged(lowerIndex);
                break;
            case CT_DELETED:
                if(!inModel)
                {
                    qWarning() << "AddressTablePriv::updateEntry: Warning: Got CT_DELETED, but entry is not in model";
                    break;
                }
                parent->beginRemoveRows(QModelIndex(), lowerIndex, upperIndex-1);
                cachedAddressTable.erase(lower, upper);
                parent->endRemoveRows();
                break;
            }
        */
    }
    
    pub fn size(&mut self) -> i32 {
        
        todo!();
        /*
            return cachedAddressTable.size();
        */
    }
    
    pub fn index(&mut self, idx: i32) -> *mut AddressTableEntry {
        
        todo!();
        /*
            if(idx >= 0 && idx < cachedAddressTable.size())
            {
                return &cachedAddressTable[idx];
            }
            else
            {
                return nullptr;
            }
        */
    }
}

///---------------------------
impl Drop for AddressTableModel {
    fn drop(&mut self) {
        todo!();
        /*
            delete priv;
        */
    }
}

impl AddressTableModel {

    pub fn new(
        parent:       *mut WalletModel,
        pk_hash_only: Option<bool>) -> Self {

        let pk_hash_only: bool = pk_hash_only.unwrap_or(false);
    
        todo!();
        /*
        : q_abstract_table_model(parent),
        : wallet_model(parent),

            columns << tr("Label") << tr("Address");
        priv = new AddressTablePriv(this);
        priv->refreshAddressTable(parent->wallet(), pk_hash_only);
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
    
    pub fn data(&self, 
        index: &QModelIndex,
        role:  i32) -> QVariant {
        
        todo!();
        /*
            if(!index.isValid())
            return QVariant();

        AddressTableEntry *rec = static_cast<AddressTableEntry*>(index.internalPointer());

        const auto column = static_cast<ColumnIndex>(index.column());
        if (role == QtDisplayRole || role == QtEditRole) {
            switch (column) {
            case Label:
                if (rec->label.isEmpty() && role == QtDisplayRole) {
                    return tr("(no label)");
                } else {
                    return rec->label;
                }
            case Address:
                return rec->address;
            } // no default case, so the compiler can warn about missing cases
            assert(false);
        } else if (role == QtFontRole) {
            switch (column) {
            case Label:
                return QFont();
            case Address:
                return gui_util::fixedPitchFont();
            } // no default case, so the compiler can warn about missing cases
            assert(false);
        } else if (role == TypeRole) {
            switch(rec->type)
            {
            case AddressTableEntry::Sending:
                return Send;
            case AddressTableEntry::Receiving:
                return Receive;
            case AddressTableEntry::Hidden:
                return {};
            } // no default case, so the compiler can warn about missing cases
            assert(false);
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
            if(!index.isValid())
            return false;
        AddressTableEntry *rec = static_cast<AddressTableEntry*>(index.internalPointer());
        std::string strPurpose = (rec->type == AddressTableEntry::Sending ? "send" : "receive");
        editStatus = OK;

        if(role == QtEditRole)
        {
            TxDestination curAddress = DecodeDestination(rec->address.toStdString());
            if(index.column() == Label)
            {
                // Do nothing, if old label == new label
                if(rec->label == value.toString())
                {
                    editStatus = NO_CHANGES;
                    return false;
                }
                walletModel->wallet().setAddressBook(curAddress, value.toString().toStdString(), strPurpose);
            } else if(index.column() == Address) {
                TxDestination newAddress = DecodeDestination(value.toString().toStdString());
                // Refuse to set invalid address, set error status and return false
                if(std::get_if<CNoDestination>(&newAddress))
                {
                    editStatus = INVALID_ADDRESS;
                    return false;
                }
                // Do nothing, if old address == new address
                else if(newAddress == curAddress)
                {
                    editStatus = NO_CHANGES;
                    return false;
                }
                // Check for duplicate addresses to prevent accidental deletion of addresses, if you try
                // to paste an existing address over another address (with a different label)
                if (walletModel->wallet().getAddress(
                        newAddress, /* name= */ nullptr, /* is_mine= */ nullptr, /* purpose= */ nullptr))
                {
                    editStatus = DUPLICATE_ADDRESS;
                    return false;
                }
                // Double-check that we're not overwriting a receiving address
                else if(rec->type == AddressTableEntry::Sending)
                {
                    // Remove old entry
                    walletModel->wallet().delAddressBook(curAddress);
                    // Add new entry with new address
                    walletModel->wallet().setAddressBook(newAddress, value.toString().toStdString(), strPurpose);
                }
            }
            return true;
        }
        return false;
        */
    }
    
    pub fn header_data(&self, 
        section:     i32,
        orientation: QOrientation,
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
    
    pub fn flags(&self, index: &QModelIndex) -> QItemFlags {
        
        todo!();
        /*
            if (!index.isValid()) return QtNoItemFlags;

        AddressTableEntry *rec = static_cast<AddressTableEntry*>(index.internalPointer());

        QtItemFlags retval = QtItemIsSelectable | QtItemIsEnabled;
        // Can edit address and label for sending addresses,
        // and only label for receiving addresses.
        if(rec->type == AddressTableEntry::Sending ||
          (rec->type == AddressTableEntry::Receiving && index.column()==Label))
        {
            retval |= QtItemIsEditable;
        }
        return retval;
        */
    }
    
    pub fn index(&self, 
        row:    i32,
        column: i32,
        parent: &QModelIndex) -> QModelIndex {
        
        todo!();
        /*
            Q_UNUSED(parent);
        AddressTableEntry *data = priv->index(row);
        if(data)
        {
            return createIndex(row, column, priv->index(row));
        }
        else
        {
            return QModelIndex();
        }
        */
    }
    
    /**
      | Update address list from core.
      |
      */
    #[Q_SLOT]
    pub fn update_entry(&mut self, 
        address: &String,
        label:   &String,
        is_mine: bool,
        purpose: &String,
        status:  i32)  {
        
        todo!();
        /*
            // Update address book model from Bitcoin core
        priv->updateEntry(address, label, isMine, purpose, status);
        */
    }
    
    /**
      | Add an address to the model.
      | 
      | Returns the added address on success,
      | and an empty string otherwise.
      |
      */
    pub fn add_row(&mut self, 
        ty:           &String,
        label:        &String,
        address:      &String,
        address_type: OutputType) -> String {
        
        todo!();
        /*
            std::string strLabel = label.toStdString();
        std::string strAddress = address.toStdString();

        editStatus = OK;

        if(type == Send)
        {
            if(!walletModel->validateAddress(address))
            {
                editStatus = INVALID_ADDRESS;
                return QString();
            }
            // Check for duplicate addresses
            {
                if (walletModel->wallet().getAddress(
                        DecodeDestination(strAddress), /* name= */ nullptr, /* is_mine= */ nullptr, /* purpose= */ nullptr))
                {
                    editStatus = DUPLICATE_ADDRESS;
                    return QString();
                }
            }

            // Add entry
            walletModel->wallet().setAddressBook(DecodeDestination(strAddress), strLabel, "send");
        }
        else if(type == Receive)
        {
            // Generate a new address to associate with given label
            TxDestination dest;
            if(!walletModel->wallet().getNewDestination(address_type, strLabel, dest))
            {
                WalletModel::UnlockContext ctx(walletModel->requestUnlock());
                if(!ctx.isValid())
                {
                    // Unlock wallet failed or was cancelled
                    editStatus = WALLET_UNLOCK_FAILURE;
                    return QString();
                }
                if(!walletModel->wallet().getNewDestination(address_type, strLabel, dest))
                {
                    editStatus = KEY_GENERATION_FAILURE;
                    return QString();
                }
            }
            strAddress = EncodeDestination(dest);
        }
        else
        {
            return QString();
        }
        return QString::fromStdString(strAddress);
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
        AddressTableEntry *rec = priv->index(row);
        if(count != 1 || !rec || rec->type == AddressTableEntry::Receiving)
        {
            // Can only remove one row at a time, and cannot remove rows not in model.
            // Also refuse to remove receiving addresses.
            return false;
        }
        walletModel->wallet().delAddressBook(DecodeDestination(rec->address.toStdString()));
        return true;
        */
    }
    
    /**
      | Look up label for address in address
      | book, if not found return empty string.
      |
      */
    pub fn label_for_address(&self, address: &String) -> String {
        
        todo!();
        /*
            std::string name;
        if (getAddressData(address, &name, /* purpose= */ nullptr)) {
            return QString::fromStdString(name);
        }
        return QString();
        */
    }
    
    /**
      | Look up purpose for address in address
      | book, if not found return empty string.
      |
      */
    pub fn purpose_for_address(&self, address: &String) -> String {
        
        todo!();
        /*
            std::string purpose;
        if (getAddressData(address, /* name= */ nullptr, &purpose)) {
            return QString::fromStdString(purpose);
        }
        return QString();
        */
    }
    
    /**
      | Look up address book data given an address
      | string.
      |
      */
    pub fn get_address_data(&self, 
        address: &String,
        name:    *mut String,
        purpose: *mut String) -> bool {
        
        todo!();
        /*
            TxDestination destination = DecodeDestination(address.toStdString());
        return walletModel->wallet().getAddress(destination, name, /* is_mine= */ nullptr, purpose);
        */
    }
    
    /**
      | Look up row index of an address in the
      | model.
      | 
      | Return -1 if not found.
      |
      */
    pub fn lookup_address(&self, address: &String) -> i32 {
        
        todo!();
        /*
            QModelIndexList lst = match(index(0, Address, QModelIndex()),
                                    QtEditRole, address, 1, QtMatchExactly);
        if(lst.isEmpty())
        {
            return -1;
        }
        else
        {
            return lst.at(0).row();
        }
        */
    }
    
    pub fn get_default_address_type(&self) -> OutputType {
        
        todo!();
        /*
            return walletModel->wallet().getDefaultAddressType(); }{
        */
    }
    
    /**
      | Notify listeners that data changed.
      |
      */
    pub fn emit_data_changed(&mut self, idx: i32)  {
        
        todo!();
        /*
            Q_EMIT dataChanged(index(idx, 0, QModelIndex()), index(idx, columns.length()-1, QModelIndex()));
        */
    }
}
