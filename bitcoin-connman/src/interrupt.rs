crate::ix!();

pub fn interrupt_socks5(interrupt: bool)  {
    
    INTERRUPT_SOCKS5_RECV.store(interrupt, atomic::Ordering::Relaxed);
}

impl Connman {
    
    pub fn interrupt(&mut self)  {
        
        {
            let guard = self.mutex_msg_proc.get();

            self.flag_interrupt_msg_proc
                .store(true, atomic::Ordering::Relaxed);
        }

        self.cond_msg_proc
            .notify_all();

        self.interrupt_net.get_mut().invoke();

        interrupt_socks5(true);

        if self.sem_outbound.is_some() {
            for i in 0..self.max_outbound.load(atomic::Ordering::Relaxed) {
                self.sem_outbound.get_mut().post();
            }
        }

        if self.sem_addnode.is_some() {
            for i in 0..self.n_max_addnode.load(atomic::Ordering::Relaxed) {
                self.sem_addnode.get_mut().post();
            }
        }
    }
}
