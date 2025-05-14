// ---------------- [ File: bitcoin-log/src/trace.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/trace.h]

#[cfg(ENABLE_TRACING)]
#[macro_export] macro_rules! trace {
    ($context:expr, 
     $event:expr) => {
        /*
                DTRACE_PROBE(context, event)
        */
    }
}

#[cfg(ENABLE_TRACING)]
#[macro_export] macro_rules! trace1 {
    ($context:expr, 
     $event:expr, 
     $a:expr) => {
        /*
                DTRACE_PROBE1(context, event, a)
        */
    }
}

#[cfg(ENABLE_TRACING)]
#[macro_export] macro_rules! trace2 {
    ($context:expr, 
     $event:expr, 
     $a:expr, 
     $b:expr) => {
        /*
                DTRACE_PROBE2(context, event, a, b)
        */
    }
}

#[cfg(ENABLE_TRACING)]
#[macro_export] macro_rules! trace3 {
    ($context:expr, 
     $event:expr, 
     $a:expr, 
     $b:expr, 
     $c:expr) => {
        /*
                DTRACE_PROBE3(context, event, a, b, c)
        */
    }
}

#[cfg(ENABLE_TRACING)]
#[macro_export] macro_rules! trace4 {
    ($context:expr, 
     $event:expr, 
     $a:expr, 
     $b:expr, 
     $c:expr, 
     $d:expr) => {
        /*
                DTRACE_PROBE4(context, event, a, b, c, d)
        */
    }
}

#[cfg(ENABLE_TRACING)]
#[macro_export] macro_rules! trace5 {
    ($context:expr, 
     $event:expr, 
     $a:expr, 
     $b:expr, 
     $c:expr, 
     $d:expr, 
     $e:expr) => {
        /*
                DTRACE_PROBE5(context, event, a, b, c, d, e)
        */
    }
}

#[cfg(ENABLE_TRACING)]
#[macro_export] macro_rules! trace6 {
    ($context:expr, 
     $event:expr, 
     $a:expr, 
     $b:expr, 
     $c:expr, 
     $d:expr, 
     $e:expr, 
     $f:expr) => {
        /*
                DTRACE_PROBE6(context, event, a, b, c, d, e, f)
        */
    }
}

#[cfg(ENABLE_TRACING)]
#[macro_export] macro_rules! trace7 {
    ($context:expr, 
     $event:expr, 
     $a:expr, 
     $b:expr, 
     $c:expr, 
     $d:expr, 
     $e:expr, 
     $f:expr, 
     $g:expr) => {
        /*
                DTRACE_PROBE7(context, event, a, b, c, d, e, f, g)
        */
    }
}

#[cfg(ENABLE_TRACING)]
#[macro_export] macro_rules! trace8 {
    ($context:expr, 
     $event:expr, 
     $a:expr, 
     $b:expr, 
     $c:expr, 
     $d:expr, 
     $e:expr, 
     $f:expr, 
     $g:expr, 
     $h:expr) => {
        /*
                DTRACE_PROBE8(context, event, a, b, c, d, e, f, g, h)
        */
    }
}

#[cfg(ENABLE_TRACING)]
#[macro_export] macro_rules! trace9 {
    ($context:expr, 
     $event:expr, 
     $a:expr, 
     $b:expr, 
     $c:expr, 
     $d:expr, 
     $e:expr, 
     $f:expr, 
     $g:expr, 
     $h:expr, 
     $i:expr) => {
        /*
                DTRACE_PROBE9(context, event, a, b, c, d, e, f, g, h, i)
        */
    }
}

#[cfg(ENABLE_TRACING)]
#[macro_export] macro_rules! trace10 {
    ($context:expr, 
     $event:expr, 
     $a:expr, 
     $b:expr, 
     $c:expr, 
     $d:expr, 
     $e:expr, 
     $f:expr, 
     $g:expr, 
     $h:expr, 
     $i:expr, 
     $j:expr) => {
        /*
                DTRACE_PROBE10(context, event, a, b, c, d, e, f, g, h, i, j)
        */
    }
}

#[cfg(ENABLE_TRACING)]
#[macro_export] macro_rules! trace11 {
    ($context:expr, 
     $event:expr, 
     $a:expr, 
     $b:expr, 
     $c:expr, 
     $d:expr, 
     $e:expr, 
     $f:expr, 
     $g:expr, 
     $h:expr, 
     $i:expr, 
     $j:expr, 
     $k:expr) => {
        /*
                DTRACE_PROBE11(context, event, a, b, c, d, e, f, g, h, i, j, k)
        */
    }
}

#[cfg(ENABLE_TRACING)]
#[macro_export] macro_rules! trace12 {
    ($context:expr, 
     $event:expr, 
     $a:expr, 
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
     $l:expr) => {
        /*
                DTRACE_PROBE12(context, event, a, b, c, d, e, f, g, h, i, j, k, l)
        */
    }
}


#[cfg(not(ENABLE_TRACING))]
#[macro_export] macro_rules! trace { 
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

#[cfg(not(ENABLE_TRACING))]
#[macro_export] macro_rules! trace5 { 
    ($context:expr, 
     $event:expr, 
     $a:expr, 
     $b:expr, 
     $c:expr, 
     $d:expr, 
     $e:expr) => { } }

#[cfg(not(ENABLE_TRACING))]
#[macro_export] macro_rules! trace6 { 
    ($context:expr, 
     $event:expr, 
     $a:expr, 
     $b:expr, 
     $c:expr, 
     $d:expr, 
     $e:expr, 
     $f:expr) => { } }

#[cfg(not(ENABLE_TRACING))]
#[macro_export] macro_rules! trace7 { 
    ($context:expr, 
     $event:expr, 
     $a:expr, 
     $b:expr, 
     $c:expr, 
     $d:expr, 
     $e:expr, 
     $f:expr, 
     $g:expr) => { } }

#[cfg(not(ENABLE_TRACING))]
#[macro_export] macro_rules! trace8 { 
    ($context:expr, 
     $event:expr, 
     $a:expr, 
     $b:expr, 
     $c:expr, 
     $d:expr, 
     $e:expr, 
     $f:expr, 
     $g:expr, 
     $h:expr) => { } }

#[cfg(not(ENABLE_TRACING))]
#[macro_export] macro_rules! trace9 { 
    ($context:expr, 
     $event:expr, 
     $a:expr, 
     $b:expr, 
     $c:expr, 
     $d:expr, 
     $e:expr, 
     $f:expr, 
     $g:expr, 
     $h:expr, 
     $i:expr) => { } }

#[cfg(not(ENABLE_TRACING))]
#[macro_export] macro_rules! trace10 { 
    ($context:expr, 
     $event:expr, 
     $a:expr, 
     $b:expr, 
     $c:expr, 
     $d:expr, 
     $e:expr, 
     $f:expr, 
     $g:expr, 
     $h:expr, 
     $i:expr, 
     $j:expr) => { } }

#[cfg(not(ENABLE_TRACING))]
#[macro_export] macro_rules! trace11 { 
    ($context:expr, 
     $event:expr, 
     $a:expr, 
     $b:expr, 
     $c:expr, 
     $d:expr, 
     $e:expr, 
     $f:expr, 
     $g:expr, 
     $h:expr, 
     $i:expr, 
     $j:expr, 
     $k:expr) => { } }

#[cfg(not(ENABLE_TRACING))]
#[macro_export] macro_rules! trace12 { 
    ($context:expr, 
     $event:expr, 
     $a:expr, 
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
     $l:expr) => { } }
