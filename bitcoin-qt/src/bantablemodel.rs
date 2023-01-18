crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/bantablemodel.h]

pub struct CombinedBan {
    subnet:    SubNet,
    ban_entry: BanEntry,
}

pub struct BannedNodeLessThan {
    column: i32,
    order:  QSortOrder,
}

/**
  | Qt model providing information about
  | connected peers, similar to the "getpeerinfo"
  | RPC call. Used by the rpc console UI.
  |
  */
#[Q_OBJECT]
pub struct BanTableModel {
    base:    QAbstractTableModel,
    node:    Rc<RefCell<dyn NodeInterface>>,
    columns: QStringList,
    priv_:   Box<BanTablePriv>,
}

pub mod ban_table_model {

    pub enum ColumnIndex {
        Address = 0,
        Bantime = 1
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/bantablemodel.cpp]
impl BanTableModel {

    pub fn new(
        node:   Rc<RefCell<dyn NodeInterface>>,
        parent: *mut QObject) -> Self {
    
        todo!();
        /*


            :
        QAbstractTableModel(parent),
        m_node(node)

        columns << tr("IP/Netmask") << tr("Banned Until");
        priv.reset(new BanTablePriv());

        // load initial data
        refresh();
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

        CCombinedBan *rec = static_cast<CCombinedBan*>(index.internalPointer());

        const auto column = static_cast<ColumnIndex>(index.column());
        if (role == QtDisplayRole) {
            switch (column) {
            case Address:
                return QString::fromStdString(rec->subnet.ToString());
            case Bantime:
                QDateTime date = QDateTime::fromMSecsSinceEpoch(0);
                date = date.addSecs(rec->banEntry.nBanUntil);
                return QLocale::system().toString(date, QLocale::LongFormat);
            } // no default case, so the compiler can warn about missing cases
            assert(false);
        }

        return QVariant();
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

        QtItemFlags retval = QtItemIsSelectable | QtItemIsEnabled;
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
        CCombinedBan *data = priv->index(row);

        if (data)
            return createIndex(row, column, data);
        return QModelIndex();
        */
    }
    
    #[Q_SLOTS]
    pub fn refresh(&mut self)  {
        
        todo!();
        /*
            Q_EMIT layoutAboutToBeChanged();
        priv->refreshBanlist(m_node);
        Q_EMIT layoutChanged();
        */
    }
    
    pub fn sort(&mut self, 
        column: i32,
        order:  QSortOrder)  {
        
        todo!();
        /*
            priv->sortColumn = column;
        priv->sortOrder = order;
        refresh();
        */
    }
    
    pub fn should_show(&mut self) -> bool {
        
        todo!();
        /*
            return priv->size() > 0;
        */
    }
}

impl BannedNodeLessThan {

    pub fn new(
        n_column: i32,
        order:    QSortOrder) -> Self {
    
        todo!();
        /*
        : column(nColumn),
        : order(fOrder),

        
        */
    }

    pub fn invoke(&self, 
        left:  &CombinedBan,
        right: &CombinedBan) -> bool {
        
        todo!();
        /*
            const CCombinedBan* pLeft = &left;
        const CCombinedBan* pRight = &right;

        if (order == QtDescendingOrder)
            std::swap(pLeft, pRight);

        switch (static_cast<BanTableModel::ColumnIndex>(column)) {
        case BanTableModel::Address:
            return pLeft->subnet.ToString().compare(pRight->subnet.ToString()) < 0;
        case BanTableModel::Bantime:
            return pLeft->banEntry.nBanUntil < pRight->banEntry.nBanUntil;
        } // no default case, so the compiler can warn about missing cases
        assert(false);
        */
    }
}

/* ------------ private implementation  ------------ */
pub struct BanTablePriv {

    /**
      | Local cache of peer information
      |
      */
    cached_banlist: QList<CombinedBan>,

    /**
      | Column to sort nodes by (default to unsorted)
      |
      */
    sort_column:    i32, // default = { -1 }

    /**
      | Order (ascending or descending) to
      | sort nodes by
      |
      */
    sort_order:     QSortOrder,
}

impl BanTablePriv {

    /**
      | Pull a full list of banned nodes from
      | Node into our cache
      |
      */
    pub fn refresh_banlist(&mut self, node: Rc<RefCell<dyn NodeInterface>>)  {
        
        todo!();
        /*
            banmap_t banMap;
            node.getBanned(banMap);

            cachedBanlist.clear();
            cachedBanlist.reserve(banMap.size());
            for (const auto& entry : banMap)
            {
                CCombinedBan banEntry;
                banEntry.subnet = entry.first;
                banEntry.banEntry = entry.second;
                cachedBanlist.append(banEntry);
            }

            if (sortColumn >= 0)
                // sort cachedBanlist (use stable sort to prevent rows jumping around unnecessarily)
                std::stable_sort(cachedBanlist.begin(), cachedBanlist.end(), BannedNodeLessThan(sortColumn, sortOrder));
        */
    }
    
    pub fn size(&self) -> i32 {
        
        todo!();
        /*
            return cachedBanlist.size();
        */
    }
    
    pub fn index(&mut self, idx: i32) -> *mut CombinedBan {
        
        todo!();
        /*
            if (idx >= 0 && idx < cachedBanlist.size())
                return &cachedBanlist[idx];

            return nullptr;
        */
    }
}
