// ---------------- [ File: bitcoinrpc-server/src/server.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/rpc/server.h]

pub const DEFAULT_RPC_SERIALIZE_VERSION: u32 = 1;

pub mod rpc_server {

    pub fn on_started(slot: fn() -> ())  {
        
        todo!();
            /*
            
            */
    }

    pub fn on_stopped(slot: fn() -> ())  {
        
        todo!();
            /*
            
            */
    }
}

/**
  | Opaque base class for timers returned
  | by NewTimerFunc.
  | 
  | This provides no methods at the moment,
  | but makes sure that delete cleans up
  | the whole state.
  |
  */
pub trait RPCTimerBase { }

/**
  | RPC timer "driver".
  |
  */
pub trait RPCTimerInterface: GetName {

}

pub trait NewTimer {

    /**
      | Factory function for timers.
      | 
      | RPC will call the function to create
      | a timer that will call func in *millis*
      | milliseconds.
      | 
      | -----------
      | @note
      | 
      | As the RPC mechanism is backend-neutral,
      | it can use different implementations
      | of timers.
      | 
      | This is needed to cope with the case in
      | which there is no HTTP server, but only
      | GUI RPC console, and to break the dependency
      | of pcserver on httprpc.
      |
      */
    fn new_timer(&mut self, 
            func:   &mut fn() -> (),
            millis: i64) -> Rc<RefCell<dyn RPCTimerBase>>;
}

pub type RpcMethodFnType = fn() -> RPCHelpMan;

pub struct RPCCommand {
    pub category:  String,
    pub name:      String,
    pub actor:     rpc_command::Actor,
    pub arg_names: Vec<String>,
    pub unique_id: libc::intptr_t,
}

pub mod rpc_command {

    use super::*;

    /**
      | RPC method handler reading request and
      | assigning result. Should return true if
      | request is fully handled, false if it
      | should be passed on to subsequent
      | handlers.
      */
    pub type Actor = fn(
        request:      &JSONRPCRequest,
        result:       &mut UniValue,
        last_handler: bool
    ) -> bool;
}

impl RPCCommand {

    /**
      | Constructor taking Actor callback
      | supporting multiple handlers.
      |
      */
    pub fn new(
        category:  String,
        name:      String,
        actor:     rpc_command::Actor,
        args:      Vec<String>,
        unique_id: libc::intptr_t) -> Self {
    
        todo!();
        /*
            : category(std::move(category)), name(std::move(name)), actor(std::move(actor)), argNames(std::move(args)),
              unique_id(unique_id)
        */
    }

    /**
      | Simplified constructor taking plain
      | 
      | RpcMethodFnType function pointer.
      |
      */
    pub fn new_from_category_and_callback(
        category: &str,
        fn_:      RpcMethodFnType) -> Self {
    
        todo!();
        /*
            : CRPCCommand(
                  category,
                  fn().m_name,
                  [fn](const JSONRPCRequest& request, UniValue& result, bool) { result = fn().HandleRequest(request); return true; },
                  fn().GetArgNames(),
                  intptr_t(fn))
        */
    }
}

/**
  | RPC command dispatcher.
  |
  */
pub struct RPCTable {
    map_commands: HashMap<String,Vec<*const RPCCommand>>,
}

lazy_static!{
    /*
    extern CRPCTable tableRPC;
    */
}

//-------------------------------------------[.cpp/bitcoin/src/rpc/server.cpp]

lazy_static!{
    /*
    static Mutex g_rpc_warmup_mutex;
    static std::atomic<bool> g_rpc_running{false};
    static bool fRPCInWarmup GUARDED_BY(g_rpc_warmup_mutex) = true;
    static std::string rpcWarmupStatus GUARDED_BY(g_rpc_warmup_mutex) = "RPC server started";
    /* Timer-creating functions */
    static RPCTimerInterface* timerInterface = nullptr;
    /* Map of name to timer. */
    static Mutex g_deadline_timers_mutex;
    static std::map<std::string, std::unique_ptr<RPCTimerBase> > deadlineTimers GUARDED_BY(g_deadline_timers_mutex);
    */
}

pub struct RPCCommandExecutionInfo
{
    method: String,
    start:  i64,
}

pub struct RPCServerInfo
{
    mutex: std::sync::Mutex<rpc_server_info::Inner>,
}

pub mod rpc_server_info {

    use super::*;

    pub struct Inner {
        active_commands: LinkedList<RPCCommandExecutionInfo>,
    }
}

lazy_static!{
    /*
    static RPCServerInfo g_rpc_server_info;
    */
}

pub struct RPCCommandExecution {
    it: Box<dyn Iterator<Item = RPCCommandExecutionInfo>>,
}

impl Drop for RPCCommandExecution {
    fn drop(&mut self) {
        todo!();
        /*
            LOCK(g_rpc_server_info.mutex);
            g_rpc_server_info.active_commands.erase(it);
        */
    }
}

impl RPCCommandExecution {

    
    pub fn new(method: &String) -> Self {
    
        todo!();
        /*


            LOCK(g_rpc_server_info.mutex);
            it = g_rpc_server_info.active_commands.insert(g_rpc_server_info.active_commands.end(), {method, GetTimeMicros()});
        */
    }
}

lazy_static!{
    /*
    static struct CRPCSignals
    {
        boost::signals2::signal<c_void ()> Started;
        boost::signals2::signal<c_void ()> Stopped;
    } g_rpcSignals;
    */
}

//TODO: where is this?
pub struct RPCServer {}

impl RPCServer {
    
    pub fn on_started(&mut self, slot: fn() -> ())  {
        
        todo!();
        /*
            g_rpcSignals.Started.connect(slot);
        */
    }
    
    pub fn on_stopped(&mut self, slot: fn() -> ())  {
        
        todo!();
        /*
            g_rpcSignals.Stopped.connect(slot);
        */
    }
}

impl RPCTable {

    pub fn help(&self, 
        str_command: &String,
        helpreq:     &JSONRPCRequest) -> String {
        
        todo!();
        /*
            std::string strRet;
        std::string category;
        std::set<intptr_t> setDone;
        std::vector<std::pair<std::string, const CRPCCommand*> > vCommands;

        for (const auto& entry : mapCommands)
            vCommands.push_back(make_pair(entry.second.front()->category + entry.first, entry.second.front()));
        sort(vCommands.begin(), vCommands.end());

        JSONRPCRequest jreq = helpreq;
        jreq.mode = JSONRPCRequest::GET_HELP;
        jreq.params = UniValue();

        for (const std::pair<std::string, const CRPCCommand*>& command : vCommands)
        {
            const CRPCCommand *pcmd = command.second;
            std::string strMethod = pcmd->name;
            if ((strCommand != "" || pcmd->category == "hidden") && strMethod != strCommand)
                continue;
            jreq.strMethod = strMethod;
            try
            {
                UniValue unused_result;
                if (setDone.insert(pcmd->unique_id).second)
                    pcmd->actor(jreq, unused_result, true /* last_handler */);
            }
            catch (const std::exception& e)
            {
                // Help text is returned in an exception
                std::string strHelp = std::string(e.what());
                if (strCommand == "")
                {
                    if (strHelp.find('\n') != std::string::npos)
                        strHelp = strHelp.substr(0, strHelp.find('\n'));

                    if (category != pcmd->category)
                    {
                        if (!category.empty())
                            strRet += "\n";
                        category = pcmd->category;
                        strRet += "== " + Capitalize(category) + " ==\n";
                    }
                }
                strRet += strHelp + "\n";
            }
        }
        if (strRet == "")
            strRet = strprintf("help: unknown command: %s\n", strCommand);
        strRet = strRet.substr(0,strRet.size()-1);
        return strRet;
        */
    }
}

pub fn help() -> RPCHelpMan {
    
    todo!();
        /*
            return RPCHelpMan{"help",
                    "\nList all commands, or get help for a specified command.\n",
                    {
                        {"command", RPCArg::Type::STR, RPCArg::DefaultHint{"all commands"}, "The command to get help on"},
                    },
                    {
                        RPCResult{RPCResult::Type::STR, "", "The help text"},
                        RPCResult{RPCResult::Type::ANY, "", ""},
                    },
                    RPCExamples{""},
            [&](const RPCHelpMan& self, const JSONRPCRequest& jsonRequest) -> UniValue
    {
        std::string strCommand;
        if (jsonRequest.params.size() > 0) {
            strCommand = jsonRequest.params[0].get_str();
        }
        if (strCommand == "dump_all_command_conversions") {
            // Used for testing only, undocumented
            return tableRPC.dumpArgMap(jsonRequest);
        }

        return tableRPC.help(strCommand, jsonRequest);
    },
        };
        */
}

pub fn stop() -> RPCHelpMan {
    
    todo!();
        /*
            static const std::string RESULT{PACKAGE_NAME " stopping"};
        return RPCHelpMan{"stop",
        // Also accept the hidden 'wait' integer argument (milliseconds)
        // For instance, 'stop 1000' makes the call wait 1 second before returning
        // to the client (intended for testing)
                    "\nRequest a graceful shutdown of " PACKAGE_NAME ".",
                    {
                        {"wait", RPCArg::Type::NUM, RPCArg::Optional::OMITTED_NAMED_ARG, "how long to wait in ms", "", {}, /* hidden */ true},
                    },
                    RPCResult{RPCResult::Type::STR, "", "A string with the content '" + RESULT + "'"},
                    RPCExamples{""},
            [&](const RPCHelpMan& self, const JSONRPCRequest& jsonRequest) -> UniValue
    {
        // Event loop will exit after current HTTP requests have been handled, so
        // this reply will get back to the client.
        StartShutdown();
        if (jsonRequest.params[0].isNum()) {
            UninterruptibleSleep(std::chrono::milliseconds{jsonRequest.params[0].get_int()});
        }
        return RESULT;
    },
        };
        */
}

pub fn uptime() -> RPCHelpMan {
    
    todo!();
        /*
            return RPCHelpMan{"uptime",
                    "\nReturns the total uptime of the server.\n",
                                {},
                                RPCResult{
                                    RPCResult::Type::NUM, "", "The number of seconds that the server has been running"
                                },
                    RPCExamples{
                        HelpExampleCli("uptime", "")
                    + HelpExampleRpc("uptime", "")
                    },
            [&](const RPCHelpMan& self, const JSONRPCRequest& request) -> UniValue
    {
        return GetTime() - GetStartupTime();
    }
        };
        */
}

pub fn getrpcinfo() -> RPCHelpMan {
    
    todo!();
        /*
            return RPCHelpMan{"getrpcinfo",
                    "\nReturns details of the RPC server.\n",
                    {},
                    RPCResult{
                        RPCResult::Type::OBJ, "", "",
                        {
                            {RPCResult::Type::ARR, "active_commands", "All active commands",
                            {
                                {RPCResult::Type::OBJ, "", "Information about an active command",
                                {
                                     {RPCResult::Type::STR, "method", "The name of the RPC command"},
                                     {RPCResult::Type::NUM, "duration", "The running time in microseconds"},
                                }},
                            }},
                            {RPCResult::Type::STR, "logpath", "The complete file path to the debug log"},
                        }
                    },
                    RPCExamples{
                        HelpExampleCli("getrpcinfo", "")
                    + HelpExampleRpc("getrpcinfo", "")},
            [&](const RPCHelpMan& self, const JSONRPCRequest& request) -> UniValue
    {
        LOCK(g_rpc_server_info.mutex);
        UniValue active_commands(UniValue::VARR);
        for (const RPCCommandExecutionInfo& info : g_rpc_server_info.active_commands) {
            UniValue entry(UniValue::VOBJ);
            entry.pushKV("method", info.method);
            entry.pushKV("duration", GetTimeMicros() - info.start);
            active_commands.push_back(entry);
        }

        UniValue result(UniValue::VOBJ);
        result.pushKV("active_commands", active_commands);

        const std::string path = LogInstance().m_file_path.u8string();
        UniValue log_path(UniValue::VSTR, path);
        result.pushKV("logpath", log_path);

        return result;
    }
        };
        */
}

lazy_static!{
    /*
    static const CRPCCommand vRPCCommands[] =
    { //  category               actor (function)
      //  ---------------------  -----------------------
        /* Overall control/query calls */
        { "control",             &getrpcinfo,             },
        { "control",             &help,                   },
        { "control",             &stop,                   },
        { "control",             &uptime,                 },
    };
    */
}

impl Default for RPCTable {

    fn default() -> Self {
    
        todo!();
        /*


            for (const auto& c : vRPCCommands) {
            appendCommand(c.name, &c);
        }
        */
    }
}

impl RPCTable {

    /**
      | Appends a CRPCCommand to the dispatch
      | table.
      | 
      | Precondition: RPC server is not running
      | 
      | Commands with different method names
      | but the same unique_id will be considered
      | aliases, and only the first registered
      | method name will show up in the help text
      | command listing. Aliased commands
      | do not have to have the same behavior.
      | Server and client code can distinguish
      | between calls based on method name,
      | and aliased commands can also register
      | different names, types, and numbers
      | of parameters.
      |
      */
    pub fn append_command(&mut self, 
        name: &String,
        pcmd: *const RPCCommand)  {
        
        todo!();
        /*
            CHECK_NONFATAL(!IsRPCRunning()); // Only add commands before rpc is running

        mapCommands[name].push_back(pcmd);
        */
    }
    
    pub fn remove_command(&mut self, 
        name: &String,
        pcmd: *const RPCCommand) -> bool {
        
        todo!();
        /*
            auto it = mapCommands.find(name);
        if (it != mapCommands.end()) {
            auto new_end = std::remove(it->second.begin(), it->second.end(), pcmd);
            if (it->second.end() != new_end) {
                it->second.erase(new_end, it->second.end());
                return true;
            }
        }
        return false;
        */
    }
}

pub fn startrpc()  {
    
    todo!();
        /*
            LogPrint(BCLog::RPC, "Starting RPC\n");
        g_rpc_running = true;
        g_rpcSignals.Started();
        */
}

pub fn interruptrpc()  {
    
    todo!();
        /*
            static std::once_flag g_rpc_interrupt_flag;
        // This function could be called twice if the GUI has been started with -server=1.
        std::call_once(g_rpc_interrupt_flag, []() {
            LogPrint(BCLog::RPC, "Interrupting RPC\n");
            // Interrupt e.g. running longpolls
            g_rpc_running = false;
        });
        */
}

pub fn stoprpc()  {
    
    todo!();
        /*
            static std::once_flag g_rpc_stop_flag;
        // This function could be called twice if the GUI has been started with -server=1.
        assert(!g_rpc_running);
        std::call_once(g_rpc_stop_flag, []() {
            LogPrint(BCLog::RPC, "Stopping RPC\n");
            
    [&]() { LOCK(g_deadline_timers_mutex);  deadlineTimers.clear() }()
    ;
            DeleteAuthCookie();
            g_rpcSignals.Stopped();
        });
        */
}

/**
  | Query whether RPC is running
  |
  */
pub fn is_rpc_running() -> bool {
    
    todo!();
        /*
            return g_rpc_running;
        */
}

/**
  | Throw JSONRPCError if RPC is not running
  |
  */
pub fn rpc_interruption_point()  {
    
    todo!();
        /*
            if (!IsRPCRunning()) throw JSONRPCError(RPC_CLIENT_NOT_CONNECTED, "Shutting down");
        */
}

/**
  | Set the RPC warmup status. When this
  | is done, all RPC calls will error out
  | immediately with RPC_IN_WARMUP.
  |
  */
pub fn set_rpc_warmup_status(new_status: &String)  {
    
    todo!();
        /*
            LOCK(g_rpc_warmup_mutex);
        rpcWarmupStatus = newStatus;
        */
}

/**
  | Mark warmup as done. RPC calls will be
  | processed from now on.
  |
  */
pub fn set_rpc_warmup_finished()  {
    
    todo!();
        /*
            LOCK(g_rpc_warmup_mutex);
        assert(fRPCInWarmup);
        fRPCInWarmup = false;
        */
}

/**
  | returns the current warmup state.
  |
  */
pub fn rpc_is_in_warmup(out_status: *mut String) -> bool {
    
    todo!();
        /*
            LOCK(g_rpc_warmup_mutex);
        if (outStatus)
            *outStatus = rpcWarmupStatus;
        return fRPCInWarmup;
        */
}

pub fn is_deprecated_rpc_enabled(method: &String) -> bool {
    
    todo!();
        /*
            const std::vector<std::string> enabled_methods = gArgs.GetArgs("-deprecatedrpc");

        return find(enabled_methods.begin(), enabled_methods.end(), method) != enabled_methods.end();
        */
}

pub fn jsonrpc_exec_one(
        jreq: JSONRPCRequest,
        req:  &UniValue) -> UniValue {
    
    todo!();
        /*
            UniValue rpc_result(UniValue::VOBJ);

        try {
            jreq.parse(req);

            UniValue result = tableRPC.execute(jreq);
            rpc_result = JSONRPCReplyObj(result, NullUniValue, jreq.id);
        }
        catch (const UniValue& objError)
        {
            rpc_result = JSONRPCReplyObj(NullUniValue, objError, jreq.id);
        }
        catch (const std::exception& e)
        {
            rpc_result = JSONRPCReplyObj(NullUniValue,
                                         JSONRPCError(RPC_PARSE_ERROR, e.what()), jreq.id);
        }

        return rpc_result;
        */
}

pub fn jsonrpc_exec_batch(
        jreq: &JSONRPCRequest,
        req:  &UniValue) -> String {
    
    todo!();
        /*
            UniValue ret(UniValue::VARR);
        for (unsigned int reqIdx = 0; reqIdx < vReq.size(); reqIdx++)
            ret.push_back(JSONRPCExecOne(jreq, vReq[reqIdx]));

        return ret.write() + "\n";
        */
}

/**
  | Process named arguments into a vector
  | of positional arguments, based on the
  | passed-in specification for the RPC
  | call's arguments.
  |
  */
#[inline] pub fn transform_named_arguments(
        in_:       &JSONRPCRequest,
        arg_names: &Vec<String>) -> JSONRPCRequest {
    
    todo!();
        /*
            JSONRPCRequest out = in;
        out.params = UniValue(UniValue::VARR);
        // Build a map of parameters, and remove ones that have been processed, so that we can throw a focused error if
        // there is an unknown one.
        const std::vector<std::string>& keys = in.params.getKeys();
        const std::vector<UniValue>& values = in.params.getValues();
        std::unordered_map<std::string, const UniValue*> argsIn;
        for (size_t i=0; i<keys.size(); ++i) {
            argsIn[keys[i]] = &values[i];
        }
        // Process expected parameters.
        int hole = 0;
        for (const std::string &argNamePattern: argNames) {
            std::vector<std::string> vargNames;
            boost::algorithm::split(vargNames, argNamePattern, boost::algorithm::is_any_of("|"));
            auto fr = argsIn.end();
            for (const std::string & argName : vargNames) {
                fr = argsIn.find(argName);
                if (fr != argsIn.end()) {
                    break;
                }
            }
            if (fr != argsIn.end()) {
                for (int i = 0; i < hole; ++i) {
                    // Fill hole between specified parameters with JSON nulls,
                    // but not at the end (for backwards compatibility with calls
                    // that act based on number of specified parameters).
                    out.params.push_back(UniValue());
                }
                hole = 0;
                out.params.push_back(*fr->second);
                argsIn.erase(fr);
            } else {
                hole += 1;
            }
        }
        // If there are still arguments in the argsIn map, this is an error.
        if (!argsIn.empty()) {
            throw JSONRPCError(RPC_INVALID_PARAMETER, "Unknown named parameter " + argsIn.begin()->first);
        }
        // Return request with named arguments transformed to positional arguments
        return out;
        */
}

pub fn execute_commands(
        commands: &Vec<*const RPCCommand>,
        request:  &JSONRPCRequest,
        result:   &mut UniValue) -> bool {
    
    todo!();
        /*
            for (const auto& command : commands) {
            if (ExecuteCommand(*command, request, result, &command == &commands.back())) {
                return true;
            }
        }
        return false;
        */
}

impl RPCTable {
    
    /**
      | Execute a method.
      | 
      | -----------
      | @param request
      | 
      | The JSONRPCRequest to execute
      | 
      | -----------
      | @return
      | 
      | Result of the call. @throws an exception
      | (UniValue) when an error happens.
      |
      */
    pub fn execute(&self, request: &JSONRPCRequest) -> UniValue {
        
        todo!();
        /*
            // Return immediately if in warmup
        {
            LOCK(g_rpc_warmup_mutex);
            if (fRPCInWarmup)
                throw JSONRPCError(RPC_IN_WARMUP, rpcWarmupStatus);
        }

        // Find method
        auto it = mapCommands.find(request.strMethod);
        if (it != mapCommands.end()) {
            UniValue result;
            if (ExecuteCommands(it->second, request, result)) {
                return result;
            }
        }
        throw JSONRPCError(RPC_METHOD_NOT_FOUND, "Method not found");
        */
    }
    
    /**
      | Returns a list of registered commands
      | 
      | -----------
      | @return
      | 
      | List of registered commands.
      |
      */
    pub fn list_commands(&self) -> Vec<String> {
        
        todo!();
        /*
            std::vector<std::string> commandList;
        for (const auto& i : mapCommands) commandList.emplace_back(i.first);
        return commandList;
        */
    }
    
    /**
      | Return all named arguments that need
      | to be converted by the client from string
      | to another JSON type
      |
      */
    pub fn dump_arg_map(&self, args_request: &JSONRPCRequest) -> UniValue {
        
        todo!();
        /*
            JSONRPCRequest request = args_request;
        request.mode = JSONRPCRequest::GET_ARGS;

        UniValue ret{UniValue::VARR};
        for (const auto& cmd : mapCommands) {
            UniValue result;
            if (ExecuteCommands(cmd.second, request, result)) {
                for (const auto& values : result.getValues()) {
                    ret.push_back(values);
                }
            }
        }
        return ret;
        */
    }
}

/**
  | Set the factory function for timer,
  | but only, if unset
  |
  */
pub fn rpc_set_timer_interface_if_unset<'a>(iface: &'a mut dyn RPCTimerInterface)  {
    
    todo!();
        /*
            if (!timerInterface)
            timerInterface = iface;
        */
}

/**
  | Set the factory function for timers
  |
  */
pub fn rpc_set_timer_interface<'a>(iface: &'a mut dyn RPCTimerInterface)  {
    
    todo!();
        /*
            timerInterface = iface;
        */
}

/**
  | Unset factory function for timers
  |
  */
pub fn rpc_unset_timer_interface<'a>(iface: &'a mut dyn RPCTimerInterface)  {
    
    todo!();
        /*
            if (timerInterface == iface)
            timerInterface = nullptr;
        */
}

/**
  | Run func nSeconds from now.
  | 
  | Overrides previous timer <name> (if
  | any).
  |
  */
pub fn rpc_run_later(
        name:      &String,
        func:      fn() -> (),
        n_seconds: i64)  {
    
    todo!();
        /*
            if (!timerInterface)
            throw JSONRPCError(RPC_INTERNAL_ERROR, "No timer handler registered for RPC");
        LOCK(g_deadline_timers_mutex);
        deadlineTimers.erase(name);
        LogPrint(BCLog::RPC, "queue run of timer %s in %i seconds (using %s)\n", name, nSeconds, timerInterface->Name());
        deadlineTimers.emplace(name, std::unique_ptr<RPCTimerBase>(timerInterface->NewTimer(func, nSeconds*1000)));
        */
}

/**
  | Retrieves any serialization flags
  | requested in command line argument
  |
  */
pub fn rpc_serialization_flags() -> i32 {
    
    todo!();
        /*
            int flag = 0;
        if (gArgs.GetIntArg("-rpcserialversion", DEFAULT_RPC_SERIALIZE_VERSION) == 0)
            flag |= SERIALIZE_TRANSACTION_NO_WITNESS;
        return flag;
        */
}

lazy_static!{
    /*
    CRPCTable tableRPC;
    */
}

pub fn execute_command(
        command:      &RPCCommand,
        request:      &JSONRPCRequest,
        result:       &mut UniValue,
        last_handler: bool) -> bool {
    
    todo!();
        /*
            try
        {
            RPCCommandExecution execution(request.strMethod);
            // Execute, convert arguments to array if necessary
            if (request.params.isObject()) {
                return command.actor(transformNamedArguments(request, command.argNames), result, last_handler);
            } else {
                return command.actor(request, result, last_handler);
            }
        }
        catch (const std::exception& e)
        {
            throw JSONRPCError(RPC_MISC_ERROR, e.what());
        }
        */
}
