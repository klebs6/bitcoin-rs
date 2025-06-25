crate::ix!();



/**
  | A shortcut for (services & GetDesirableServiceFlags(services))
  | == GetDesirableServiceFlags(services),
  | ie determines whether the given set
  | of service flags are sufficient for
  | a peer to be "relevant".
  |
  */
#[inline] pub fn has_all_desirable_service_flags(services: ServiceFlags) -> bool {
    
    todo!();
        /*
            return !(GetDesirableServiceFlags(services) & (~services));
        */
}

/**
  | Checks if a peer with the given service
  | flags may be capable of having a robust
  | address-storage DB.
  |
  */
#[inline] pub fn may_have_useful_addressdb(services: ServiceFlags) -> bool {
    
    todo!();
        /*
            return (services & NODE_NETWORK) || (services & NODE_NETWORK_LIMITED);
        */
}
