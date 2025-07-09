crate::ix!();

pub type TransformType = fn(
    _0: *mut u32,
    _1: *const u8,
    _2: usize
) -> c_void;

pub type TransformD64Type = fn(_0: *mut u8, _1: *const u8) -> c_void;

lazy_static!{
    /*
    TransformType Transform = sha256::Transform;
    TransformD64Type TransformD64 = sha256::TransformD64;
    TransformD64Type TransformD64_2way = nullptr;
    TransformD64Type TransformD64_4way = nullptr;
    TransformD64Type TransformD64_8way = nullptr;
    */
}
