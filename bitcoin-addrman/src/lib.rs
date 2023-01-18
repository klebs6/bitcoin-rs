#![feature(core_intrinsics)]

#[macro_use] mod imports; use imports::*;

x!{add}
x!{load}
x!{read_from_stream}
x!{addrman}
x!{addr}
x!{attempt}
x!{check}
x!{clear}
x!{config}
x!{connected}
x!{create}
x!{delete}
x!{find}
x!{format}
x!{get}
x!{good}
x!{info}
x!{inner}
x!{make_tried}
x!{pimpl} //can we avoid this idiom?
x!{resolve}
x!{select}
x!{select_tried_collision}
x!{set_services}
x!{swap}
x!{serialize}
x!{deserialize}
