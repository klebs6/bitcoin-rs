// ---------------- [ File: bitcoin-qt/src/qvaluecombobox.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/qvaluecombobox.h]

/**
  | QComboBox that can be used with QDataWidgetMapper
  | to select ordinal values from a model.
  |
  */
#[Q_OBJECT]
#[Q_PROPERTY(QVariant value READ value WRITE setValue NOTIFY valueChanged USER true)]
pub struct QValueComboBox {
    base: QComboBox,
    role: i32,
}

//-------------------------------------------[.cpp/bitcoin/src/qt/qvaluecombobox.cpp]
impl QValueComboBox {

    #[Q_SIGNAL]
    pub fn value_changed(&mut self)  {
        
        todo!();
        /*
        
        */
    }

    pub fn new(parent: *mut QWidget) -> Self {
    
        todo!();
        /*
        : q_combo_box(parent),
        : role(QtUserRole),

            connect(this, qOverload<int>(&QComboBox::currentIndexChanged), this, &QValueComboBox::handleSelectionChanged);
        */
    }
    
    pub fn value(&self) -> QVariant {
        
        todo!();
        /*
            return itemData(currentIndex(), role);
        */
    }
    
    pub fn set_value(&mut self, value: &QVariant)  {
        
        todo!();
        /*
            setCurrentIndex(findData(value, role));
        */
    }
    
    /**
      | Specify model role to use as ordinal
      | value (defaults to QtUserRole)
      |
      */
    pub fn set_role(&mut self, role: i32)  {
        
        todo!();
        /*
            this->role = _role;
        */
    }
    
    #[Q_SLOT]
    pub fn handle_selection_changed(&mut self, idx: i32)  {
        
        todo!();
        /*
            Q_EMIT valueChanged();
        */
    }
}
