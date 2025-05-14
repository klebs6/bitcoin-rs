// ---------------- [ File: bitcoin-block/src/allocate_file_range.rs ]
crate::ix!();

/**
  | this function tries to make a particular
  | range of a file allocated (corresponding
  | to disk space) it is advisory, and the
  | range specified in the arguments will
  | never contain live data
  |
  */
pub fn allocate_file_range(
        file:   *mut libc::FILE,
        offset: u32,
        mut length: u32)  {

    unsafe {

        #[cfg(WIN32)]
        {
            //  Windows-specific version
            let h_file: HANDLE = get_osfhandle(fileno(file)) as HANDLE;;
            let mut n_file_size = large_integer::default();
            let n_end_pos: i64 = offset as i64 + length;

            n_file_size.u.low_part = n_end_pos & 0xFFFFFFFF;
            n_file_size.u.high_part = n_end_pos >> 32;

            set_file_pointer_ex(h_file, n_file_size, 0, file_begin);
            set_end_of_file(h_file);

            return;
        }

        #[cfg(MAC_OSX)]
        {
            //  OSX specific version
            //
            //  NOTE: Contrary to other OS versions,
            //  the OSX version assumes that
            //
            //  NOTE: offset is the size of the file.
            let mut fst = fstore_t::default();;
            fst.fst_flags = F_ALLOCATECONTIG;
            fst.fst_posmode = F_PEOFPOSMODE;
            fst.fst_offset = 0;

            // mac os fst_length takes the # of free
            // bytes to allocate, not desired file
            // size
            fst.fst_length = length;
            fst.fst_bytesalloc = 0;

            if fcntl(fileno(file),F_PREALLOCATE,&fst) == -1 {
                fst.fst_flags = F_ALLOCATEALL;
                fcntl(fileno(file), F_PREALLOCATE, &fst);
            }

            ftruncate(fileno(file), offset as libc::ptrdiff_t + length);

            return;
        }

        #[cfg(HAVE_POSIX_FALLOCATE)]
        {
            //  Version using posix_fallocate
            let n_end_pos: libc::ptrdiff_t = offset as libc::ptrdiff_t + length;;

            if 0 == posix_fallocate(fileno(file),0,n_end_pos) {
                return;
            }
        }

        //  Fallback version
        //  TODO: just write one byte per block
        lazy_static!{
            static ref buf: [u8; 65536] = [0; 65536];
        }

        if libc::fseek(
            file,
            offset.into(),
            libc::SEEK_SET) != 0 
        {
            return;
        }

        while length > 0{

            let mut now: u32 = 65536;

            if length < now {
                now = length;
            }

            // allowed to fail; this function is
            // advisory anyway
            libc::fwrite(
                buf.as_ptr() as *const c_void,
                1,
                now.try_into().unwrap(),
                file
            );

            length -= now;
        }
    }
}
