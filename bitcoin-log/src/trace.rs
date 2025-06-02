// ---------------- [ File: bitcoin-log/src/trace.rs ]
crate::ix!();

include!(concat!(env!("OUT_DIR"), "/probes.rs"));

#[cfg(ENABLE_TRACING)] #[macro_export] macro_rules! trace0 {
    ($ctx:expr, $event:expr) => {
        generic_provider::trace0!(|| ($ctx, $event));
    };
}

#[cfg(ENABLE_TRACING)] #[macro_export] macro_rules! trace1 {
    ($ctx:expr, $event:expr, $a:expr) => {
        generic_provider::trace1!(|| ($ctx, $event, $a));
    };
}

#[cfg(ENABLE_TRACING)]
#[macro_export]
macro_rules! trace2 {
    ($ctx:expr, $event:expr, $a:expr, $b:expr) => {
        generic_provider::trace2!(|| ($ctx, $event, $a, $b));
    };
}

#[cfg(ENABLE_TRACING)]
#[macro_export]
macro_rules! trace3 {
    ($ctx:expr, $event:expr, $a:expr, $b:expr, $c:expr) => {
        generic_provider::trace3!(|| ($ctx, $event, $a, $b, $c));
    };
}

#[cfg(ENABLE_TRACING)]
#[macro_export]
macro_rules! trace4 {
    ($ctx:expr, $event:expr, $a:expr, $b:expr, $c:expr, $d:expr) => {
        generic_provider::trace4!(|| ($ctx, $event, $a, $b, $c, $d));
    };
}

//-------------------------------------------[.cpp/bitcoin/src/util/trace.h]

#[cfg(not(ENABLE_TRACING))]
#[macro_export] macro_rules! trace0 { 
    ($context:expr, 
     $event:expr) => { } }

#[cfg(not(ENABLE_TRACING))]
#[macro_export] macro_rules! trace1 { 
    ($context:expr, 
     $event:expr, 
     $a:expr) => { } }

#[cfg(not(ENABLE_TRACING))]
#[macro_export] macro_rules! trace2 { 
    ($context:expr, 
     $event:expr, 
     $a:expr, 
     $b:expr) => { } }

#[cfg(not(ENABLE_TRACING))]
#[macro_export] macro_rules! trace3 { 
    ($context:expr, 
     $event:expr, 
     $a:expr, 
     $b:expr, 
     $c:expr) => { } }

#[cfg(not(ENABLE_TRACING))]
#[macro_export] macro_rules! trace4 { 
    ($context:expr, 
     $event:expr, 
     $a:expr, 
     $b:expr, 
     $c:expr, 
     $d:expr) => { } }
