crate::ix!();

impl Connman {

    pub fn wake_message_handler(&self)  {
        
        self.mutex_msg_proc
            .get()
            .msg_proc_wake
            .store(true, atomic::Ordering::Relaxed);

        self.cond_msg_proc.notify_one();
    }
}
