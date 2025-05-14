// ---------------- [ File: bitcoin-qt/src/coincontroltreewidget.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/coincontroltreewidget.h]
//-------------------------------------------[.cpp/bitcoin/src/qt/coincontroltreewidget.cpp]

#[Q_OBJECT]
pub struct CoinControlTreeWidget {
    base: QTreeWidget,
}

pub trait KeyPressEvent {
    fn key_press_event(&mut self, event: *mut QKeyEvent);
}

impl KeyPressEvent for CoinControlTreeWidget {

    fn key_press_event(&mut self, event: *mut QKeyEvent)  {
        
        todo!();
        /*
            if (event->key() == QtKey_Space) // press spacebar -> select checkbox
        {
            event->ignore();
            if (this->currentItem()) {
                int COLUMN_CHECKBOX = 0;
                this->currentItem()->setCheckState(COLUMN_CHECKBOX, ((this->currentItem()->checkState(COLUMN_CHECKBOX) == QtChecked) ? QtUnchecked : QtChecked));
            }
        }
        else if (event->key() == QtKey_Escape) // press esc -> close dialog
        {
            event->ignore();
            CoinControlDialog *coinControlDialog = static_cast<CoinControlDialog*>(this->parentWidget());
            coinControlDialog->done(QDialog::Accepted);
        }
        else
        {
            this->QTreeWidget::keyPressEvent(event);
        }
        */
    }
}

impl CoinControlTreeWidget {
    
    pub fn new(parent: *mut QWidget) -> Self {
    
        todo!();
        /*
        : q_tree_widget(parent),

        
        */
    }
}
