crate::ix!();

pub const BASIC_FILTER_P: u8 = 19;
pub const BASIC_FILTER_M: u32 = 784931;

lazy_static!{

    pub static ref G_FILTER_TYPES: HashMap<BlockFilterType, String> = {

        let mut x = HashMap::<BlockFilterType, String>::new();

        x.insert(BlockFilterType::BASIC, "basic".to_string());

        x
    };
}
