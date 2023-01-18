#![allow(soft_unstable)]
#![feature(test)]

#[macro_use] mod imports; use imports::*;

x!{bench_addrman}
x!{bench_base58}
x!{bench_bech32}
x!{bench_bench_bitcoin}
x!{bench_block_assemble}
x!{bench_ccoins_caching}
x!{bench_chacha_poly_aead}
x!{bench_chacha20}
x!{bench_checkblock}
x!{bench_checkqueue}
x!{bench_coin_selection}
x!{bench_crypto_hash}
x!{bench_data}
x!{bench_duplicate_inputs}
x!{bench_examples}
x!{bench_gcs_filter}
x!{bench_hashpadding}
x!{bench_lockedpool}
x!{bench_mempool_eviction}
x!{bench_mempool_stress}
x!{bench_merkle_root}
x!{bench_peer_eviction}
x!{bench_poly1305}
x!{bench_prevector}
x!{bench_rollingbloom}
x!{bench_rpc_blockchain}
x!{bench_rpc_mempool}
x!{bench_util_time}
x!{bench_verify_script}
x!{bench_wallet_balance}
x!{bench}
