// ---------------- [ File: bitcoin-remote/src/convert_table.rs ]
crate::ix!();

pub struct RPCConvertTable {
    members:         HashSet<(String,i32)>,
    members_by_name: HashSet<(String,String)>,
}

impl RPCConvertTable {
    
    pub fn convert_with_method_and_idx(&mut self, 
        method: &str,
        idx:    i32) -> bool {
        
        todo!();
        /*
            return (members.count(std::make_pair(method, idx)) > 0);
        */
    }
    
    pub fn convert_with_method_and_name(&mut self, 
        method: &str,
        name:   &str) -> bool {
        
        todo!();
        /*
            return (membersByName.count(std::make_pair(method, name)) > 0);
        */
    }
    
    pub fn new() -> Self {
    
        todo!();
        /*


            for (const auto& cp : vRPCConvertParams) {
            members.emplace(cp.methodName, cp.paramIdx);
            membersByName.emplace(cp.methodName, cp.paramName);
        }
        */
    }
}

lazy_static!{
    pub static ref RPC_CVT_TABLE: Mutex<RPCConvertTable> = Mutex::new(RPCConvertTable::new());
}

