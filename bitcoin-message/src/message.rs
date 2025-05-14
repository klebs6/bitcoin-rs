// ---------------- [ File: bitcoin-message/src/message.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/message.h]

lazy_static!{
    /*
    extern const std::string MESSAGE_MAGIC;
    */
}

/**
  | The result of a signed message verification.
  | 
  | Message verification takes as an input:
  | 
  | - address (with whose private key the
  | message is supposed to have been signed)
  | 
  | - signature
  | 
  | - message
  |
  */
pub enum MessageVerificationResult {

    /**
      | The provided address is invalid.
      |
      */
    ERR_INVALID_ADDRESS,

    /**
      | The provided address is valid but does
      | not refer to a public key.
      |
      */
    ERR_ADDRESS_NO_KEY,

    /**
      | The provided signature couldn't be
      | parsed (maybe invalid base64).
      |
      */
    ERR_MALFORMED_SIGNATURE,

    /**
      | A public key could not be recovered from
      | the provided signature and message.
      |
      */
    ERR_PUBKEY_NOT_RECOVERED,

    /**
      | The message was not signed with the private
      | key of the provided address.
      |
      */
    ERR_NOT_SIGNED,

    /**
      | The message verification was successful.
      |
      */
    OK
}

//-------------------------------------------[.cpp/bitcoin/src/util/message.cpp]

/**
  | Text used to signify that a signed message
  | follows and to prevent inadvertently
  | signing a transaction.
  |
  */
pub const MESSAGE_MAGIC: &'static str = "Bitcoin Signed Message:\n";

/**
  | Verify a signed message.
  | 
  | -----------
  | @param[in] address
  | 
  | Signer's bitcoin address, it must refer
  | to a public key.
  | ----------
  | @param[in] signature
  | 
  | The signature in base64 format.
  | ----------
  | @param[in] message
  | 
  | The message that was signed.
  | 
  | -----------
  | @return
  | 
  | result code
  |
  */
pub fn message_verify(
        address:   &String,
        signature: &String,
        message:   &String) -> MessageVerificationResult {
    
    todo!();
        /*
            TxDestination destination = DecodeDestination(address);
        if (!IsValidDestination(destination)) {
            return MessageVerificationResult::ERR_INVALID_ADDRESS;
        }

        if (std::get_if<PKHash>(&destination) == nullptr) {
            return MessageVerificationResult::ERR_ADDRESS_NO_KEY;
        }

        bool invalid = false;
        std::vector<unsigned char> signature_bytes = DecodeBase64(signature.c_str(), &invalid);
        if (invalid) {
            return MessageVerificationResult::ERR_MALFORMED_SIGNATURE;
        }

        CPubKey pubkey;
        if (!pubkey.RecoverCompact(MessageHash(message), signature_bytes)) {
            return MessageVerificationResult::ERR_PUBKEY_NOT_RECOVERED;
        }

        if (!(TxDestination(PKHash(pubkey)) == destination)) {
            return MessageVerificationResult::ERR_NOT_SIGNED;
        }

        return MessageVerificationResult::OK;
        */
}

/**
  | Sign a message.
  | 
  | -----------
  | @param[in] privkey
  | 
  | Private key to sign with.
  | ----------
  | @param[in] message
  | 
  | The message to sign.
  | ----------
  | @param[out] signature
  | 
  | Signature, base64 encoded, only set
  | if true is returned.
  | 
  | -----------
  | @return
  | 
  | true if signing was successful.
  |
  */
pub fn message_sign(
    privkey:   &Key,
    message:   &String,
    signature: &mut String) -> bool {
    
    todo!();
        /*
            std::vector<unsigned char> signature_bytes;

        if (!privkey.SignCompact(MessageHash(message), signature_bytes)) {
            return false;
        }

        signature = EncodeBase64(signature_bytes);

        return true;
        */
}

/**
  | Hashes a message for signing and verification
  | in a manner that prevents inadvertently
  | signing a transaction.
  |
  */
pub fn message_hash(message: &String) -> u256 {
    
    todo!();
        /*
            CHashWriter hasher(SER_GETHASH, 0);
        hasher << MESSAGE_MAGIC << message;

        return hasher.GetHash();
        */
}
