// ---------------- [ File: bitcoinsecp256k1-group/src/ge_storage.rs ]
crate::ix!();

pub struct GeStorage {
    pub x: FeStorage,
    pub y: FeStorage,
}

#[macro_export]
macro_rules! ge_storage_const {
    ($a:expr,
     $b:expr,
     $c:expr,
     $d:expr,
     $e:expr,
     $f:expr,
     $g:expr,
     $h:expr,
     $i:expr,
     $j:expr,
     $k:expr,
     $l:expr,
     $m:expr,
     $n:expr,
     $o:expr,
     $p:expr) => {
        GeStorage {
            x: fe_storage_const!(($a), ($b), ($c), ($d), ($e), ($f), ($g), ($h)),
            y: fe_storage_const!(($i), ($j), ($k), ($l), ($m), ($n), ($o), ($p)),
        }
    };
}

#[macro_export]
macro_rules! ge_storage_const_get {
    ($t:ident) => {
        fe_storage_const_get!($t.x), fe_storage_const_get!($t.y)
    };
}

#[cfg(test)]
mod ge_storage_rs_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn ge_storage_const_macro_converts_back_to_expected_fe_values() {
        tracing::info!("Validating ge_storage_const! macro and ge_from_storage produce expected field values.");

        unsafe {
            let st: GeStorage = ge_storage_const!(
                0, 0, 0, 0, 0, 0, 0, 1,
                0, 0, 0, 0, 0, 0, 0, 2
            );

            let mut p: Ge = core::mem::zeroed();
            ge_from_storage(core::ptr::addr_of_mut!(p), core::ptr::addr_of!(st));

            let expected_x: Fe = secp256k1_group_exhaustive_test_support::fe_int(1);
            let expected_y: Fe = secp256k1_group_exhaustive_test_support::fe_int(2);

            assert!(fe_equal_var(core::ptr::addr_of!(p.x), core::ptr::addr_of!(expected_x)) != 0);
            assert!(fe_equal_var(core::ptr::addr_of!(p.y), core::ptr::addr_of!(expected_y)) != 0);
            assert!(ge_is_infinity(core::ptr::addr_of!(p)) == 0);
        }
    }
}

#[inline(always)]
pub fn ge_storage_x(this: *const GeStorage) -> *const FeStorage {
    unsafe { core::ptr::addr_of!((*this).x) }
}

#[inline(always)]
pub fn ge_storage_x_mut(this: *mut GeStorage) -> *mut FeStorage {
    unsafe { core::ptr::addr_of_mut!((*this).x) }
}

#[inline(always)]
pub fn ge_storage_y(this: *const GeStorage) -> *const FeStorage {
    unsafe { core::ptr::addr_of!((*this).y) }
}

#[inline(always)]
pub fn ge_storage_y_mut(this: *mut GeStorage) -> *mut FeStorage {
    unsafe { core::ptr::addr_of_mut!((*this).y) }
}
