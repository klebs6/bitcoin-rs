// ---------------- [ File: bitcoin-qt/src/qvalidatedlineedit.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/qvalidatedlineedit.h]

/**
  | Line edit that can be marked as "invalid"
  | to show input validation feedback.
  | When marked as invalid, it will get a
  | red background until it is focused.
  |
  */
#[Q_OBJECT]
pub struct QValidatedLineEdit {
    base:            QLineEdit,
    valid:           bool,
    check_validator: *const QValidator,
}

//-------------------------------------------[.cpp/bitcoin/src/qt/qvalidatedlineedit.cpp]
impl QValidatedLineEdit {
    
    #[Q_SIGNAL]
    pub fn validation_did_change(&mut self, validated_line_edit: *mut QValidatedLineEdit)  { }

    pub fn new(parent: *mut QWidget) -> Self {
    
        todo!();
        /*
        : q_line_edit(parent),
        : valid(true),
        : check_validator(nullptr),

            connect(this, &QValidatedLineEdit::textChanged, this, &QValidatedLineEdit::markValid);
        */
    }
    
    #[Q_SLOT]
    pub fn set_valid(&mut self, valid: bool)  {
        
        todo!();
        /*
            if(_valid == this->valid)
        {
            return;
        }

        if(_valid)
        {
            setStyleSheet("");
        }
        else
        {
            setStyleSheet(STYLE_INVALID);
        }
        this->valid = _valid;
        */
    }
    
    pub fn focus_in_event(&mut self, evt: *mut QFocusEvent)  {
        
        todo!();
        /*
            // Clear invalid flag on focus
        setValid(true);

        QLineEdit::focusInEvent(evt);
        */
    }
    
    pub fn focus_out_event(&mut self, evt: *mut QFocusEvent)  {
        
        todo!();
        /*
            checkValidity();

        QLineEdit::focusOutEvent(evt);
        */
    }
    
    #[Q_SLOT]
    pub fn mark_valid(&mut self)  {
        
        todo!();
        /*
            // As long as a user is typing ensure we display state as valid
        setValid(true);
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            setValid(true);
        QLineEdit::clear();
        */
    }
    
    #[Q_SLOT]
    pub fn set_enabled(&mut self, enabled: bool)  {
        
        todo!();
        /*
            if (!enabled)
        {
            // A disabled QValidatedLineEdit should be marked valid
            setValid(true);
        }
        else
        {
            // Recheck validity when QValidatedLineEdit gets enabled
            checkValidity();
        }

        QLineEdit::setEnabled(enabled);
        */
    }
    
    #[Q_SLOT]
    pub fn check_validity(&mut self)  {
        
        todo!();
        /*
            if (text().isEmpty())
        {
            setValid(true);
        }
        else if (hasAcceptableInput())
        {
            setValid(true);

            // Check contents on focus out
            if (checkValidator)
            {
                QString address = text();
                int pos = 0;
                if (checkValidator->validate(address, pos) == QValidatorAcceptable)
                    setValid(true);
                else
                    setValid(false);
            }
        }
        else
            setValid(false);

        Q_EMIT validationDidChange(this);
        */
    }
    
    pub fn set_check_validator(&mut self, v: *const QValidator)  {
        
        todo!();
        /*
            checkValidator = v;
        */
    }
    
    pub fn is_valid(&mut self) -> bool {
        
        todo!();
        /*
            // use checkValidator in case the QValidatedLineEdit is disabled
        if (checkValidator)
        {
            QString address = text();
            int pos = 0;
            if (checkValidator->validate(address, pos) == QValidatorAcceptable)
                return true;
        }

        return valid;
        */
    }
}
