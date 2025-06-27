
// ---------------- [ File: bitcoin-portmap/src/natpmp_mapping.rs ]
crate::ix!();

#[cfg(feature = "natpmp")]
pub fn natpmp_mapping(
        natpmp:                 *mut Natpmp,
        external_ipv4_addr:     &InAddr,
        private_port:           u16,
        external_ip_discovered: &mut bool) -> bool {
    
    todo!();
        /*
        const uint16_t suggested_external_port = g_mapport_external_port ? g_mapport_external_port : private_port;
        const int r_send = sendnewportmappingrequest(natpmp, NATPMP_PROTOCOL_TCP, private_port, suggested_external_port, /*seconds*/ 3600 );

        if (r_send == 12 ) {/* OK */
            int r_read;
            natpmpresp_t response;
            do {
                r_read = readnatpmpresponseorretry(natpmp, &response);
            } while (r_read == NATPMP_TRYAGAIN);

            if (r_read == 0) {
                auto pm = response.pnu.newportmapping;
                if (private_port == pm.privateport && pm.lifetime > 0) {
                    g_mapport_external_port = pm.mappedpublicport;
                    const CService external{external_ipv4_addr, pm.mappedpublicport};
                    if (!external_ip_discovered && fDiscover) {
                        AddLocal(external, LOCAL_MAPPED);
                        external_ip_discovered = true;
                    }
                    LogPrintf("natpmp: Port mapping successful. External address = %s\n", external.ToString());
                    return true;
                } else {
                    LogPrintf("natpmp: Port mapping failed.\n");
                }
            } else if (r_read == NATPMP_ERR_NOGATEWAYSUPPORT) {
                LogPrintf("natpmp: The gateway does not support NAT-PMP.\n");
            } else {
                LogPrintf("natpmp: readnatpmpresponseorretry() for port mapping failed with %d error.\n", r_read);
            }
        } else {
            LogPrintf("natpmp: sendnewportmappingrequest() failed with %d error.\n", r_send);
        }

        return false;
        */
}
