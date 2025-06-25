// ---------------- [ File: bitcoin-portmap/src/natpmp_discover.rs ]
crate::ix!();

#[cfg(any(USE_NATPMP,USE_UPNP))]
#[cfg(USE_NATPMP)]
pub fn natpmp_discover(
        natpmp:             *mut NatPmp,
        external_ipv4_addr: &mut InAddr) -> bool {
    
    todo!();
        /*
        const int r_send = sendpublicaddressrequest(natpmp);

        if (r_send == 2 ) { /* OK */
            int r_read;
            natpmpresp_t response;
            do {
                r_read = readnatpmpresponseorretry(natpmp, &response);
            } while (r_read == NATPMP_TRYAGAIN);

            if (r_read == 0) {
                external_ipv4_addr = response.pnu.publicaddress.addr;
                return true;
            } else if (r_read == NATPMP_ERR_NOGATEWAYSUPPORT) {
                LogPrintf("natpmp: The gateway does not support NAT-PMP.\n");
            } else {
                LogPrintf("natpmp: readnatpmpresponseorretry() for public address failed with %d error.\n", r_read);
            }
        } else {
            LogPrintf("natpmp: sendpublicaddressrequest() failed with %d error.\n", r_send);
        }

        return false;
        */
}
