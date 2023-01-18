crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/bitcoinamountfield.h]

/**
  | Widget for entering bitcoin amounts.
  |
  */
// ugly hack: for some unknown reason CAmount
//
// (instead of i64) does not work here as expected
//
// discussion:
// https://github.com/bitcoin/bitcoin/pull/5117
#[Q_PROPERTY(i64 value READ value WRITE setValue NOTIFY valueChanged USER true)]
#[Q_OBJECT]
pub struct BitcoinAmountField {
    base:   QWidget,
    amount: *mut AmountSpinBox,
    unit:   *mut QValueComboBox,
}

impl BitcoinAmountField {

    #[Q_SIGNALS]
    pub fn value_changed(&mut self)  { }

    pub fn new(parent: *mut QWidget) -> Self {
    
        todo!();
        /*
        : q_widget(parent),
        : amount(nullptr),

            amount = new AmountSpinBox(this);
        amount->setLocale(QLocale::c());
        amount->installEventFilter(this);
        amount->setMaximumWidth(240);

        QHBoxLayout *layout = new QHBoxLayout(this);
        layout->addWidget(amount);
        unit = new QValueComboBox(this);
        unit->setModel(new BitcoinUnits(this));
        layout->addWidget(unit);
        layout->addStretch(1);
        layout->setContentsMargins(0,0,0,0);

        setLayout(layout);

        setFocusPolicy(QtTabFocus);
        setFocusProxy(amount);

        // If one if the widgets changes, the combined content changes as well
        connect(amount, &AmountSpinBox::valueChanged, this, &BitcoinAmountField::valueChanged);
        connect(unit, qOverload<int>(&QComboBox::currentIndexChanged), this, &BitcoinAmountField::unitChanged);

        // Set default based on configuration
        unitChanged(unit->currentIndex());
        */
    }
    
    /**
      | Make field empty and ready for new input.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            amount->clear();
        unit->setCurrentIndex(0);
        */
    }
    
    /**
      | Enable/Disable.
      |
      */
    pub fn set_enabled(&mut self, enabled: bool)  {
        
        todo!();
        /*
            amount->setEnabled(fEnabled);
        unit->setEnabled(fEnabled);
        */
    }
    
    /**
      | Perform input validation, mark field
      | as invalid if entered value is not valid.
      |
      */
    pub fn validate(&mut self) -> bool {
        
        todo!();
        /*
            bool valid = false;
        value(&valid);
        setValid(valid);
        return valid;
        */
    }
    
    /**
      | Mark current value as invalid in UI.
      |
      */
    pub fn set_valid(&mut self, valid: bool)  {
        
        todo!();
        /*
            if (valid)
            amount->setStyleSheet("");
        else
            amount->setStyleSheet(STYLE_INVALID);
        */
    }
    
    /**
      | Intercept focus-in event and ',' key
      | presses
      |
      */
    #[Q_SIGNALS]
    pub fn event_filter(&mut self, 
        object: *mut QObject,
        event:  *mut QEvent) -> bool {
        
        todo!();
        /*
            if (event->type() == QEvent::FocusIn)
        {
            // Clear invalid flag on focus
            setValid(true);
        }
        return QWidget::eventFilter(object, event);
        */
    }
    
    /**
      | Qt messes up the tab chain by default
      | in some cases (issue https://bugreports.qt-project.org/browse/QTBUG-10907),
      | in these cases we have to set it up manually.
      |
      */
    pub fn setup_tab_chain(&mut self, prev: *mut QWidget) -> *mut QWidget {
        
        todo!();
        /*
            QWidget::setTabOrder(prev, amount);
        QWidget::setTabOrder(amount, unit);
        return unit;
        */
    }
    
    pub fn value(&self, valid_out: *mut bool) -> Amount {
        
        todo!();
        /*
            return amount->value(valid_out);
        */
    }
    
    pub fn set_value(&mut self, value: &Amount)  {
        
        todo!();
        /*
            amount->setValue(value);
        */
    }
    
    /**
      | If allow empty is set to false the field
      | will be set to the minimum allowed value
      | if left empty. *
      |
      */
    pub fn set_allow_empty(&mut self, allow: bool)  {
        
        todo!();
        /*
            amount->SetAllowEmpty(allow);
        */
    }
    
    /**
      | Set the minimum value in satoshis *
      |
      */
    pub fn set_min_value(&mut self, value: &Amount)  {
        
        todo!();
        /*
            amount->SetMinValue(value);
        */
    }
    
    /**
      | Set the maximum value in satoshis *
      |
      */
    pub fn set_max_value(&mut self, value: &Amount)  {
        
        todo!();
        /*
            amount->SetMaxValue(value);
        */
    }
    
    /**
      | Make read-only *
      |
      */
    pub fn set_read_only(&mut self, read_only: bool)  {
        
        todo!();
        /*
            amount->setReadOnly(fReadOnly);
        */
    }
    
    #[Q_SLOT]
    pub fn unit_changed(&mut self, idx: i32)  {
        
        todo!();
        /*
            // Use description tooltip for current unit for the combobox
        unit->setToolTip(unit->itemData(idx, QtToolTipRole).toString());

        // Determine new unit ID
        int newUnit = unit->itemData(idx, BitcoinUnits::UnitRole).toInt();

        amount->setDisplayUnit(newUnit);
        */
    }
    
    /**
      | Change unit used to display amount.
      |
      */
    pub fn set_display_unit(&mut self, new_unit: i32)  {
        
        todo!();
        /*
            unit->setValue(newUnit);
        */
    }
    
    /**
      | Set single step in satoshis *
      |
      */
    pub fn set_single_step(&mut self, step: &Amount)  {
        
        todo!();
        /*
            amount->setSingleStep(step);
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/qt/bitcoinamountfield.cpp]

/**
  | QSpinBox that uses fixed-point numbers
  | internally and uses our own formatting/parsing
  | functions.
  |
  */
#[Q_OBJECT]
pub struct AmountSpinBox {
    base:                     QAbstractSpinBox,
    current_unit:             i32, //{BitcoinUnits::BTC};
    single_step:              Amount, //{CAmount(100000)}; // satoshis
    cached_minimum_size_hint: RefCell<QSize>,
    allow_empty:              bool, //{true};
    min_amount:               Amount, //{CAmount(0)};
    max_amount:               Amount, //{BitcoinUnits::maxMoney()};
}

impl AmountSpinBox {
    
    pub fn new(parent: *mut QWidget) -> Self {
    
        todo!();
        /*
        : q_abstract_spin_box(parent),

            setAlignment(QtAlignRight);

            connect(lineEdit(), &QLineEdit::textEdited, this, &AmountSpinBox::valueChanged);
        */
    }
    
    pub fn validate(&self, 
        text: &mut String,
        pos:  &mut i32) -> QValidatorState {
        
        todo!();
        /*
            if(text.isEmpty())
                return QValidatorIntermediate;
            bool valid = false;
            parse(text, &valid);
            /* Make sure we return Intermediate so that fixup() is called on defocus */
            return valid ? QValidatorIntermediate : QValidatorInvalid;
        */
    }
    
    pub fn fixup(&self, input: &mut String)  {
        
        todo!();
        /*
            bool valid;
            CAmount val;

            if (input.isEmpty() && !m_allow_empty) {
                valid = true;
                val = m_min_amount;
            } else {
                valid = false;
                val = parse(input, &valid);
            }

            if (valid) {
                val = qBound(m_min_amount, val, m_max_amount);
                input = BitcoinUnits::format(currentUnit, val, false, BitcoinUnits::SeparatorStyle::ALWAYS);
                lineEdit()->setText(input);
            }
        */
    }
    
    pub fn value(&self, valid_out: Option<*mut bool>) -> Amount {

        todo!();
        /*
            return parse(text(), valid_out);
        */
    }
    
    pub fn set_value(&mut self, value: &Amount)  {
        
        todo!();
        /*
            lineEdit()->setText(BitcoinUnits::format(currentUnit, value, false, BitcoinUnits::SeparatorStyle::ALWAYS));
            Q_EMIT valueChanged();
        */
    }
    
    pub fn set_allow_empty(&mut self, allow: bool)  {
        
        todo!();
        /*
            m_allow_empty = allow;
        */
    }
    
    pub fn set_min_value(&mut self, value: &Amount)  {
        
        todo!();
        /*
            m_min_amount = value;
        */
    }
    
    pub fn set_max_value(&mut self, value: &Amount)  {
        
        todo!();
        /*
            m_max_amount = value;
        */
    }
    
    pub fn step_by(&mut self, steps: i32)  {
        
        todo!();
        /*
            bool valid = false;
            CAmount val = value(&valid);
            val = val + steps * singleStep;
            val = qBound(m_min_amount, val, m_max_amount);
            setValue(val);
        */
    }
    
    pub fn set_display_unit(&mut self, unit: i32)  {
        
        todo!();
        /*
            bool valid = false;
            CAmount val = value(&valid);

            currentUnit = unit;
            lineEdit()->setPlaceholderText(BitcoinUnits::format(currentUnit, m_min_amount, false, BitcoinUnits::SeparatorStyle::ALWAYS));
            if(valid)
                setValue(val);
            else
                clear();
        */
    }
    
    pub fn set_single_step(&mut self, step: &Amount)  {
        
        todo!();
        /*
            singleStep = step;
        */
    }
    
    pub fn minimum_size_hint(&self) -> QSize {
        
        todo!();
        /*
            if(cachedMinimumSizeHint.isEmpty())
            {
                ensurePolished();

                const QFontMetrics fm(fontMetrics());
                int h = lineEdit()->minimumSizeHint().height();
                int w = gui_util::TextWidth(fm, BitcoinUnits::format(BitcoinUnits::BTC, BitcoinUnits::maxMoney(), false, BitcoinUnits::SeparatorStyle::ALWAYS));
                w += 2; // cursor blinking space

                QStyleOptionSpinBox opt;
                initStyleOption(&opt);
                QSize hint(w, h);
                QSize extra(35, 6);
                opt.rect.setSize(hint + extra);
                extra += hint - style()->subControlRect(QStyle::CC_SpinBox, &opt,
                                                        QStyle::SC_SpinBoxEditField, this).size();
                // get closer to final result by repeating the calculation
                opt.rect.setSize(hint + extra);
                extra += hint - style()->subControlRect(QStyle::CC_SpinBox, &opt,
                                                        QStyle::SC_SpinBoxEditField, this).size();
                hint += extra;
                hint.setHeight(h);

                opt.rect = rect();

                cachedMinimumSizeHint = style()->sizeFromContents(QStyle::CT_SpinBox, &opt, hint, this)
                                        .expandedTo(QApplication::globalStrut());
            }
            return cachedMinimumSizeHint;
        */
    }

    /**
      | Parse a string into a number of base monetary
      | units and return validity.
      | 
      | -----------
      | @note
      | 
      | Must return 0 if !valid.
      |
      */
    pub fn parse(&self, 
        text:      &String,
        valid_out: Option<*mut bool>) -> Amount {

        todo!();
        /*
            CAmount val = 0;
            bool valid = BitcoinUnits::parse(currentUnit, text, &val);
            if(valid)
            {
                if(val < 0 || val > BitcoinUnits::maxMoney())
                    valid = false;
            }
            if(valid_out)
                *valid_out = valid;
            return valid ? val : 0;
        */
    }
    
    pub fn event(&mut self, event: *mut QEvent) -> bool {
        
        todo!();
        /*
            if (event->type() == QEvent::KeyPress || event->type() == QEvent::KeyRelease)
            {
                QKeyEvent *keyEvent = static_cast<QKeyEvent *>(event);
                if (keyEvent->key() == QtKey_Comma)
                {
                    // Translate a comma into a period
                    QKeyEvent periodKeyEvent(event->type(), QtKey_Period, keyEvent->modifiers(), ".", keyEvent->isAutoRepeat(), keyEvent->count());
                    return QAbstractSpinBox::event(&periodKeyEvent);
                }
            }
            return QAbstractSpinBox::event(event);
        */
    }
    
    pub fn step_enabled(&self) -> StepEnabled {
        
        todo!();
        /*
            if (isReadOnly()) // Disable steps when AmountSpinBox is read-only
                return StepNone;
            if (text().isEmpty()) // Allow step-up with empty field
                return StepUpEnabled;

            StepEnabled rv = StepNone;
            bool valid = false;
            CAmount val = value(&valid);
            if (valid) {
                if (val > m_min_amount)
                    rv |= StepDownEnabled;
                if (val < m_max_amount)
                    rv |= StepUpEnabled;
            }
            return rv;
        */
    }

    #[Q_SIGNAL]
    pub fn value_changed(&mut self)  { }
}
