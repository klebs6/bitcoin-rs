crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/compat/stdin.h]

pub struct NoechoInst { }

impl Drop for NoechoInst {
    fn drop(&mut self) {
        todo!();
        /*
            SetStdinEcho(true);
        */
    }
}

impl NoechoInst {
    
    pub fn new() -> Self {
    
        todo!();
        /*


            SetStdinEcho(false);
        */
    }
}

///-------------------------------
#[macro_export] macro_rules! no_stdin_echo {
    () => {
        /*
                NoechoInst _no_echo
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/compat/stdin.cpp]

/**
   https://stackoverflow.com/questions/1413445/reading-a-password-from-stdcin
  */
pub fn set_stdin_echo(enable: bool)  {
    
    todo!();
        /*
            #ifdef WIN32
        HANDLE hStdin = GetStdHandle(STD_INPUT_HANDLE);
        DWORD mode;
        GetConsoleMode(hStdin, &mode);
        if (!enable) {
            mode &= ~ENABLE_ECHO_INPUT;
        } else {
            mode |= ENABLE_ECHO_INPUT;
        }
        SetConsoleMode(hStdin, mode);
    #else
        struct termios tty;
        tcgetattr(STDIN_FILENO, &tty);
        if (!enable) {
            tty.c_lflag &= ~ECHO;
        } else {
            tty.c_lflag |= ECHO;
        }
        (c_void)tcsetattr(STDIN_FILENO, TCSANOW, &tty);
    #endif
        */
}

pub fn stdin_terminal() -> bool {
    
    todo!();
        /*
            #ifdef WIN32
        return _isatty(_fileno(stdin));
    #else
        return isatty(fileno(stdin));
    #endif
        */
}

pub fn stdin_ready() -> bool {
    
    todo!();
        /*
            if (!StdinTerminal()) {
            return true;
        }
    #ifdef WIN32
        return false;
    #else
        struct pollfd fds;
        fds.fd = 0; /* this is STDIN */
        fds.events = POLLIN;
        return poll(&fds, 1, 0) == 1;
    #endif
        */
}
