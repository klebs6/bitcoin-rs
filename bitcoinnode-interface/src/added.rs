// ---------------- [ File: bitcoinnode-interface/src/added.rs ]
crate::ix!();

pub struct AddedNodeInfo
{
    pub str_added_node:   String,
    pub resolved_address: Service,
    pub connected:        bool,
    pub inbound:          bool,
}
