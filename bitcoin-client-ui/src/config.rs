crate::ix!();

//this was typed "Auto"
pub const ABORT_ERROR: fn(s: &BilingualStr) -> bool = init_error;

lazy_static!{
    static ref UI_INTERFACE: ClientUIInterface = ClientUIInterface::default();
    static ref UI_SIGNALS:   UISignals         = UISignals::default();
}
