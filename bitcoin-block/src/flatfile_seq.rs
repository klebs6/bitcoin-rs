crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/flatfile.h]

/**
  | FlatFileSeq represents a sequence
  | of numbered files storing raw data.
  | This class facilitates access to and
  | efficient management of these files.
  |
  */
pub struct FlatFileSeq {
    pub dir:        PathBuf,
    pub prefix:     *const u8,
    pub chunk_size: usize,
}

//-------------------------------------------[.cpp/bitcoin/src/flatfile.cpp]
impl FlatFileSeq {

    /**
      | Constructor
      | 
      | -----------
      | @param dir
      | 
      | The base directory that all files live
      | in.
      | ----------
      | @param prefix
      | 
      | A short prefix given to all file names.
      | ----------
      | @param chunk_size
      | 
      | Disk space is pre-allocated in multiples
      | of this amount.
      |
      */
    pub fn new(
        dir:        PathBuf,
        prefix:     *const u8,
        chunk_size: usize) -> Result<Self,StdException> {
    
        let mut x = Self {
            dir:        dir,
            prefix:     prefix,
            chunk_size: chunk_size,
        };

        if x.chunk_size == 0 {
            return Err(invalid_argument("chunk_size must be positive"));
        }

        Ok(x)
    }
    
    /**
      | Get the name of the file at the given position.
      |
      */
    pub fn file_name(&self, pos: &FlatFilePos) -> PathBuf {
        
        let mut buf: PathBuf = PathBuf::new();

        buf.push(self.dir.clone());

        buf.push(format!{"{:p}{:05}.dat", self.prefix, pos.n_file});

        buf
    }
    
    /**
      | Open a handle to the file at the given
      | position.
      |
      */
    pub fn open(&mut self, 
        pos:       &FlatFilePos,
        read_only: Option<bool>) -> *mut libc::FILE {

        let read_only: bool = read_only.unwrap_or(false);

        if pos.is_null() {
            return std::ptr::null_mut();
        }

        let path: PathBuf = self.file_name(pos);

        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        let mode = match read_only {
            true   => "rb",
            false  => "rb+"
        }.as_ptr() as *const i8;

        let path_str = path.as_os_str().to_str().unwrap().as_ptr() as *const i8;

        let mut file: *mut libc::FILE = {
            unsafe {
                libc::fopen(path_str,mode)
            }
        };

        if file == null_mut() && !read_only {
            unsafe {
                file = libc::fopen(path_str,"wb+".as_ptr() as *const i8);
            }
        }

        if file == null_mut() {

            log_printf!(
                "Unable to open file {}\n", 
                path_to_string(path)
            );

            return std::ptr::null_mut();
        }

        if pos.n_pos != 0 
        && unsafe { 
            libc::fseek(
                file,
                pos.n_pos.try_into().unwrap(),
                libc::SEEK_SET) 
        } != 0 
        {
            log_printf!(
                "Unable to seek to position %u of %s\n", 
                pos.n_pos, 
                path_to_string(path)
            );

            unsafe {
                libc::fclose(file);
            }

            return std::ptr::null_mut();
        }

        file
    }
    
    /**
      | Allocate additional space in a file
      | after the given starting position.
      | The amount allocated will be the minimum
      | multiple of the sequence chunk size
      | greater than add_size.
      | 
      | -----------
      | @param[in] pos
      | 
      | The starting position that bytes will
      | be allocated after.
      | ----------
      | @param[in] add_size
      | 
      | The minimum number of bytes to be allocated.
      | ----------
      | @param[out] out_of_space
      | 
      | Whether the allocation failed due to
      | insufficient disk space.
      | 
      | -----------
      | @return
      | 
      | The number of bytes successfully allocated.
      |
      */
    pub fn allocate(&mut self, 
        pos:          &FlatFilePos,
        add_size:     usize,
        out_of_space: &mut bool) -> usize {
        
        *out_of_space = false;

        let n_old_chunks: u32 = {

            let n_pos      = pos.n_pos;
            let chunk_size: u32 = self.chunk_size.try_into().unwrap();

            (n_pos + chunk_size - 1) / chunk_size
        };

        let n_new_chunks: u32 = {

            let n_pos         = pos.n_pos;
            let chunk_size: u32 = self.chunk_size.try_into().unwrap();
            let add_size:   u32 = add_size.try_into().unwrap();

            (n_pos + add_size + chunk_size - 1) / chunk_size
        };

        if n_new_chunks > n_old_chunks {

            let old_size: usize = pos.n_pos.try_into().unwrap();
            let new_size: usize = usize::try_from(n_new_chunks).unwrap() * self.chunk_size;
            let inc_size: usize = new_size - old_size;

            if check_disk_space(
                &self.dir,
                Some(inc_size.try_into().unwrap())
            ) {

                let file: *mut libc::FILE = self.open(pos, None);

                if file != null_mut() {

                    log_print!(
                        bc_log::validation, 
                        "Pre-allocating up to position 0x{:x} in {}{:05}.dat\n", 
                        new_size, 
                        prefix, 
                        pos.n_file
                    );

                    allocate_file_range(
                        file, 
                        pos.n_pos, 
                        inc_size.try_into().unwrap()
                    );

                    unsafe {
                        libc::fclose(file);
                    }

                    return inc_size;
                }

            } else {

                *out_of_space = true;
            }
        }

        0
    }
    
    /**
      | Commit a file to disk, and optionally
      | truncate off extra pre-allocated bytes
      | if final.
      | 
      | -----------
      | @param[in] pos
      | 
      | The first unwritten position in the
      | file to be flushed.
      | ----------
      | @param[in] finalize
      | 
      | True if no more data will be written to
      | this file.
      | 
      | -----------
      | @return
      | 
      | true on success, false on failure.
      |
      */
    pub fn flush(&mut self, 
        pos:      &FlatFilePos,
        finalize: Option<bool>) -> Result<bool,String> {

        let finalize: bool = finalize.unwrap_or(false);
        
        // Avoid fseek to nPos
        let file: *mut libc::FILE = self.open(&FlatFilePos::new(pos.n_file,0), None);

        if file == null_mut() {

            let msg = format!{
                "flush: failed to open file {}", 
                pos.n_file
            };

            return Err(msg);
        }

        if finalize && !truncate_file(file,pos.n_pos) {

            unsafe {
                libc::fclose(file);
            }

            let msg = format!{
                "flush: failed to truncate file {}",
                pos.n_file
            };

            return Err(msg);
        }

        if !file_commit(file) {

            unsafe {
                libc::fclose(file);
            }

            let msg = format!{
                "flush: failed to commit file {}",
                pos.n_file
            };

            return Err(msg);
        }

        directory_commit(&self.dir);

        unsafe {
            libc::fclose(file);
        }

        Ok(true)
    }
}
