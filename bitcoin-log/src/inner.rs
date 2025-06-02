crate::ix!();

#[derive(MutGetters, Getters, Setters)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
pub struct LoggerInner {
    fileout: *mut libc::FILE, // default = nullptr

    msgs_before_open: LinkedList<String>,

    buffering: bool, // default = true

    // Changed from `LinkedList<fn(&String) -> ()>`
    // to `LinkedList<Box<dyn Fn(&String) + Send + Sync + 'static>>`
    print_callbacks: LinkedList<Box<dyn Fn(&String) + Send + Sync + 'static>>,
}
