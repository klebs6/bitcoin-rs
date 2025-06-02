crate::ix!();

#[derive(MutGetters, Getters, Setters)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
pub struct LoggerInner {
    pub(crate) fileout: *mut libc::FILE, // default = nullptr

    pub(crate) msgs_before_open: LinkedList<String>,

    pub(crate) buffering: bool, // default = true

    // Changed from `LinkedList<fn(&String) -> ()>`
    // to `LinkedList<Box<dyn Fn(&String) + Send + Sync + 'static>>`
    pub(crate) print_callbacks: LinkedList<Box<dyn Fn(&String) + Send + Sync + 'static>>,
}
