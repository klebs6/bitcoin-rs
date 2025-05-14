// ---------------- [ File: bitcoin-qt/src/bitcoinunits.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/bitcoinunits.h]

/**
  | U+2009 THIN SPACE = UTF-8 E2 80 89
  |
  */
pub const REAL_THIN_SP_CP:          usize = 0x2009;
pub const REAL_THIN_SP_UTF8: &[u8] = &[0xE2, 0x80, 0x89];

/**
  | QMessageBox seems to have a bug whereby it
  | doesn't display thin/hair spaces correctly.
  | Workaround is to display a space in a small
  | font.  If you change this, please test that it
  | doesn't cause the parent span to start
  | wrapping.
  */
pub const HTML_HACK_SP: &'static str = "<span style='white-space: nowrap; font-size: 6pt'> </span>";

/**
   Define THIN_SP_* variables to be our preferred
   type of thin space
  */
macro_rules! thin_sp_cp {
    () => {
        /*
                REAL_THIN_SP_CP
        */
    }
}

macro_rules! thin_sp_utf8 {
    () => {
        /*
                REAL_THIN_SP_UTF8
        */
    }
}

macro_rules! thin_sp_html {
    () => {
        /*
                HTML_HACK_SP
        */
    }
}

/**
  | Bitcoin unit definitions. Encapsulates
  | parsing and formatting and serves as
  | list model for drop-down selection
  | boxes.
  |
  */
#[Q_OBJECT]
pub struct BitcoinUnits {
    base:     QAbstractListModel,
    unitlist: QList<bitcoin_units::Unit>,
}

pub mod bitcoin_units {

    use super::*;

    /**
      | @name AbstractListModel implementation
      | 
      | List model for unit drop-down selection
      | box.
      |
      */
    #[repr(i32)]
    pub enum RoleIndex {

        /**
          | Unit identifier
          |
          */
        UnitRole = USER_ROLE
    }

    /**
      | Bitcoin units.
      | 
      | -----------
      | @note
      | 
      | Source: https://en.bitcoin.it/wiki/Units
      | . Please add only sensible ones
      |
      */
    pub enum Unit
    {
        BTC,
        mBTC,
        uBTC,
        SAT
    }

    pub enum SeparatorStyle
    {
        NEVER,
        STANDARD,
        ALWAYS
    }

    /*
      | @name Static API
      | 
      | Unit conversion and formatting
      |
      */

    /**
      | Get list of units, for drop-down box
      |
      */
    pub fn available_units() -> QList<Unit> {
        
        todo!();
        /*
        
        */
    }

    /**
      | Is unit ID valid?
      |
      */
    pub fn valid(unit: i32) -> bool {
        
        todo!();
        /*
        
        */
    }

    /**
      | Long name
      |
      */
    pub fn long_name(unit: i32) -> String {
        
        todo!();
        /*
        
        */
    }

    /**
      | Short name
      |
      */
    pub fn short_name(unit: i32) -> String {
        
        todo!();
        /*
        
        */
    }

    /**
      | Longer description
      |
      */
    pub fn description(unit: i32) -> String {
        
        todo!();
        /*
        
        */
    }

    /**
      | Number of Satoshis (1e-8) per unit
      |
      */
    pub fn factor(unit: i32) -> i64 {
        
        todo!();
        /*
        
        */
    }

    /**
      | Number of decimals left
      |
      */
    pub fn decimals(unit: i32) -> i32 {
        
        todo!();
        /*
        
        */
    }

    /**
      | Format as string
      |
      */
    pub fn format(
        unit:       i32,
        amount:     &Amount,
        plussign:   Option<bool>,
        separators: Option<SeparatorStyle>,
        justify:    Option<bool>) -> String {

        let plussign:             bool = plussign.unwrap_or(false);
        let separators: SeparatorStyle = separators.unwrap_or(SeparatorStyle::STANDARD);
        let justify:              bool = justify.unwrap_or(false);

        todo!();
        /*
        
        */
    }

    /**
      | Format as string (with unit)
      |
      */
    pub fn format_with_unit(
        unit:       i32,
        amount:     &Amount,
        plussign:   Option<bool>,
        separators: Option<SeparatorStyle>) -> String {

        let plussign:             bool = plussign.unwrap_or(false);
        let separators: SeparatorStyle = separators.unwrap_or(SeparatorStyle::STANDARD);

        todo!();
        /*
        
        */
    }

    /**
      | Format as HTML string (with unit)
      |
      */
    pub fn format_html_with_unit(
        unit:       i32,
        amount:     &Amount,
        plussign:   Option<bool>,
        separators: Option<SeparatorStyle>) -> String {

        let plussign:             bool = plussign.unwrap_or(false);
        let separators: SeparatorStyle = separators.unwrap_or(SeparatorStyle::STANDARD);

        todo!();
        /*
        
        */
    }

    /**
      | Format as string (with unit) of fixed
      | length to preserve privacy, if it is
      | set.
      |
      */
    pub fn format_with_privacy(
        unit:       i32,
        amount:     &Amount,
        separators: SeparatorStyle,
        privacy:    bool) -> String {
        
        todo!();
        /*
        
        */
    }

    /**
      | Parse string to coin amount
      |
      */
    pub fn parse(
        unit:    i32,
        value:   &String,
        val_out: *mut Amount) -> bool {
        
        todo!();
        /*
        
        */
    }

    pub fn row_count(parent: &QModelIndex) -> i32 {
        
        todo!();
        /*
        
        */
    }
    
    pub fn data(
        index: &QModelIndex,
        role:  i32) -> QVariant {
        
        todo!();
        /*
        
        */
    }
    
    pub fn remove_spaces(text: String) -> String {
        
        todo!();
        /*
            text.remove(' ');
            text.remove(QChar(THIN_SP_CP));
            return text;
        */
    }

    /**
      | Return maximum number of base units
      | (Satoshis)
      |
      */
    pub fn max_money() -> Amount {
        
        todo!();
        /*
        
        */
    }
}

pub type BitcoinUnit = bitcoin_units::Unit;

//-------------------------------------------[.cpp/bitcoin/src/qt/bitcoinunits.cpp]

pub const MAX_DIGITS_BTC: usize = 16;

impl BitcoinUnits {

    pub fn new(parent: *mut QObject) -> Self {
    
        todo!();
        /*
        : q_abstract_list_model(parent),
        : unitlist(availableUnits()),

        
        */
    }
    
    pub fn available_units(&mut self) -> QList<bitcoin_units::Unit> {
        
        todo!();
        /*
            QList<BitcoinUnits::Unit> unitlist;
        unitlist.append(BTC);
        unitlist.append(mBTC);
        unitlist.append(uBTC);
        unitlist.append(SAT);
        return unitlist;
        */
    }
    
    pub fn valid(&mut self, unit: i32) -> bool {
        
        todo!();
        /*
            switch(unit)
        {
        case BTC:
        case mBTC:
        case uBTC:
        case SAT:
            return true;
        default:
            return false;
        }
        */
    }
    
    pub fn long_name(&mut self, unit: i32) -> String {
        
        todo!();
        /*
            switch(unit)
        {
        case BTC: return QString("BTC");
        case mBTC: return QString("mBTC");
        case uBTC: return QString::fromUtf8("µBTC (bits)");
        case SAT: return QString("Satoshi (sat)");
        default: return QString("???");
        }
        */
    }
    
    pub fn short_name(&mut self, unit: i32) -> String {
        
        todo!();
        /*
            switch(unit)
        {
        case uBTC: return QString::fromUtf8("bits");
        case SAT: return QString("sat");
        default: return longName(unit);
        }
        */
    }
    
    pub fn description(&mut self, unit: i32) -> String {
        
        todo!();
        /*
            switch(unit)
        {
        case BTC: return QString("Bitcoins");
        case mBTC: return QString("Milli-Bitcoins (1 / 1" THIN_SP_UTF8 "000)");
        case uBTC: return QString("Micro-Bitcoins (bits) (1 / 1" THIN_SP_UTF8 "000" THIN_SP_UTF8 "000)");
        case SAT: return QString("Satoshi (sat) (1 / 100" THIN_SP_UTF8 "000" THIN_SP_UTF8 "000)");
        default: return QString("???");
        }
        */
    }
    
    pub fn factor(&mut self, unit: i32) -> i64 {
        
        todo!();
        /*
            switch(unit)
        {
        case BTC: return 100000000;
        case mBTC: return 100000;
        case uBTC: return 100;
        case SAT: return 1;
        default: return 100000000;
        }
        */
    }
    
    pub fn decimals(&mut self, unit: i32) -> i32 {
        
        todo!();
        /*
            switch(unit)
        {
        case BTC: return 8;
        case mBTC: return 5;
        case uBTC: return 2;
        case SAT: return 0;
        default: return 0;
        }
        */
    }
    
    pub fn format(&mut self, 
        unit:       i32,
        n_in:       &Amount,
        plus:       bool,
        separators: bitcoin_units::SeparatorStyle,
        justify:    bool) -> String {
        
        todo!();
        /*
            // Note: not using straight sprintf here because we do NOT want
        // localized number formatting.
        if(!valid(unit))
            return QString(); // Refuse to format invalid unit
        i64 n = (i64)nIn;
        i64 coin = factor(unit);
        int num_decimals = decimals(unit);
        i64 n_abs = (n > 0 ? n : -n);
        i64 quotient = n_abs / coin;
        QString quotient_str = QString::number(quotient);
        if (justify) {
            quotient_str = quotient_str.rightJustified(MAX_DIGITS_BTC - num_decimals, ' ');
        }

        // Use SI-style thin space separators as these are locale independent and can't be
        // confused with the decimal marker.
        QChar thin_sp(THIN_SP_CP);
        int q_size = quotient_str.size();
        if (separators == SeparatorStyle::ALWAYS || (separators == SeparatorStyle::STANDARD && q_size > 4))
            for (int i = 3; i < q_size; i += 3)
                quotient_str.insert(q_size - i, thin_sp);

        if (n < 0)
            quotient_str.insert(0, '-');
        else if (fPlus && n > 0)
            quotient_str.insert(0, '+');

        if (num_decimals > 0) {
            i64 remainder = n_abs % coin;
            QString remainder_str = QString::number(remainder).rightJustified(num_decimals, '0');
            return quotient_str + QString(".") + remainder_str;
        } else {
            return quotient_str;
        }
        */
    }

    /**
      | NOTE: Using formatWithUnit in an HTML context
      | risks wrapping quantities at the thousands
      | separator. More subtly, it also results in
      | a standard space rather than a thin space, due
      | to a bug in Qt's XML whitespace
      | canonicalisation
      |
      | Please take care to use formatHtmlWithUnit
      | instead, when appropriate.
      */
    pub fn format_with_unit(&mut self, 
        unit:       i32,
        amount:     &Amount,
        plussign:   bool,
        separators: bitcoin_units::SeparatorStyle) -> String {
        
        todo!();
        /*
            return format(unit, amount, plussign, separators) + QString(" ") + shortName(unit);
        */
    }
    
    pub fn format_html_with_unit(&mut self, 
        unit:       i32,
        amount:     &Amount,
        plussign:   bool,
        separators: bitcoin_units::SeparatorStyle) -> String {
        
        todo!();
        /*
            QString str(formatWithUnit(unit, amount, plussign, separators));
        str.replace(QChar(THIN_SP_CP), QString(THIN_SP_HTML));
        return QString("<span style='white-space: nowrap;'>%1</span>").arg(str);
        */
    }
    
    pub fn format_with_privacy(&mut self, 
        unit:       i32,
        amount:     &Amount,
        separators: bitcoin_units::SeparatorStyle,
        privacy:    bool) -> String {
        
        todo!();
        /*
            assert(amount >= 0);
        QString value;
        if (privacy) {
            value = format(unit, 0, false, separators, true).replace('0', '#');
        } else {
            value = format(unit, amount, false, separators, true);
        }
        return value + QString(" ") + shortName(unit);
        */
    }
    
    pub fn parse(&mut self, 
        unit:    i32,
        value:   &String,
        val_out: *mut Amount) -> bool {
        
        todo!();
        /*
            if(!valid(unit) || value.isEmpty())
            return false; // Refuse to parse invalid unit or empty string
        int num_decimals = decimals(unit);

        // Ignore spaces and thin spaces when parsing
        QStringList parts = removeSpaces(value).split(".");

        if(parts.size() > 2)
        {
            return false; // More than one dot
        }
        QString whole = parts[0];
        QString decimals;

        if(parts.size() > 1)
        {
            decimals = parts[1];
        }
        if(decimals.size() > num_decimals)
        {
            return false; // Exceeds max precision
        }
        bool ok = false;
        QString str = whole + decimals.leftJustified(num_decimals, '0');

        if(str.size() > 18)
        {
            return false; // Longer numbers will exceed 63 bits
        }
        CAmount retvalue(str.toLongLong(&ok));
        if(val_out)
        {
            *val_out = retvalue;
        }
        return ok;
        */
    }
    
    /**
      | Gets title for amount column including
      | current display unit if optionsModel
      | reference available 
      |
      */
    pub fn get_amount_column_title(&mut self, unit: i32) -> String {
        
        todo!();
        /*
            QString amountTitle = QObject::tr("Amount");
        if (BitcoinUnits::valid(unit))
        {
            amountTitle += " ("+BitcoinUnits::shortName(unit) + ")";
        }
        return amountTitle;
        */
    }
    
    pub fn row_count(&self, parent: &QModelIndex) -> i32 {
        
        todo!();
        /*
            Q_UNUSED(parent);
        return unitlist.size();
        */
    }
    
    pub fn data(&self, 
        index: &QModelIndex,
        role:  i32) -> QVariant {
        
        todo!();
        /*
            int row = index.row();
        if(row >= 0 && row < unitlist.size())
        {
            Unit unit = unitlist.at(row);
            switch(role)
            {
            case QtEditRole:
            case QtDisplayRole:
                return QVariant(longName(unit));
            case QtToolTipRole:
                return QVariant(description(unit));
            case UnitRole:
                return QVariant(static_cast<int>(unit));
            }
        }
        return QVariant();
        */
    }
    
    pub fn max_money(&mut self) -> Amount {
        
        todo!();
        /*
            return MAX_MONEY;
        */
    }
}
