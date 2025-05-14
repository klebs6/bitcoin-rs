// ---------------- [ File: bitcoin-daemon/src/fork_daemon.rs ]
crate::ix!();

/**
  | Custom implementation of daemon().
  | This implements the same order of operations
  | as glibc.
  | 
  | Opens a pipe to the child process to be
  | able to wait for an event to occur.
  | 
  | -----------
  | @return
  | 
  | 0  if successful, and in child process.
  | >0 if successful, and in parent process.
  | -1 in case of error (in parent process).
  | 
  | In case of success, endpoint will be
  | one end of a pipe from the child to parent
  | process, which can be used with TokenWrite
  | (in the child) or TokenRead (in the parent).
  |
  */
#[cfg(HAVE_DECL_FORK)]
pub fn fork_daemon(
    nochdir:  bool,
    noclose:  bool,
    endpoint: &mut TokenPipeEnd) -> Result<i32,&'static str> {

    /*
       | communication pipe with child process
       |
       */
    let umbilical = TokenPipe::make();

    if umbilical.is_none() {
        return Err("pipe or pipe2 failed");
    }

    let pid: i32 = libc::fork();

    if pid < 0 {
        return Err("fork failed");
    }

    if pid != 0 {

        /*
           | Parent process gets read end, closes
           | write end.
           |
           */
        endpoint = umbilical.unwrap().take_read_end();

        umbilical.as_ref().unwrap().take_write_end().close();

        let status: i32 = endpoint.token_read();

        if status != 0 { 
            endpoint.close();
            return Err("something went wrong while setting up child process.");
        }

        return Ok(pid);
    }

    /*
       | Child process gets write end, closes
       | read end.
       |
       */
    endpoint = umbilical.as_ref().unwrap().take_write_end();

    umbilical.as_ref().unwrap().take_read_end().close();

    #[cfg(HAVE_DECL_SETSID)]
    if libc::setsid() < 0 {
        libc::exit(1); // setsid failed.
    }

    if !nochdir {
        if libc::chdir("/") != 0 {
            libc::exit(1); // chdir failed.
        }
    }

    if !noclose {

        /*
           | Open /dev/null, and clone it into
           | STDIN, STDOUT and STDERR to detach from
           | terminal.
           |
           */
        let fd: i32 = libc::open("/dev/null", O_RDWR);

        if fd >= 0 {

            let err: bool = 
                libc::dup2(fd, STDIN_FILENO) < 0 
                || libc::dup2(fd, STDOUT_FILENO) < 0 
                || libc::dup2(fd, STDERR_FILENO) < 0;

            /*
               | Don't close if fd<=2 to try to
               | handle the case where the program
               | was invoked without any file
               | descriptors open.
               */
            if fd > 2 {
                libc::close(fd);
            }

            if err {
                libc::exit(1); // dup2 failed.
            }

        } else {
            libc::exit(1); // open /dev/null failed.
        }
    }

    endpoint.token_write(0); // Success

    Ok(0)
}
