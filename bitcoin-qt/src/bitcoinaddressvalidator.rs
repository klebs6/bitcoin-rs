crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/bitcoinaddressvalidator.h]

/**
  | Base58 entry widget validator, checks
  | for valid characters and removes some
  | whitespace.
  |
  */
#[Q_OBJECT]
pub struct BitcoinAddressEntryValidator {
    base: QValidator,
}

/**
  | Bitcoin address widget validator,
  | checks for a valid bitcoin address.
  |
  */
#[Q_OBJECT]
pub struct BitcoinAddressCheckValidator {
    base: QValidator,
}

//-------------------------------------------[.cpp/bitcoin/src/qt/bitcoinaddressvalidator.cpp]

/**
  | Base58 characters are: "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"
  | 
  | This is:
  | 
  | - All numbers except for '0'
  | 
  | - All upper-case letters except for
  | 'I' and 'O'
  | 
  | - All lower-case letters except for
  | 'l'
  |
  */
impl BitcoinAddressEntryValidator {
    
    pub fn new(parent: *mut QObject) -> Self {
    
        todo!();
        /*
        : q_validator(parent),
        */
    }
    
    fn validate_impl(&self, 
        input: &mut String,
        pos:   &mut i32) -> QValidatorState {
        
        todo!();
        /*
            Q_UNUSED(pos);

        // Empty address is "intermediate" input
        if (input.isEmpty())
            return QValidatorIntermediate;

        // Correction
        for (int idx = 0; idx < input.size();)
        {
            bool removeChar = false;
            QChar ch = input.at(idx);
            // Corrections made are very conservative on purpose, to avoid
            // users unexpectedly getting away with typos that would normally
            // be detected, and thus sending to the wrong address.
            switch(ch.unicode())
            {
            // Qt categorizes these as "Other_Format" not "Separator_Space"
            case 0x200B: // ZERO WIDTH SPACE
            case 0xFEFF: // ZERO WIDTH NO-BREAK SPACE
                removeChar = true;
                break;
            default:
                break;
            }

            // Remove whitespace
            if (ch.isSpace())
                removeChar = true;

            // To next character
            if (removeChar)
                input.remove(idx, 1);
            else
                ++idx;
        }

        // Validation
        QValidatorState state = QValidatorAcceptable;
        for (int idx = 0; idx < input.size(); ++idx)
        {
            int ch = input.at(idx).unicode();

            if (((ch >= '0' && ch<='9') ||
                (ch >= 'a' && ch<='z') ||
                (ch >= 'A' && ch<='Z')) &&
                ch != 'I' && ch != 'O') // Characters invalid in both Base58 and Bech32
            {
                // Alphanumeric and not a 'forbidden' character
            }
            else
            {
                state = QValidatorInvalid;
            }
        }

        return state;
        */
    }
    
    pub fn validate(&self, 
        input: &mut String,
        pos:   &mut i32) -> QValidatorState {
        
        todo!();
        /*
            Q_UNUSED(pos);
        // Validate the passed Bitcoin address
        if (IsValidDestinationString(input.toStdString())) {
            return QValidatorAcceptable;
        }

        return QValidatorInvalid;
        */
    }
}
