// ---------------- [ File: bitcoin-imports/src/lib.rs ]
#![feature(test)]
#![feature(generic_associated_types)]
#![feature(exact_size_is_empty)]
#![allow(soft_unstable)]

//------------------------------------------
#[macro_use] mod multidex; pub use multidex::*;
#[macro_use] mod util;     pub use util::*;


#[macro_export] macro_rules! x { 
    ($x:ident) => { 
        mod $x; 
        pub use $x::*; 
    }
}

#[macro_export] macro_rules! ix { 
    () => { 
        use crate::{ 
            imports::* , 
        };
        use crate::*;
    } 
}

#[macro_export] macro_rules! as_mut_cvoid {
    ($x:expr) => {
        $x as *mut _ as *mut c_void
    };
}

#[macro_export] macro_rules! mut_cvoid {
    ($x:expr) => {
        &mut $x as *mut _ as *mut c_void
    };
}

#[macro_export] macro_rules! as_cvoid {
    ($x:expr) => {
        $x as *const _ as *const c_void
    }
}

#[macro_export] macro_rules! ternary {
    ($condition:expr,$if_true:expr,$if_false:expr) => {
        match $condition {
            true => $if_true,
            false => $if_false,
        }
    }
}

#[macro_export] macro_rules! from_cstr {
    ($ptr:expr) => {
        unsafe {
            std::ffi::CStr::from_ptr(
                $ptr as *const libc::c_char
            )
            .to_string_lossy()
            .to_string()
        }
    }
}

#[macro_export] macro_rules! func {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }}
}


//---------------------------
//this struct should be:
//
//- lightweight to clone
//- a wrapper around a single private inner item
//- send and sync
#[derive(Debug)]
pub struct Amo<T>(Arc<RwLock<Option<T>>>);

impl<T> Default for Amo<T> {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(None)))
    }
}

impl<T: PartialOrd> PartialOrd for Amo<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get().partial_cmp(&other.get())
    }
}

impl<T: Ord> Ord for Amo<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get().cmp(&other.get())
    }
}

pub fn amo_none<T>() -> Amo<T> {
    Amo::<T>::none()
}

impl<T: Serialize> Serialize for Amo<T> {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        match *self.getopt() {
            Some(ref val) => serializer.serialize_some(val),
            None          => serializer.serialize_none(),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Amo<T> {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> 
    {
        todo!();
    }
}

impl<T: PartialEq> PartialEq for Amo<T> {
    fn eq(&self, other: &Self) -> bool {

        match (self.getopt().as_ref(), other.getopt().as_ref()) {
            (Some(a), Some(b)) => a.eq(b),
            (None, None)       => true, // should it be this way?
            _                  => false,
        }
    }
}

impl<T: Eq> Eq for Amo<T> {}

pub type AmoReadGuard<'a, T>       = MappedRwLockReadGuard<'a, T>;
pub type AmoWriteGuard<'a, T>      = MappedRwLockWriteGuard<'a, T>;

pub type AmoOuterReadGuard<'a, T>  = RwLockReadGuard<'a, Option<T>>;
pub type AmoOuterWriteGuard<'a, T> = RwLockWriteGuard<'a, Option<T>>;

impl<T> Clone for Amo<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> From<T> for Amo<T> {
    fn from(x: T) -> Self {
        Self(Arc::new(RwLock::new(Some(x))))
    }
}

impl<T> From<&Self> for Amo<T> {
    fn from(x: &Self) -> Self {
        x.clone()
    }
}

impl<T> From<Option<T>> for Amo<T> {
    fn from(x: Option<T>) -> Self {
        Self(Arc::new(RwLock::new(x)))
    }
}

impl<T> Amo<T> {

    delegate!{
        to self.0.read() {
            pub fn is_some(&self) -> bool;
            pub fn is_none(&self) -> bool;
        }

        to self.0.write() {
            pub fn take(&self) -> Option<T>;
        }
    }

    //fn inner(&self) -> Arc<RwLock<Option<T>>> { self.0.clone() }

    pub fn inner_arc(self) -> Arc<T> {
        todo!();
    }

    pub fn load(&self, inner: Option<T>) {
        todo!();
    }

    pub fn replace(&self, inner: &T) {
        todo!();
    }

    pub fn none() -> Self {
        Self(Arc::new(RwLock::new(None)))
    }

    pub fn get<'a>(&'a self) -> AmoReadGuard<'a, T> {
        let mut guard = self.0.read();
        RwLockReadGuard::map(guard, |g| g.as_ref().unwrap())
    }

    pub fn get_mut<'a>(&'a self) -> AmoWriteGuard<'a, T> {
        let mut guard = self.0.write();
        RwLockWriteGuard::map(guard, |g| g.as_mut().unwrap())
    }

    pub fn getopt<'a>(&'a self) -> AmoOuterReadGuard<'a, T> {
        self.0.read()
    }

    pub fn getopt_mut<'a>(&'a self) -> AmoOuterWriteGuard<'a, T> {
        self.0.write()
    }
}

extern crate test;

pub struct Broken {}

impl Deref for Broken {

    type Target = BrokenDelegate;

    fn deref(&self) -> &Self::Target { 
        todo!(); 
    }
}

pub struct BrokenDelegate {}

impl BrokenDelegate {

    pub fn get_mem_pool_parents_const<T>(&self) -> T { 
        todo!() 
    }

    pub fn get_shared_tx<T>(&self) -> T { 
        todo!(); 
    }
}

#[cfg(windows)] extern crate winapi;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate derivative;
#[macro_use] extern crate static_assertions;
#[macro_use] extern crate bitflags;
#[macro_use] extern crate phf;
#[macro_use] extern crate maplit;
#[macro_use] extern crate serde_big_array;

pub use std::fs;

//pub use actix_web::*;
pub use ::atomic::*;
pub use ::atomic::Atomic;
pub use ::priority_queue::*;
pub use ::serde::*;
pub use ::serde::{Serialize,Deserialize};
pub use atomic_take::*;
pub use bitflags::*;
pub use integer_encoding::*;
pub use bitflags::bitflags;
pub use bitset::*;
pub use same_file::is_same_file;
pub use core::ffi::*;
pub use core::ffi::c_void;
pub use indoc::{indoc,formatdoc};
pub use std::ffi::OsStr;
pub use std::fs::{File,canonicalize};
pub use const_default::ConstDefault;
pub use tracing::{info,warn,debug,trace,error};
pub use traced_test::traced_test;
pub use tracing_setup::*;

pub use serde_big_array::BigArray;

pub use rand;

pub use rand::{
    thread_rng,
    RngCore,
};

pub use rand::seq::{
    SliceRandom,
};

pub use std::io::{
    BufWriter,
    BufReader,
    BufRead,
    Write,
    Read,
};

pub use std::string::{
    FromUtf8Error,
};

pub use phf::{phf_set, Set};

pub use thiserror::*;
pub use libevent::*;
pub use libevent_sys::*;
pub use core::ops::*;
pub use core::ptr::*;
pub use crossbeam::queue::SegQueue;
//pub use embedded_time::duration::*;
//pub use embedded_time::duration::Duration;
//pub use embedded_time::duration::{Microseconds,Seconds};
//pub use embedded_time::fixed_point::*;
pub use hashbrown::{HashMap,HashSet};
pub use lazy_static::*;
pub use lazy_static::lazy_static;
pub use libc::*;
pub use libc_stdhandle::*;
pub use libc;
pub use maplit::*;
pub use maplit::hashmap;
pub use modular_bitfield::prelude::*;
pub use multimap::*;
pub use multiset::HashMultiSet as MultiSet;

pub use nix;
pub use nix::sys::time::{
    TimeVal,
    TimeValLike,
};
pub use nix::sys::select::{
    select,
    FdSet,
};

pub use fs2;
pub use errno::{errno,Errno};
pub use num::Integer;

//pub use std::sync::MutexGuard;
pub use parking_lot;
pub use parking_lot::lock_api::RawMutex as RawMutexTrait;
pub use parking_lot::lock_api::RawReentrantMutex;

pub use parking_lot::{
    Condvar,
    ReentrantMutex,
    RawMutex,
    MutexGuard,
    MappedMutexGuard,
    Mutex,
    RwLock,
    MappedRwLockReadGuard,
    MappedRwLockWriteGuard,
    RwLockReadGuard,
    RwLockWriteGuard,
};

pub use pnet_sys::*;
pub use quanta;
pub use shared_mutex::*;
//pub use socket2::*;
pub use static_assertions::*;
pub use std::alloc::*;
pub use std::any::*;
pub use std::cell::*;
pub use std::cmp::*;
pub use std::cmp::Ordering;
pub use std::collections::*;
pub use std::hash::*;
pub use float_ord::*;
pub use std::mem::*;
pub use delegate::*;
pub use std::marker::*;
pub use atomic_float::{AtomicF32,AtomicF64};
pub use chrono;

pub use comparator::collections::binary_heap::BinaryHeap as MaxHeap;
pub use comparator::Comparator;

pub use time::{
    Date,
    Time,
    Instant,
    Duration,
    OffsetDateTime,
    PrimitiveDateTime,
};

#[macro_export] macro_rules! ptr_to_string {
    ($item:tt) => {
        std::ffi::CStr::from_ptr(psz_dest as *const c_char)
            .to_string_lossy()
            .to_string()
    }
}

//pub use std::net::*;
pub use std::net::{
    SocketAddr,
};

pub use std::ops::*;
pub use std::env;
pub use std::path::*;
pub use std::ptr::*;
pub use std::rc::*;
pub use std::sync::*;
pub use std::sync::atomic::*;
pub use std::sync::atomic::AtomicBool;
pub use std::sync::atomic;

pub use std::sync::{
    Arc,
    //Mutex,
};

pub use getset::{Setters,Getters,MutGetters};
pub use std::sync::Mutex as StdMutex;
pub use std::sync;
pub use test::Bencher;
pub use time_point::*;
pub use tinyvec::*;
pub use tinyvec;
pub use try_catch::*;
pub use std::pin::Pin;
pub use std::fmt;

pub use derivative::*;
pub use std::thread::{
    Thread,
    JoinHandle
};

pub use std::iter::{
    Peekable,
    Enumerate,
};

/*
pub use qt_core::{
    QSize,
    QModelIndex,
    ItemDataRole,
    cpp_core::CppBox,
};

pub use qmetaobject::{
    USER_ROLE,
};

pub use qt_gui::{
    QColor,
    QIcon,
};
*/

pub enum TryBlockResult<R,E> {
    Return(R),
    Err(E),
    Break,
    Success,
}

x!{stdexcept}

/**
  | Assume is the identity function.
  | 
  | - Should be used to run non-fatal checks.
  | In debug builds it behaves like
  | 
  | Assert()/assert() to notify developers
  | and testers about non-fatal errors.
  | 
  | In production it doesn't warn or log
  | anything.
  | 
  | - For fatal errors, use Assert().
  | 
  | - For non-fatal errors in interactive
  | sessions (e.g. RPC or command line interfaces),
  | CHECK_NONFATAL() might be more appropriate.
  |
  */
#[cfg(ABORT_ON_FAILED_ASSUME)]
#[macro_export] macro_rules! assume {
    ($val:expr) => {
        assert!($val);
        $val
    }
}

#[cfg(not(ABORT_ON_FAILED_ASSUME))]
#[macro_export] macro_rules! assume {
    ($val:expr) => {
        /*
                ([&]() ->
        decltype(get_pure_r_value(val)) { auto&&
        check = (val); return
        std::forward<decltype(get_pure_r_value(val))>(check);
        }())
        */
        $val
    }
}

/**
  | TODO: where else can we put this?
  |
  | TODO: wrap this in tiny vec, but need impl
  | Array for const N: usize
  |
  */
pub type PreVector<T,const N: usize> = tinyvec::TinyVec::<[T; N]>;
//pub type PreVector<T,const N: usize> = Rc<[T; N]>; 

pub trait Flush {

    /**
      | Save state to disk.
      |
      | Alternatively: Make sure all changes are
      | flushed to database file.
      |
      */
    fn flush(&mut self);
}

//use serde::*;
pub struct CommonType<T> { p: std::marker::PhantomData<T> }
pub type LockGuard<'a,T> = std::sync::MutexGuard<'a,T>;
//pub struct Elsewhere {}
//pub type KVMap      = Elsewhere;
//pub type Reader     = Elsewhere;
//pub type WriteBatch = Elsewhere;
