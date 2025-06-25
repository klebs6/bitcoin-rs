crate::ix!();

#[cfg(any(USE_NATPMP,USE_UPNP))]
#[cfg(USE_NATPMP)]
pub fn process_natpmp() -> bool {
    
    todo!();
        /*
        bool ret = false;
        natpmp_t natpmp;
        struct in_addr external_ipv4_addr;
        if (NatpmpInit(&natpmp) && NatpmpDiscover(&natpmp, external_ipv4_addr)) {
            bool external_ip_discovered = false;
            const uint16_t private_port = GetListenPort();
            do {
                ret = NatpmpMapping(&natpmp, external_ipv4_addr, private_port, external_ip_discovered);
            } while (ret && g_mapport_interrupt.sleep_for(PORT_MAPPING_REANNOUNCE_PERIOD));
            g_mapport_interrupt.reset();

            const int r_send = sendnewportmappingrequest(&natpmp, NATPMP_PROTOCOL_TCP, private_port, g_mapport_external_port, /* remove a port mapping */ 0);
            g_mapport_external_port = 0;
            if (r_send == 12 ) { /* OK */
                LogPrintf("natpmp: Port mapping removed successfully.\n");
            } else {
                LogPrintf("natpmp: sendnewportmappingrequest(0) failed with %d error.\n", r_send);
            }
        }

        closenatpmp(&natpmp);
        return ret;
        */
}
