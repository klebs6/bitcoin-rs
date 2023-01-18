crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/qt/sendcoinsrecipient.h]

pub struct SendCoinsRecipient {

    /**
      | If from an unauthenticated payment request,
      | this is used for storing the addresses,
      | e.g. address-A<br />address-B<br
      |  />address-C.
      |
      | Info: As we don't need to process addresses
      | in here when using payment requests, we can
      | abuse it for displaying an address list.
      |
      | Todo: This is a hack, should be replaced
      | with a cleaner solution!
      */
    address:                  String,

    label:                    String,
    amount:                   Amount,

    /**
      | If from a payment request, this is used
      | for storing the memo
      |
      */
    message:                  String,

    /**
      | Keep the payment request around as a
      | serialized string to ensure load/store
      | is lossless.
      |
      */
    payment_request:          String,

    /**
      | Empty if no authentication or invalid
      | signature/cert/etc.
      |
      */
    authenticated_merchant:   String,

    /**
      | memory only
      |
      */
    subtract_fee_from_amount: bool,

    n_version:                i32,
}

pub mod send_coins_recipient {

    pub const CURRENT_VERSION: i32 = 1;
}

lazy_static!{
    /*
    SERIALIZE_METHODS(SendCoinsRecipient, obj)
        {
            std::string address_str, label_str, message_str, auth_merchant_str;

            SER_WRITE(obj, address_str = obj.address.toStdString());
            SER_WRITE(obj, label_str = obj.label.toStdString());
            SER_WRITE(obj, message_str = obj.message.toStdString());
            SER_WRITE(obj, auth_merchant_str = obj.authenticatedMerchant.toStdString());

            READWRITE(obj.nVersion, address_str, label_str, obj.amount, message_str, obj.sPaymentRequest, auth_merchant_str);

            SER_READ(obj, obj.address = QString::fromStdString(address_str));
            SER_READ(obj, obj.label = QString::fromStdString(label_str));
            SER_READ(obj, obj.message = QString::fromStdString(message_str));
            SER_READ(obj, obj.authenticatedMerchant = QString::fromStdString(auth_merchant_str));
        }
    */
}

impl Default for SendCoinsRecipient {
    fn default() -> Self {
    
        todo!();
        /*
            : amount(0),
            fSubtractFeeFromAmount(false),
            nVersion(SendCoinsRecipient::CURRENT_VERSION)
        */
    }
}
    
impl SendCoinsRecipient {

    pub fn new(
        addr:    &String,
        label:   &String,
        amount:  &Amount,
        message: &String) -> Self {
    
        todo!();
        /*


            : address(addr), label(_label), amount(_amount), message(_message), fSubtractFeeFromAmount(false), nVersion(SendCoinsRecipient::CURRENT_VERSION)
        */
    }
}
