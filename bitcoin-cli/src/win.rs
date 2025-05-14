// ---------------- [ File: bitcoin-cli/src/win.rs ]
crate::ix!();

#[cfg(WIN32)]
pub struct WinCmdLineArgs {
    argc: i32,
    argv: *mut *mut u8,
    args: Vec<String>,
}

#[cfg(WIN32)]
impl WinCmdLineArgs {
    
    pub fn get(&mut self) -> Pair<i32,*mut *mut u8> {
        
        todo!();
        /*
        
        */
    }
}

#[cfg(WIN32)]
impl Drop for WinCmdLineArgs {

    #[cfg(WIN32)]
    fn drop(&mut self) {
        todo!();
        /*
            delete[] argv;
        */
    }
}

#[cfg(WIN32)]
impl WinCmdLineArgs {
    
    #[cfg(WIN32)]
    pub fn new() -> Self {
    
        todo!();
        /*
        wchar** wargv = CommandLineToArgvW(GetCommandLineW(), &argc);
        wstring_convert<codecvt_utf8_utf16<wchar>, wchar> utf8_cvt;
        argv = new char*[argc];
        args.resize(argc);
        for (int i = 0; i < argc; i++) {
            args[i] = utf8_cvt.to_bytes(wargv[i]);
            argv[i] = &*args[i].begin();
        }
        LocalFree(wargv);
        */
    }
    
    #[cfg(WIN32)]
    pub fn get(&mut self) -> (i32,*mut *mut u8) {
        (argc, argv)
    }
}
