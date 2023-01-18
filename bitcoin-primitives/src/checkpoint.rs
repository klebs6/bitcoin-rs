crate::ix!();

pub type MapCheckpoints = HashMap<i32,u256>;

#[derive(Default)]
pub struct CheckpointData {
    map_checkpoints: MapCheckpoints,
}

impl CheckpointData {

    pub fn get_height(&self) -> i32 {
        
        todo!();
        /*
            const auto& final_checkpoint = mapCheckpoints.rbegin();
            return final_checkpoint->first /* height */;
        */
    }
}
