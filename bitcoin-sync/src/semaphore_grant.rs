// ---------------- [ File: bitcoin-sync/src/semaphore_grant.rs ]
crate::ix!();

/**
  | RAII-style semaphore lock
  |
  */
#[derive(Clone)]
pub struct SemaphoreGrant {
    sem:        Amo<Semaphore>,
    have_grant: bool,
}

impl Drop for SemaphoreGrant {
    fn drop(&mut self) {
        todo!();
        /*
            Release();
        */
    }
}

impl Into<bool> for &SemaphoreGrant {
    
    #[inline] fn into(self) -> bool {
        todo!();
        /*
            return fHaveGrant;
        */
    }
}

impl Default for SemaphoreGrant {

    fn default() -> Self {
        todo!();
        /*
        : sem(nullptr),
        : have_grant(false),

        
        */
    }
}

impl SemaphoreGrant {
    
    pub fn acquire(&mut self)  {
        
        todo!();
        /*
            if (fHaveGrant)
                return;
            sem->wait();
            fHaveGrant = true;
        */
    }
    
    pub fn release(&mut self)  {
        
        todo!();
        /*
            if (!fHaveGrant)
                return;
            sem->post();
            fHaveGrant = false;
        */
    }
    
    pub fn try_acquire(&mut self) -> bool {
        
        todo!();
        /*
            if (!fHaveGrant && sem->try_wait())
                fHaveGrant = true;
            return fHaveGrant;
        */
    }
    
    pub fn move_to(&mut self, grant: &mut SemaphoreGrant)  {
        
        todo!();
        /*
            grant.Release();
            grant.sem = sem;
            grant.fHaveGrant = fHaveGrant;
            fHaveGrant = false;
        */
    }
    
    
    pub fn new(
        sema: Amo<Semaphore>,
        try_: Option<bool>) -> Self {
        let try_:bool = try_.unwrap_or(false);
        todo!();
        /*
        : sem(&sema),
        : have_grant(false),

            if (fTry)
                TryAcquire();
            else
                Acquire();
        */
    }
}
