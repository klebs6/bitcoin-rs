crate::ix!();

pub fn check_disk_space(
        dir:              &Path,
        additional_bytes: Option<u64>) -> bool {

    let additional_bytes: u64 = additional_bytes.unwrap_or(0);

    // 50 MiB
    pub const MIN_DISK_SPACE: u64 = 52428800;
    
    let free_bytes_available: u64 = fs2::free_space(dir).unwrap(); //fs::space(dir).available;

    free_bytes_available >= MIN_DISK_SPACE + additional_bytes
}
