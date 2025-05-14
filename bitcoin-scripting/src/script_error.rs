// ---------------- [ File: bitcoin-scripting/src/script_error.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/script/script_error.h]

#[repr(u8)]
pub enum ScriptError
{
    OK = 0,
    UNKNOWN_ERROR,
    EVAL_FALSE,
    OP_RETURN,

    /* -------------------- Max sizes  -------------------- */
    SCRIPT_SIZE,
    PUSH_SIZE,
    OP_COUNT,
    STACK_SIZE,
    SIG_COUNT,
    PUBKEY_COUNT,

    /* ------------ Failed verify operations  ------------ */
    VERIFY,
    EQUALVERIFY,
    CHECKMULTISIGVERIFY,
    CHECKSIGVERIFY,
    NUMEQUALVERIFY,

    /* --------- Logical/Format/Canonical errors  --------- */
    BAD_OPCODE,
    DISABLED_OPCODE,
    INVALID_STACK_OPERATION,
    INVALID_ALTSTACK_OPERATION,
    UNBALANCED_CONDITIONAL,

    /* --- CHECKLOCKTIMEVERIFY and CHECKSEQUENCEVERIFY  --- */
    NEGATIVE_LOCKTIME,
    UNSATISFIED_LOCKTIME,

    /* ------------------ Malleability  ------------------ */
    SIG_HASHTYPE,
    SIG_DER,
    MINIMALDATA,
    SIG_PUSHONLY,
    SIG_HIGH_S,
    SIG_NULLDUMMY,
    PUBKEYTYPE,
    CLEANSTACK,
    MINIMALIF,
    SIG_NULLFAIL,

    /* ---------------- softfork safeness  ---------------- */
    DISCOURAGE_UPGRADABLE_NOPS,
    DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM,
    DISCOURAGE_UPGRADABLE_TAPROOT_VERSION,
    DISCOURAGE_OP_SUCCESS,
    DISCOURAGE_UPGRADABLE_PUBKEYTYPE,

    /* --------------- segregated witness  --------------- */
    WITNESS_PROGRAM_WRONG_LENGTH,
    WITNESS_PROGRAM_WITNESS_EMPTY,
    WITNESS_PROGRAM_MISMATCH,
    WITNESS_MALLEATED,
    WITNESS_MALLEATED_P2SH,
    WITNESS_UNEXPECTED,
    WITNESS_PUBKEYTYPE,

    /* --------------------- Taproot  --------------------- */
    SCHNORR_SIG_SIZE,
    SCHNORR_SIG_HASHTYPE,
    SCHNORR_SIG,
    TAPROOT_WRONG_CONTROL_SIZE,
    TAPSCRIPT_VALIDATION_WEIGHT,
    TAPSCRIPT_CHECKMULTISIG,
    TAPSCRIPT_MINIMALIF,

    /* --------------- Constant scriptCode  --------------- */
    OP_CODESEPARATOR,
    SIG_FINDANDDELETE,

    ERROR_COUNT
}

pub const SCRIPT_ERR_LAST: usize = ScriptError::ERROR_COUNT as usize;

//-------------------------------------------[.cpp/bitcoin/src/script/script_error.cpp]

pub fn script_error_string(serror: ScriptError) -> String {
    
    todo!();
        /*
            switch (serror)
        {
            case ScriptError::OK:
                return "No error";
            case ScriptError::EVAL_FALSE:
                return "Script evaluated without error but finished with a false/empty top stack element";
            case ScriptError::VERIFY:
                return "Script failed an OP_VERIFY operation";
            case ScriptError::EQUALVERIFY:
                return "Script failed an OP_EQUALVERIFY operation";
            case ScriptError::CHECKMULTISIGVERIFY:
                return "Script failed an OP_CHECKMULTISIGVERIFY operation";
            case ScriptError::CHECKSIGVERIFY:
                return "Script failed an OP_CHECKSIGVERIFY operation";
            case ScriptError::NUMEQUALVERIFY:
                return "Script failed an OP_NUMEQUALVERIFY operation";
            case ScriptError::SCRIPT_SIZE:
                return "Script is too big";
            case ScriptError::PUSH_SIZE:
                return "Push value size limit exceeded";
            case ScriptError::OP_COUNT:
                return "Operation limit exceeded";
            case ScriptError::STACK_SIZE:
                return "Stack size limit exceeded";
            case ScriptError::SIG_COUNT:
                return "Signature count negative or greater than pubkey count";
            case ScriptError::PUBKEY_COUNT:
                return "Pubkey count negative or limit exceeded";
            case ScriptError::BAD_OPCODE:
                return "Opcode missing or not understood";
            case ScriptError::DISABLED_OPCODE:
                return "Attempted to use a disabled opcode";
            case ScriptError::INVALID_STACK_OPERATION:
                return "Operation not valid with the current stack size";
            case ScriptError::INVALID_ALTSTACK_OPERATION:
                return "Operation not valid with the current altstack size";
            case ScriptError::OP_RETURN:
                return "OP_RETURN was encountered";
            case ScriptError::UNBALANCED_CONDITIONAL:
                return "Invalid OP_IF construction";
            case ScriptError::NEGATIVE_LOCKTIME:
                return "Negative locktime";
            case ScriptError::UNSATISFIED_LOCKTIME:
                return "Locktime requirement not satisfied";
            case ScriptError::SIG_HASHTYPE:
                return "Signature hash type missing or not understood";
            case ScriptError::SIG_DER:
                return "Non-canonical DER signature";
            case ScriptError::MINIMALDATA:
                return "Data push larger than necessary";
            case ScriptError::SIG_PUSHONLY:
                return "Only push operators allowed in signatures";
            case ScriptError::SIG_HIGH_S:
                return "Non-canonical signature: S value is unnecessarily high";
            case ScriptError::SIG_NULLDUMMY:
                return "Dummy CHECKMULTISIG argument must be zero";
            case ScriptError::MINIMALIF:
                return "OP_IF/NOTIF argument must be minimal";
            case ScriptError::SIG_NULLFAIL:
                return "Signature must be zero for failed CHECK(MULTI)SIG operation";
            case ScriptError::DISCOURAGE_UPGRADABLE_NOPS:
                return "NOPx reserved for soft-fork upgrades";
            case ScriptError::DISCOURAGE_UPGRADABLE_WITNESS_PROGRAM:
                return "Witness version reserved for soft-fork upgrades";
            case ScriptError::DISCOURAGE_UPGRADABLE_TAPROOT_VERSION:
                return "Taproot version reserved for soft-fork upgrades";
            case ScriptError::DISCOURAGE_OP_SUCCESS:
                return "OP_SUCCESSx reserved for soft-fork upgrades";
            case ScriptError::DISCOURAGE_UPGRADABLE_PUBKEYTYPE:
                return "Public key version reserved for soft-fork upgrades";
            case ScriptError::PUBKEYTYPE:
                return "Public key is neither compressed or uncompressed";
            case ScriptError::CLEANSTACK:
                return "Stack size must be exactly one after execution";
            case ScriptError::WITNESS_PROGRAM_WRONG_LENGTH:
                return "Witness program has incorrect length";
            case ScriptError::WITNESS_PROGRAM_WITNESS_EMPTY:
                return "Witness program was passed an empty witness";
            case ScriptError::WITNESS_PROGRAM_MISMATCH:
                return "Witness program hash mismatch";
            case ScriptError::WITNESS_MALLEATED:
                return "Witness requires empty scriptSig";
            case ScriptError::WITNESS_MALLEATED_P2SH:
                return "Witness requires only-redeemscript scriptSig";
            case ScriptError::WITNESS_UNEXPECTED:
                return "Witness provided for non-witness script";
            case ScriptError::WITNESS_PUBKEYTYPE:
                return "Using non-compressed keys in segwit";
            case ScriptError::SCHNORR_SIG_SIZE:
                return "Invalid Schnorr signature size";
            case ScriptError::SCHNORR_SIG_HASHTYPE:
                return "Invalid Schnorr signature hash type";
            case ScriptError::SCHNORR_SIG:
                return "Invalid Schnorr signature";
            case ScriptError::TAPROOT_WRONG_CONTROL_SIZE:
                return "Invalid Taproot control block size";
            case ScriptError::TAPSCRIPT_VALIDATION_WEIGHT:
                return "Too much signature validation relative to witness weight";
            case ScriptError::TAPSCRIPT_CHECKMULTISIG:
                return "OP_CHECKMULTISIG(VERIFY) is not available in tapscript";
            case ScriptError::TAPSCRIPT_MINIMALIF:
                return "OP_IF/NOTIF argument must be minimal in tapscript";
            case ScriptError::OP_CODESEPARATOR:
                return "Using OP_CODESEPARATOR in non-witness script";
            case ScriptError::SIG_FINDANDDELETE:
                return "Signature is found in scriptCode";
            case ScriptError::UNKNOWN_ERROR:
            case ScriptError::ERROR_COUNT:
            default: break;
        }
        return "unknown error";
        */
}
