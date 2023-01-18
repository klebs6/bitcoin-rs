crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/transactionfilterproxy.h]

/**
  | Filter the transaction list according
  | to pre-specified rules.
  |
  */
#[Q_OBJECT]
pub struct TransactionFilterProxy {
    base:              QSortFilterProxyModel,
    date_from:         Option<QDateTime>,
    date_to:           Option<QDateTime>,
    search_string:     String,
    type_filter:       u32,
    watch_only_filter: transaction_filter_proxy::WatchOnlyFilter,
    min_amount:        Amount,
    limit_rows:        i32,
    show_inactive:     bool,
}

pub mod transaction_filter_proxy {

    /**
      | Type filter bit field (all types)
      |
      */
    pub const ALL_TYPES: u32 = 0xFFFFFFFF;

    pub enum WatchOnlyFilter
    {
        WatchOnlyFilter_All,
        WatchOnlyFilter_Yes,
        WatchOnlyFilter_No
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/transactionfilterproxy.cpp]
impl TransactionFilterProxy {
    
    pub fn ty(ty: i32) -> u32 {
        
        todo!();
        /*
            return 1<<type;
        */
    }

    pub fn new(parent: *mut QObject) -> Self {
    
        todo!();
        /*
            :
        QSortFilterProxyModel(parent),
        m_search_string(),
        typeFilter(ALL_TYPES),
        watchOnlyFilter(WatchOnlyFilter_All),
        minAmount(0),
        limitRows(-1),
        showInactive(true)
        */
    }
    
    pub fn filter_accepts_row(&self, 
        source_row:    i32,
        source_parent: &QModelIndex) -> bool {
        
        todo!();
        /*
            QModelIndex index = sourceModel()->index(sourceRow, 0, sourceParent);

        int status = index.data(TransactionTableModel::StatusRole).toInt();
        if (!showInactive && status == TransactionStatus::Conflicted)
            return false;

        int type = index.data(TransactionTableModel::TypeRole).toInt();
        if (!(TYPE(type) & typeFilter))
            return false;

        bool involvesWatchAddress = index.data(TransactionTableModel::WatchonlyRole).toBool();
        if (involvesWatchAddress && watchOnlyFilter == WatchOnlyFilter_No)
            return false;
        if (!involvesWatchAddress && watchOnlyFilter == WatchOnlyFilter_Yes)
            return false;

        QDateTime datetime = index.data(TransactionTableModel::DateRole).toDateTime();
        if (dateFrom && datetime < *dateFrom) return false;
        if (dateTo && datetime > *dateTo) return false;

        QString address = index.data(TransactionTableModel::AddressRole).toString();
        QString label = index.data(TransactionTableModel::LabelRole).toString();
        QString txid = index.data(TransactionTableModel::TxHashRole).toString();
        if (!address.contains(m_search_string, QtCaseInsensitive) &&
            !  label.contains(m_search_string, QtCaseInsensitive) &&
            !   txid.contains(m_search_string, QtCaseInsensitive)) {
            return false;
        }

        i64 amount = llabs(index.data(TransactionTableModel::AmountRole).toLongLong());
        if (amount < minAmount)
            return false;

        return true;
        */
    }
    
    /**
      | Filter transactions between date range.
      | Use std::nullopt for open range.
      |
      */
    pub fn set_date_range(&mut self, 
        from: &Option<QDateTime>,
        to:   &Option<QDateTime>)  {
        
        todo!();
        /*
            dateFrom = from;
        dateTo = to;
        invalidateFilter();
        */
    }
    
    pub fn set_search_string(&mut self, search_string: &String)  {
        
        todo!();
        /*
            if (m_search_string == search_string) return;
        m_search_string = search_string;
        invalidateFilter();
        */
    }
    
    /**
      | @note
      | 
      | Type filter takes a bit field created
      | with TYPE() or ALL_TYPES
      |
      */
    pub fn set_type_filter(&mut self, modes: u32)  {
        
        todo!();
        /*
            this->typeFilter = modes;
        invalidateFilter();
        */
    }
    
    pub fn set_min_amount(&mut self, minimum: &Amount)  {
        
        todo!();
        /*
            this->minAmount = minimum;
        invalidateFilter();
        */
    }
    
    pub fn set_watch_only_filter(&mut self, filter: transaction_filter_proxy::WatchOnlyFilter)  {
        
        todo!();
        /*
            this->watchOnlyFilter = filter;
        invalidateFilter();
        */
    }
    
    /**
      | Set maximum number of rows returned,
      | -1 if unlimited.
      |
      */
    pub fn set_limit(&mut self, limit: i32)  {
        
        todo!();
        /*
            this->limitRows = limit;
        */
    }
    
    /**
      | Set whether to show conflicted transactions.
      |
      */
    pub fn set_show_inactive(&mut self, show_inactive: bool)  {
        
        todo!();
        /*
            this->showInactive = _showInactive;
        invalidateFilter();
        */
    }
    
    pub fn row_count(&self, parent: Option<&QModelIndex>) -> i32 {

        let parent: &QModelIndex = unsafe { parent.unwrap_or(&QModelIndex::new()) };
        
        todo!();
        /*
            if(limitRows != -1)
        {
            return std::min(QSortFilterProxyModel::rowCount(parent), limitRows);
        }
        else
        {
            return QSortFilterProxyModel::rowCount(parent);
        }
        */
    }
}
