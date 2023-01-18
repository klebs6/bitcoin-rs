crate::ix!();

pub fn format_exception(
        pex:        Option<&StdException>,
        psz_thread: &str) -> String {

    
    #[cfg(WIN32)]
    let psz_module: [i8; MAX_PATH] = "";

    #[cfg(WIN32)]
    get_module_file_namea(None, psz_module, size_of_val(&psz_module));

    #[cfg(not(WIN32))]
    let psz_module: &str = "bitcoin";

    if pex.is_some() {

        format!{
            "EXCEPTION: {:?}       \n{} in {}       \n",
                pex.unwrap(),
                psz_module,
                psz_thread
        }

    } else {

        format!{
            "UNKNOWN EXCEPTION       \n{} in {}       \n",
            psz_module,
            psz_thread
        }
    }
}

pub fn print_exception_continue(
        pex:        Option<&StdException>,
        psz_thread: &str)  {
    
    let message: String = format_exception(pex,psz_thread);

    println!("\n\n************************\n{}\n", message);

    eprintln!("\n\n************************\n{}\n", message);
}

/*
#ifdef WIN32
// Export main() and ensure working ASLR on Windows.
// Exporting a symbol will prevent the linker from stripping
// the .reloc section from the binary, which is a requirement
// for ASLR. This is a temporary workaround until a fixed
// version of binutils is used for releases.
__declspec(dllexport) int main(int argc, char* argv[])
{
    util::WinCmdLineArgs winArgs;
    std::tie(argc, argv) = winArgs.get();
#else
int main(int argc, char* argv[])
{
#endif
*/
pub fn cli_main(argv: &Vec<String>) -> Result<i32,StdException> {
    

    setup_environment();

    if !setup_networking() {
        eprintln!("Error: Initializing networking failed\n");
        return Ok(EXIT_FAILURE);
    }

    unsafe {
        event_set_log_callback(Some(libevent_log_cb));
    }

    let try_block = || -> TryBlockResult::<_,StdException> {
        let ret: i32 = app_init_rpc(argv);
        if ret != CONTINUE_EXECUTION {
            return TryBlockResult::Return(ret);
        }
        TryBlockResult::Success
    };

    match try_block() {
        TryBlockResult::Return(v)  => return Ok(v),
        TryBlockResult::Err(e)  => {
            print_exception_continue(Some(&e), "AppInitRPC()");
            return Ok(EXIT_FAILURE);
        },

        TryBlockResult::Break   => { }
        TryBlockResult::Success => { }
    }

    let mut ret: Result<i32,StdException> = Ok(EXIT_FAILURE);

    let try_block = |ret: &mut Result<i32,StdException>| -> TryBlockResult::<_,StdException> {
        *ret = command_linerpc(argv);

        match ret {
            Ok(_)  => {},
            Err(ref mut e) => return TryBlockResult::Err(e.clone()),
        }

        TryBlockResult::Success
    };

    match try_block(&mut ret) {
        TryBlockResult::Return(v)  => return v,
        TryBlockResult::Err(e)  => {
            match e {
                StdException::Default { .. }  => {
                    print_exception_continue(Some(&e), "CommandLineRPC()");
                }
                _  => {
                    print_exception_continue(None, "CommandLineRPC()");
                }
            }
        },

        TryBlockResult::Break   => { }
        TryBlockResult::Success => { }
    }

    ret
}
