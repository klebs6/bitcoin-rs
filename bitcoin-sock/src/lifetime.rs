// ---------------- [ File: bitcoin-sock/src/lifetime.rs ]
crate::ix!();

#[cfg(test)]
mod sock_lifetime_spec {
    use super::*;

    #[cfg(unix)]
    fn make_socket_pair() -> (libc::c_int, libc::c_int) {
        let mut sv = [-1; 2];
        let ret =
            unsafe { libc::socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, sv.as_mut_ptr()) };
        assert_eq!(ret, 0, "socketpair() failed");
        (sv[0], sv[1])
    }

    #[traced_test]
    fn reset_closes_descriptor() {
        #[cfg(unix)]
        {
            use libc::{read, EBADF, EINTR};

            let (a, _b) = make_socket_pair();
            let mut sock = Sock::from(a);

            sock.reset();
            assert_eq!(sock.get(), crate::compat::INVALID_SOCKET);

            let mut buf = [0u8; 1];
            let ret = unsafe { read(a, buf.as_mut_ptr() as *mut _, 1) };
            assert_eq!(ret, -1);

            let err = last_errno();
            assert_eq!(err, EBADF, "descriptor should be closed");
            assert_ne!(err, EINTR, "unexpected EINTR");
        }

        info!("reset_closes_descriptor passed");
    }

    #[traced_test]
    fn release_transfers_ownership() {
        #[cfg(unix)]
        {
            use libc::{fcntl, F_GETFD};

            let (a, _b) = make_socket_pair();
            let mut sock = Sock::from(a);

            let raw = sock.release();
            assert_eq!(sock.get(), crate::compat::INVALID_SOCKET);
            assert_ne!(raw, crate::compat::INVALID_SOCKET);

            let ret = unsafe { fcntl(raw, F_GETFD) };
            assert_ne!(ret, -1, "descriptor should still be open");
        }

        info!("release_transfers_ownership passed");
    }
}
