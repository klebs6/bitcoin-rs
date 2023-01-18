crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/transactiondescdialog.h]

/**
  | Dialog showing transaction details.
  |
  */
#[Q_OBJECT]
pub struct TransactionDescDialog {
    base: QDialog,
    ui:   *mut UiTransactionDescDialog,
}

impl Drop for TransactionDescDialog {
    fn drop(&mut self) {
        todo!();
        /*
            delete ui;
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/transactiondescdialog.cpp]
impl TransactionDescDialog {
    
    pub fn new(
        idx:    &QModelIndex,
        parent: *mut QWidget) -> Self {
    
        todo!();
        /*


            :
        QDialog(parent, typename gui_util::dialog_flags),
        ui(new UiTransactionDescDialog)

        ui->setupUi(this);
        setWindowTitle(tr("Details for %1").arg(idx.data(TransactionTableModel::TxHashRole).toString()));
        QString desc = idx.data(TransactionTableModel::LongDescriptionRole).toString();
        ui->detailText->setHtml(desc);

        typename gui_util::handleCloseWindowShortcut(this);
        */
    }
}
