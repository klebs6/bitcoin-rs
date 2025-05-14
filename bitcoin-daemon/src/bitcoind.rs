// ---------------- [ File: bitcoin-daemon/src/bitcoind.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bitcoind.cpp]

lazy_static!{
    /*
    const std::function<std::string(const char*)> G_TRANSLATION_FUN = nullptr;
    UrlDecodeFn* const URL_DECODE = urlDecode;
    */
}

pub fn bitcoind_main(
        argc: i32,
        argv: &[*mut u8]) -> i32 {
    
    #[cfg(WIN32)]
    {
        let win_args = WinCmdLineArgs::new();
        (argc, argv) = win_args.get();
    }

    let mut node: NodeContext = unsafe { std::mem::zeroed() };

    let mut exit_status: i32 = 0;

    let init: Box<dyn Init> = make_node_init(
        &mut node, 
        argc, 
        argv, 
        &mut exit_status
    );

    /* 
       if (!init) { return exit_status; } 
       */

    setup_environment();

    /*
      | Connect bitcoind signal handlers
      |
      */
    noui_connect();

    if app_init(&mut node, argc, argv) {
        libc::EXIT_SUCCESS
    } else {
        libc::EXIT_FAILURE
    }
}
