crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/rpc/external_signer.cpp]

#[cfg(ENABLE_EXTERNAL_SIGNER)]
pub fn enumeratesigners() -> RPCHelpMan {
    
    todo!();
        /*
            return RPCHelpMan{"enumeratesigners",
            "Returns a list of external signers from -signer.",
            {},
            RPCResult{
                RPCResult::Type::OBJ, "", "",
                {
                    {RPCResult::Type::ARR, "signers", /* optional */ false, "",
                    {
                        {RPCResult::Type::OBJ, "", "",
                        {
                            {RPCResult::Type::STR_HEX, "fingerprint", "Master key fingerprint"},
                            {RPCResult::Type::STR, "name", "Device name"},
                        }},
                    },
                    }
                }
            },
            RPCExamples{
                HelpExampleCli("enumeratesigners", "")
                + HelpExampleRpc("enumeratesigners", "")
            },
            [&](const RPCHelpMan& self, const JSONRPCRequest& request) -> UniValue
            {
                const std::string command = gArgs.GetArg("-signer", "");
                if (command == "") throw JSONRPCError(RPC_MISC_ERROR, "Error: restart bitcoind with -signer=<cmd>");
                const std::string chain = gArgs.GetChainName();
                UniValue signers_res = UniValue::VARR;
                try {
                    std::vector<ExternalSigner> signers;
                    ExternalSigner::Enumerate(command, signers, chain);
                    for (const ExternalSigner& signer : signers) {
                        UniValue signer_res = UniValue::VOBJ;
                        signer_res.pushKV("fingerprint", signer.m_fingerprint);
                        signer_res.pushKV("name", signer.m_name);
                        signers_res.push_back(signer_res);
                    }
                } catch (const std::exception& e) {
                    throw JSONRPCError(RPC_MISC_ERROR, e.what());
                }
                UniValue result(UniValue::VOBJ);
                result.pushKV("signers", signers_res);
                return result;
            }
        };
        */
}

#[cfg(ENABLE_EXTERNAL_SIGNER)]
pub fn register_signer_rpc_commands(t: &mut RPCTable)  {
    
    todo!();
        /*
            // clang-format off
    static const CRPCCommand commands[] =
    { // category              actor (function)
      // --------------------- ------------------------
      { "signer",              &enumeratesigners,      },
    };
    // clang-format on
        for (const auto& c : commands) {
            t.appendCommand(c.name, &c);
        }
        */
}
