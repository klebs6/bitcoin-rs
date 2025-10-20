// ---------------- [ File: bitcoinnode-interface/src/peer.rs ]
crate::ix!();

/**
  | Is our peer's addrLocal potentially
  | useful as an external IP source?
  |
  */
pub fn is_peer_addr_local_good(pnode: *mut dyn NodeInterface) 
    -> bool 
{
    todo!();
        /*
            CService addrLocal = pnode->GetAddrLocal();
        return fDiscover && pnode->addr.IsRoutable() && addrLocal.IsRoutable() &&
               IsReachable(addrLocal.GetNetwork());
        */
}

/**
  | Returns a local address that we should
  | advertise to this peer
  |
  */
pub fn get_local_addr_for_peer(pnode: Amo<Box<dyn NodeInterface>>) 
    -> Option<Address> 
{
    todo!();
        /*
            CAddress addrLocal = GetLocalAddress(&pnode->addr, pnode->GetLocalServices());
        if (gArgs.GetBoolArg("-addrmantest", false)) {
            // use IPv4 loopback during addrmantest
            addrLocal = CAddress(CService(LookupNumeric("127.0.0.1", GetListenPort())), pnode->GetLocalServices());
        }
        // If discovery is enabled, sometimes give our peer the address it
        // tells us that it sees us as in case it has a better idea of our
        // address than we do.
        FastRandomContext rng;
        if (IsPeerAddrLocalGood(pnode) && (!addrLocal.IsRoutable() ||
             rng.randbits((GetnScore(addrLocal) > LOCAL_MANUAL) ? 3 : 1) == 0))
        {
            addrLocal.SetIP(pnode->GetAddrLocal());
        }
        if (addrLocal.IsRoutable() || gArgs.GetBoolArg("-addrmantest", false))
        {
            LogPrint(LogFlags::NET, "Advertising address %s to peer=%d\n", addrLocal.ToString(), pnode->GetId());
            return addrLocal;
        }
        // Address is unroutable. Don't advertise.
        return std::nullopt;
        */
}
