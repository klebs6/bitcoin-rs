crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/transactionoverviewwidget.h]

#[Q_OBJECT]
pub struct TransactionOverviewWidget {
    base: QListView,
}

impl TransactionOverviewWidget {
    
    pub fn new(parent: Option<&mut QWidget>) -> Self {
        todo!();
        /*
        : q_list_view(parent),

        
        */
    }
    
    pub fn size_hint(&self) -> QSize {
        
        todo!();
        /*
            return {sizeHintForColumn(TransactionTableModel::ToAddress), QListView::sizeHint().height()};
        */
    }
    
    pub fn show_event(&mut self, event: *mut QShowEvent)  {
        
        todo!();
        /*
            Q_UNUSED(event);
            QSizePolicy sp = sizePolicy();
            sp.setHorizontalPolicy(QSizePolicy::Minimum);
            setSizePolicy(sp);
        */
    }
}
