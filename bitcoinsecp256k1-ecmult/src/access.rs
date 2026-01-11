// ---------------- [ File: bitcoinsecp256k1-ecmult/src/access.rs ]
crate::ix!();

#[inline(always)]
pub(crate) unsafe fn ge_x(a: *const Ge) -> *const Fe {
    core::ptr::addr_of!((*(a)).x)
}

#[inline(always)]
pub(crate) unsafe fn ge_y(a: *const Ge) -> *const Fe {
    core::ptr::addr_of!((*(a)).y)
}

#[inline(always)]
pub(crate) unsafe fn ge_x_mut(a: *mut Ge) -> *mut Fe {
    core::ptr::addr_of_mut!((*(a)).x)
}

#[inline(always)]
pub(crate) unsafe fn ge_y_mut(a: *mut Ge) -> *mut Fe {
    core::ptr::addr_of_mut!((*(a)).y)
}

#[inline(always)]
pub(crate) unsafe fn ge_infinity(a: *const Ge) -> *const i32 {
    core::ptr::addr_of!((*(a)).infinity)
}

#[inline(always)]
pub(crate) unsafe fn ge_infinity_mut(a: *mut Ge) -> *mut i32 {
    core::ptr::addr_of_mut!((*(a)).infinity)
}

#[inline(always)]
pub(crate) unsafe fn gej_x(a: *const Gej) -> *const Fe {
    core::ptr::addr_of!((*(a)).x)
}

#[inline(always)]
pub(crate) unsafe fn gej_y(a: *const Gej) -> *const Fe {
    core::ptr::addr_of!((*(a)).y)
}

#[inline(always)]
pub(crate) unsafe fn gej_z(a: *const Gej) -> *const Fe {
    core::ptr::addr_of!((*(a)).z)
}

#[inline(always)]
pub(crate) unsafe fn gej_x_mut(a: *mut Gej) -> *mut Fe {
    core::ptr::addr_of_mut!((*(a)).x)
}

#[inline(always)]
pub(crate) unsafe fn gej_y_mut(a: *mut Gej) -> *mut Fe {
    core::ptr::addr_of_mut!((*(a)).y)
}

#[inline(always)]
pub(crate) unsafe fn gej_z_mut(a: *mut Gej) -> *mut Fe {
    core::ptr::addr_of_mut!((*(a)).z)
}

#[inline(always)]
pub(crate) unsafe fn gej_infinity(a: *const Gej) -> *const i32 {
    core::ptr::addr_of!((*(a)).infinity)
}

#[inline(always)]
pub(crate) unsafe fn gej_infinity_mut(a: *mut Gej) -> *mut i32 {
    core::ptr::addr_of_mut!((*(a)).infinity)
}

#[inline(always)]
pub(crate) unsafe fn ge_storage_x(a: *const GeStorage) -> *const FeStorage {
    core::ptr::addr_of!((*(a)).x)
}

#[inline(always)]
pub(crate) unsafe fn ge_storage_y(a: *const GeStorage) -> *const FeStorage {
    core::ptr::addr_of!((*(a)).y)
}

#[inline(always)]
pub(crate) unsafe fn ge_storage_x_mut(a: *mut GeStorage) -> *mut FeStorage {
    core::ptr::addr_of_mut!((*(a)).x)
}

#[inline(always)]
pub(crate) unsafe fn ge_storage_y_mut(a: *mut GeStorage) -> *mut FeStorage {
    core::ptr::addr_of_mut!((*(a)).y)
}


