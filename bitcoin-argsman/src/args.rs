// ---------------- [ File: bitcoin-argsman/src/args.rs ]
crate::ix!();

pub mod base_options {
    use super::*;

    lazy_static!{
        pub static ref ARG_SET_CHAIN: ArgDescriptor = ArgDescriptor {
            name: "-chain=<chain>", 
            help: formatdoc!{
                "Use the chain <chain> (default:
                main). Allowed values: main, test, signet,
                regtest"
            },
            flags:    ArgsManagerFlags::ALLOW_ANY, 
            category: OptionsCategory::CHAINPARAMS
        };

        pub static ref ARG_REGTEST: ArgDescriptor = ArgDescriptor {
            name: "-regtest", 
            help: formatdoc!{
                "Enter regression test mode, which uses
                a special chain in which blocks can be solved
                instantly. This is intended for regression testing
                tools and app development. Equivalent to
                -chain=regtest."
            }, 
            flags:    ArgsManagerFlags::ALLOW_ANY | ArgsManagerFlags::DEBUG_ONLY, 
            category: OptionsCategory::CHAINPARAMS
        };

        pub static ref ARG_TESTACTIVATIONHEIGHT: ArgDescriptor = ArgDescriptor {
            name: "-testactivationheight=name@height.", 
            help: format!{
                "Set the activation height of 'name'
                (segwit, bip34, dersig,
                cltv,csv). (regtest-only)"
            }, 
            flags:    ArgsManagerFlags::ALLOW_ANY | ArgsManagerFlags::DEBUG_ONLY, 
            category: OptionsCategory::DEBUG_TEST
        };

        pub static ref ARG_TESTNET: ArgDescriptor = ArgDescriptor {
            name:  "-testnet", 
            help:  formatdoc!{
                "Use the test chain. Equivalent to -chain=test."
            }, 
            flags:    ArgsManagerFlags::ALLOW_ANY, 
            category: OptionsCategory::CHAINPARAMS
        };

        pub static ref ARG_SET_VBPARAMS: ArgDescriptor = ArgDescriptor {
            name:  "-vbparams=deployment:start:end[:min_activation_height]", 
            help:  formatdoc!{
                "Use given start/end times and
                min_activation_height for specified version bits
                deployment (regtest-only)"
            }, 
            flags:    ArgsManagerFlags::ALLOW_ANY | ArgsManagerFlags::DEBUG_ONLY, 
            category: OptionsCategory::CHAINPARAMS
        };

        pub static ref ARG_SIGNET: ArgDescriptor = ArgDescriptor {
            name: "-signet", 
            help: formatdoc!{
                "Use the signet chain. Equivalent to
                -chain=signet. Note that the network is defined by
                the -signetchallenge parameter"
            }, 
            flags:    ArgsManagerFlags::ALLOW_ANY, 
            category: OptionsCategory::CHAINPARAMS
        };

        pub static ref ARG_SIGNET_CHALLENGE: ArgDescriptor = ArgDescriptor {
            name:  "-signetchallenge", 
            help:  formatdoc!{
                "Blocks must satisfy the given script
                to be considered valid (only for signet networks;
                defaults to the global default signet test network
                challenge)"
            }, 
            flags:    ArgsManagerFlags::ALLOW_STRING, 
            category: OptionsCategory::CHAINPARAMS
        };

        pub static ref ARG_SIGNET_SEEDNODE: ArgDescriptor = ArgDescriptor {
            name: "-signetseednode", 
            help: formatdoc!{
                "Specify a seed node for the signet
                network, in the hostname[:port] format,
                e.g. sig.net:1234 (may be used multiple times to
                specify multiple seed nodes; defaults to the
                global default signet test networkseed node(s))"
            }, 
            flags:    ArgsManagerFlags::ALLOW_STRING, 
            category: OptionsCategory::CHAINPARAMS
        };
    }
}

lazy_static!{

    pub static ref ARG_HELP: ArgDescriptor = ArgDescriptor {
        name:  "-?", 
        help:  formatdoc!{
            "Print this help message and exit"
        }, 
        flags:    ArgsManagerFlags::ALLOW_ANY, 
        category: OptionsCategory::OPTIONS
    };

    pub static ref ARG_VERSION: ArgDescriptor = ArgDescriptor {
        name:  "-version",
        help:  formatdoc!{
            "Print version and exit"
        },
        flags:    ArgsManagerFlags::ALLOW_ANY,
        category: OptionsCategory::OPTIONS,
    };

    pub static ref ARG_SET_CONF_FILE: ArgDescriptor = ArgDescriptor {
        name: "-conf=<file>", 
        help: formatdoc!{
            "Specify configuration file. Relative
            paths will be prefixed by datadir
            location. (default: {})", 
            BITCOIN_CONF_FILENAME
        }, 
        flags:    ArgsManagerFlags::ALLOW_ANY, 
        category: OptionsCategory::OPTIONS
    };

    pub static ref ARG_SET_DATADIR: ArgDescriptor = ArgDescriptor {
        name:  "-datadir=<dir>", 
        help:  formatdoc!{
            "Specify data directory"
        }, 
        flags:    ArgsManagerFlags::ALLOW_ANY, 
        category: OptionsCategory::OPTIONS
    };

    pub static ref ARG_GENERATE: ArgDescriptor = ArgDescriptor {
        name: "-generate", 
        help: formatdoc!{
            "Generate blocks immediately, equivalent
            to RPC getnewaddress followed by RPC
            generatetoaddress. Optional positional integer
            arguments are number of blocks to generate
            (default: {}) and maximum iterations to try
            (default: {}), equivalent to RPC generatetoaddress
            nblocks and maxtries arguments. Example:
            bitcoin-cli -generate 4 1000", 
            DEFAULT_NBLOCKS, 
            DEFAULT_MAX_TRIES
        }, 
        flags:    ArgsManagerFlags::ALLOW_ANY, 
        category: OptionsCategory::OPTIONS
    };

    pub static ref ARG_ADDRINFO: ArgDescriptor = ArgDescriptor {
        name: "-addrinfo", 
        help: formatdoc!{
            "Get the number of addresses known to
            the node, per network and total."
        }, 
        flags:    ArgsManagerFlags::ALLOW_ANY, 
        category: OptionsCategory::OPTIONS
    };

    pub static ref ARG_GETINFO: ArgDescriptor = ArgDescriptor {
        name: "-getinfo", 
        help: formatdoc!{
            "Get general information from the remote
            server. Note that unlike server-side RPC calls,
            the results of -getinfo is the result of multiple
            non-atomic requests. Some entries in the result
            may represent results from different states
            (e.g. wallet balance may be as of a different
            block from the chain state reported)"
        }, 
        flags:    ArgsManagerFlags::ALLOW_ANY, 
        category: OptionsCategory::OPTIONS
    };

    pub static ref ARG_NETINFO: ArgDescriptor = ArgDescriptor {
        name: "-netinfo", 
        help: formatdoc!{
            "Get network peer connection information
            from the remote server. An optional
            integer argument from 0 to 4 can be passed
            for dif ferent peers listings (default:
            0). Pass \"help\" for detailed help
            documentation."
        }, 
        flags:    ArgsManagerFlags::ALLOW_ANY, 
        category: OptionsCategory::OPTIONS
    };

    pub static ref ARG_SET_COLOR: ArgDescriptor = ArgDescriptor { 
        name: "-color=<when>", 
        help: formatdoc!{
            "Color setting for CLI output (default:
            {}). Valid values: always, auto (add color codes
            when standard output is connected to a terminal
            and OS is not WIN32), never.",
            DEFAULT_COLOR_SETTING
        }, 
        flags:    ArgsManagerFlags::ALLOW_STRING, 
        category: OptionsCategory::OPTIONS
    };

    pub static ref ARG_NAMED: ArgDescriptor = ArgDescriptor { 
        name: "-named", 
        help: formatdoc!{
            "Pass named instead of positional
            arguments (default: {})",
            DEFAULT_NAMED
        }, 
        flags:    ArgsManagerFlags::ALLOW_ANY, 
        category: OptionsCategory::OPTIONS,
    };

    pub static ref ARG_SET_RPC_CLIENTTIMEOUT: ArgDescriptor = ArgDescriptor { 
        name: "-rpcclienttimeout=<n>", 
        help: formatdoc!{
            "Timeout in seconds during HTTP requests,
            or 0 for no timeout. (default: {})",
            DEFAULT_HTTP_CLIENT_TIMEOUT
        }, 
        flags:    ArgsManagerFlags::ALLOW_ANY, 
        category: OptionsCategory::OPTIONS
    };

    pub static ref ARG_SET_RPC_COOKIEFILE: ArgDescriptor = ArgDescriptor { 
        name: "-rpccookiefile=<loc>", 
        help: formatdoc!{
            "Location of the auth cookie. Relative
            paths will be prefixed by a net-specific datadir
            location. (default: data dir)"
        }, 
        flags:    ArgsManagerFlags::ALLOW_ANY, 
        category: OptionsCategory::OPTIONS
    };

    pub static ref ARG_SET_RPC_CONNECT: ArgDescriptor = ArgDescriptor { 
        name: "-rpcconnect=<ip>", 
        help: formatdoc!{
            "Send commands to node running on <ip>
            (default: {})",
            DEFAULT_RPCCONNECT
        }, 
        flags:    ArgsManagerFlags::ALLOW_ANY, 
        category: OptionsCategory::OPTIONS
    };

    pub static ref ARG_SET_RPC_PASSWORD: ArgDescriptor = ArgDescriptor { 
        name: "-rpcpassword=<pw>", 
        help: formatdoc!{
            "Password for JSON-RPC connections"
        }, 
        flags:    ArgsManagerFlags::ALLOW_ANY, 
        category: OptionsCategory::OPTIONS
    };

    pub static ref ARG_SET_RPC_PORT: ArgDescriptor = ArgDescriptor { 
        name: "-rpcport=<port>", 
        help: formatdoc!{
            "Connect to JSON-RPC on <port> (default:
            {}, testnet: {}, signet: {}, regtest: {})",
            DEFAULT_BASE_PARAMS.rpc_port(),
            TESTNET_BASE_PARAMS.rpc_port(),
            SIGNET_BASE_PARAMS.rpc_port(),
            REGTEST_BASE_PARAMS.rpc_port()
        }, 
        flags:    ArgsManagerFlags::ALLOW_ANY | ArgsManagerFlags::NETWORK_ONLY, 
        category: OptionsCategory::OPTIONS
    };

    pub static ref ARG_SET_RPC_USER: ArgDescriptor = ArgDescriptor { 
        name: "-rpcuser=<user>", 
        help: formatdoc!{
            "Username for JSON-RPC connections"
        }, 
        flags:    ArgsManagerFlags::ALLOW_ANY, 
        category: OptionsCategory::OPTIONS
    };

    pub static ref ARG_SET_RPC_WAIT: ArgDescriptor = ArgDescriptor { 
        name: "-rpcwait", 
        help: formatdoc!{
            "Wait for RPC server to start"
        }, 
        flags:    ArgsManagerFlags::ALLOW_ANY, 
        category: OptionsCategory::OPTIONS
    };

    pub static ref ARG_SET_RPC_WAITTIMEOUT: ArgDescriptor = ArgDescriptor { 
        name: "-rpcwaittimeout=<n>", 
        help: formatdoc!{
            "Timeout in seconds to wait for the RPC
            server to start, or 0 for no timeout. (default:
            {})",
            DEFAULT_WAIT_CLIENT_TIMEOUT
        }, 
        flags:    ArgsManagerFlags::ALLOW_INT, 
        category: OptionsCategory::OPTIONS
    };

    pub static ref ARG_SET_RPC_WALLET: ArgDescriptor = ArgDescriptor { 
        name: "-rpcwallet=<walletname>", 
        help: formatdoc!{
            "Send RPC for non-default wallet on RPC
            server (needs to exactly match corresponding
            -wallet option passed to bi tcoind). This changes
            the RPC endpoint used,
            e.g. http://127.0.0.1:8332/wallet/<walletname>"
        }, 
        flags:    ArgsManagerFlags::ALLOW_ANY, 
        category: OptionsCategory::OPTIONS
    };

    pub static ref ARG_STDIN: ArgDescriptor = ArgDescriptor { 
        name: "-stdin", 
        help: formatdoc!{
            "Read extra arguments from standard input,
            one per line until EOF/Ctrl-D (recommended for
            sensitive information such as passphras es). When
            combined with -stdinrpcpass, the first line from
            standard input is used for the RPC password."
        }, 
        flags:    ArgsManagerFlags::ALLOW_ANY, 
        category: OptionsCategory::OPTIONS
    };

    pub static ref ARG_STDINRPCPASS: ArgDescriptor = ArgDescriptor { 
        name: "-stdinrpcpass", 
        help: formatdoc!{
            "Read RPC password from standard input as
            a single line. When combined with -stdin, the
            first line from standard input is used for the RPC
            password. When combined with
            -stdinwalletpassphrase, -stdinrpcpass consumes the
            first line, and -stdinwalletpassphrase consumes
            the second ."
        }, 
        flags:    ArgsManagerFlags::ALLOW_ANY, 
        category: OptionsCategory::OPTIONS
    };

    pub static ref ARG_STDINWALLETPASSPHRASE: ArgDescriptor = ArgDescriptor { 
        name: "-stdinwalletpassphrase", 
        help: formatdoc!{
            "Read wallet passphrase from standard
            input as a single line. When combined with -stdin,
            the first line from stan dard input is used for
            the wallet passphrase."
        }, 
        flags:    ArgsManagerFlags::ALLOW_ANY, 
        category: OptionsCategory::OPTIONS
    };
}
