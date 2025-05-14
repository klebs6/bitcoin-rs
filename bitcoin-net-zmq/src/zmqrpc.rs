// ---------------- [ File: bitcoin-net-zmq/src/zmqrpc.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/zmq/zmqrpc.h]
//-------------------------------------------[.cpp/bitcoin/src/zmq/zmqrpc.cpp]

pub fn getzmqnotifications() -> RPCHelpMan {
    
    todo!();
        /*
            return RPCHelpMan{"getzmqnotifications",
                    "\nReturns information about the active ZeroMQ notifications.\n",
                    {},
                    RPCResult{
                        RPCResult::Type::ARR, "", "",
                        {
                            {RPCResult::Type::OBJ, "", "",
                            {
                                {RPCResult::Type::STR, "type", "Type of notification"},
                                {RPCResult::Type::STR, "address", "Address of the publisher"},
                                {RPCResult::Type::NUM, "hwm", "Outbound message high water mark"},
                            }},
                        }
                    },
                    RPCExamples{
                        HelpExampleCli("getzmqnotifications", "")
                + HelpExampleRpc("getzmqnotifications", "")
                    },
            [&](const RPCHelpMan& self, const JSONRPCRequest& request) -> UniValue
    {
        UniValue result(UniValue::VARR);
        if (g_zmq_notification_interface != nullptr) {
            for (const auto* n : g_zmq_notification_interface->GetActiveNotifiers()) {
                UniValue obj(UniValue::VOBJ);
                obj.pushKV("type", n->GetType());
                obj.pushKV("address", n->GetAddress());
                obj.pushKV("hwm", n->GetOutboundMessageHighWaterMark());
                result.push_back(obj);
            }
        }

        return result;
    },
        };
        */
}

lazy_static!{
    /*
    const CRPCCommand commands[] =
    { //  category           actor (function)
      //  -----------------  -----------------------
        { "zmq",             &getzmqnotifications,    },
    };
    */
}

pub fn register_zmqrpc_commands(t: &mut RPCTable)  {
    
    todo!();
        /*
            for (const auto& c : commands) {
            t.appendCommand(c.name, &c);
        }
        */
}

//-------------------------------------------[.cpp/bitcoin/src/zmq/zmqutil.h]
//-------------------------------------------[.cpp/bitcoin/src/zmq/zmqutil.cpp]

pub fn zmq_error(str_: &String)  {
    
    todo!();
        /*
            LogPrint(BCLog::ZMQ, "zmq: Error: %s, msg: %s\n", str, zmq_strerror(errno));
        */
}
