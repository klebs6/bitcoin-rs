.PHONY: build vendor json

export CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG=true

HACK_BDB   := env CFLAGS="-Wno-error=implicit-function-declaration"
HACK_LLVM  := env LLVM_CONFIG_PATH="/usr/local/opt/llvm/bin/llvm-config"

HACK_CLANG := env LD_LIBRARY_PATH=/usr/local/opt/llvm/lib/ 

HACK_CLANG := 

RUSTFLAGS := -Awarnings
CARGO     := env CARGO_MSG_LIMIT=15 \
			 CARGO_BUILD_JOBS=12 \
			 NUM_JOBS=12 \
			 cargo 

#CARGO    := env CARGO_BUILD_JOBS=12 NUM_JOBS=12 cargo 

#CARGO_MSG_LIMIT := 100
BUILD := build --verbose
TEST  := test
BENCH := bench

RUST_LOG := debug
RUST_LOG := bitcoinleveldb_crc32=off,debug

#DEFAULT := hack_test
#DEFAULT := test_one_release
DEFAULT := test_active
#DEFAULT := build
#DEFAULT := build_active
#DEFAULT := test_one
#DEFAULT := test_ignored
#DEFAULT := test_one_ignored

FEATURES := --features "leveldb_snappy"
FEATURES := 

NO_FAIL_FAST := --no-fail-fast

#-------------------------------[active-below]
ACTIVE := bitcoin-top

# ---[leveldb-layer-1]
ACTIVE := bitcoinleveldb-repair
ACTIVE := bitcoinleveldb-modeldb
ACTIVE := bitcoinleveldb-dbconstructor
ACTIVE := bitcoinleveldb-harness
ACTIVE := bitcoinleveldb-dbimplwriter
ACTIVE := bitcoinleveldb-dbinterface

# ---[leveldb-layer-2]
#ACTIVE := bitcoinleveldb-dbtest
#ACTIVE := bitcoinleveldb-db
#ACTIVE := bitcoinleveldb-dbimpl
#ACTIVE := bitcoinleveldb-dbiter

# ---[leveldb-layer-3]
#ACTIVE := bitcoinleveldb-bench    #loc: 3003
#ACTIVE := bitcoinleveldb-test     #loc: 3261
#ACTIVE := bitcoin-leveldb         #loc: 37

#----------------------------------------
# ---[secp-layer-0]
#ACTIVE := bitcoinsecp256k1-modinv
#ACTIVE := bitcoinsecp256k1-scratch
# ---[secp-layer-1]
#ACTIVE := bitcoinsecp256k1-field
#ACTIVE := bitcoinsecp256k1-scalar
# ---[secp-layer-2]
#ACTIVE := bitcoinsecp256k1-group
# ---[secp-layer-3]
#ACTIVE := bitcoinsecp256k1-ec
# ---[secp-layer-4]
#ACTIVE := bitcoin-secp256k1
# ---[secp-layer-5]
#ACTIVE := bitcoinsecp256k1-keys
#ACTIVE := bitcoinsecp256k1-parse
#ACTIVE := bitcoinsecp256k1-recovery
# ---[secp-layer-6]
#ACTIVE := bitcoinsecp256k1-bench
#ACTIVE := bitcoinsecp256k1-schnorr

#----------------------------------------
# ---[layer 0]
#ACTIVE := bitcoin-checkqueue
#ACTIVE := bitcoin-scheduler
#ACTIVE := bitcoin-validation

# ---[layer 1]
#ACTIVE := bitcoin-subnet

# ---[layer 2]
#ACTIVE := bitcoin-addr
#ACTIVE := bitcoin-netpermissions
#ACTIVE := bitcoin-proxy
#ACTIVE := bitcoin-sam

# ---[layer 3]
#ACTIVE := bitcoin-dns

# ---[layer 4]
#ACTIVE := bitcoin-key

# ---[layer 5]
#ACTIVE := bitcoin-hdchain
#ACTIVE := bitcoin-message

# ---[layer 6]
#ACTIVE := bitcoin-crypter

# ---[layer 7]
#ACTIVE := bitcoin-scripting

# ---[layer 8]
#ACTIVE := bitcoin-compressor
#ACTIVE := bitcoin-netmsg
#ACTIVE := bitcoin-tx

# ---[layer 9]
#ACTIVE := bitcoin-block
#ACTIVE := bitcoin-bloom
#ACTIVE := bitcoin-noui
#ACTIVE := bitcoin-rbf

# ---[layer 10]
#ACTIVE := bitcoin-blockencoding
#ACTIVE := bitcoin-chain-consensus
#ACTIVE := bitcoin-client-ui
#ACTIVE := bitcoin-foundblock
#ACTIVE := bitcoin-merkle
#ACTIVE := bitcoin-txmempoolentry
#ACTIVE := bitcoinnode-txrelay

# ---[layer 11]
#ACTIVE := bitcoin-blockpolicy
#ACTIVE := bitcoin-deployment
#ACTIVE := bitcoin-pow
#ACTIVE := bitcoin-signet

# ---[layer 12]
#ACTIVE := bitcoin-db

# ---[layer 13]
#ACTIVE := bitcoin-coinsview

# ---[layer 14]
#ACTIVE := bitcoin-policy
#ACTIVE := bitcoin-signingprovider
#ACTIVE := bitcoinchain-params

# ---[layer 15]
#ACTIVE := bitcoin-blockman
#ACTIVE := bitcoin-net
#ACTIVE := bitcoin-packages
#ACTIVE := bitcoin-portmap
#ACTIVE := bitcoin-psbt

# ---[layer 16]
#ACTIVE := bitcoin-addrman
#ACTIVE := bitcoin-banman
#ACTIVE := bitcoin-tor
#ACTIVE := bitcoin-txmempool
#ACTIVE := bitcoinnode-stats

# ---[layer 17]
#ACTIVE := bitcoin-system

# ---[layer 18]
#ACTIVE := bitcoin-scriptpubkeyman

# ---[layer 19]
#ACTIVE := bitcoin-bdb
#ACTIVE := bitcoin-chainman
#ACTIVE := bitcoin-index
#ACTIVE := bitcoin-sqlite

# ---[layer 20]
#ACTIVE := bitcoin-blockfilter
#ACTIVE := bitcoin-coincontrol
#ACTIVE := bitcoin-ipc

# ---[wallet layer 0]
#ACTIVE := bitcoinwallet-salvage
#ACTIVE := bitcoinwallet-feature

# ---[wallet layer 1]
#ACTIVE := bitcoinwallet-interface

# ---[wallet layer 2]
#ACTIVE := bitcoin-walletdb

# ---[wallet layer 3]
#ACTIVE := bitcoinwallet-context

# ---[wallet layer 4]
#ACTIVE := bitcoinwallet-client

# ---[wallet layer 5]
#ACTIVE := bitcoinwallet-library

# ---[wallet layer 6]
#ACTIVE := bitcoinwallet-fees
#ACTIVE := bitcoinwallet-init
#ACTIVE := bitcoinwallet-receive
#ACTIVE := bitcoinwallet-spend

# ---[layer 21]
#ACTIVE := bitcoinnode-interface

# ---[layer 22]
#ACTIVE := bitcoin-connman
#ACTIVE := bitcoin-node

# ---[layer 23]
#ACTIVE := bitcoin-http
#ACTIVE := bitcoin-peerman

# ---[layer 24]
#ACTIVE := bitcoin-indexed-chain

# ---[layer 25]
#ACTIVE := bitcoin-coinselect
#ACTIVE := bitcoin-init
#ACTIVE := bitcoin-mainsignals
#ACTIVE := bitcoin-miner
#ACTIVE := bitcoin-net-zmq
#ACTIVE := bitcoin-restapi

# ---[layer 26]
#ACTIVE := bitcoinrpc-util
#ACTIVE := bitcoinrpc-server
#ACTIVE := bitcoinrpc-blockchain
#ACTIVE := bitcoinrpc-mining
#ACTIVE := bitcoinrpc-misc
#ACTIVE := bitcoinrpc-net
#ACTIVE := bitcoinrpc-txn
#ACTIVE := bitcoinrpc-dump
#ACTIVE := bitcoinrpc-wallet

# ---[layer 27]
#ACTIVE := bitcoin-cli
#ACTIVE := bitcoin-test
#ACTIVE := bitcoin-dummywallet
#ACTIVE := bitcoin-dumpwallet

# ---[layer 28]
#ACTIVE := bitcoin-bench
#ACTIVE := bitcoin-daemon
#ACTIVE := bitcoin-fuzz

# ---[layer 29]
#ACTIVE := bitcoin-top

#-------------------------------DONE

INDIVIDUAL_TEST := propagate_26bit_carries_once
INDIVIDUAL_TEST := poly1305
INDIVIDUAL_TEST := final_carry_and_sub_p
INDIVIDUAL_TEST := decrypt_matches_reference_aes128
INDIVIDUAL_TEST := load_byte_validation
INDIVIDUAL_TEST := load_byte
INDIVIDUAL_TEST := save_byte
INDIVIDUAL_TEST := shift_row
INDIVIDUAL_TEST := aes_setup_round_key_validation
INDIVIDUAL_TEST := compute_g_plus5_minus_p
INDIVIDUAL_TEST := populate_round_zero
INDIVIDUAL_TEST := sha256_round

default: $(DEFAULT)

gen_doc:
	RUSTFLAGS=$(RUSTFLAGS) ./u/generate-rustdoc-db

build:
	$(HACK_CLANG) RUST_BACKTRACE=full RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(BUILD)

build_active:
	$(HACK_CLANG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(BUILD) -p $(ACTIVE) --verbose

test_active:
	$(HACK_CLANG) RUST_LOG=$(RUST_LOG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(TEST) -p $(ACTIVE) --verbose $(FEATURES) $(NO_FAIL_FAST)

test_ignored:
	$(HACK_CLANG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(TEST) --release -p $(ACTIVE) --verbose $(FEATURES) -- --ignored

test_one:
	RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(TEST) $(INDIVIDUAL_TEST) -p $(ACTIVE) -- $(NOCAPTURE)

test_one_ignored:
	$(HACK_CLANG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(TEST) $(INDIVIDUAL_TEST) --release -p $(ACTIVE) --verbose $(FEATURES) -- --ignored

test_one_release:
	RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(TEST) --release $(INDIVIDUAL_TEST) -p $(ACTIVE) -- --ignored $(NOCAPTURE)

hack_test:
	RUSTFLAGS=$(RUSTFLAGS) cargo hack test --feature-powerset -p $(ACTIVE) --verbose -- --test-threads 1

vendor:
	RUSTFLAGS=$(RUSTFLAGS) $(CARGO) vendor

json:
	$(HACK_CLANG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(BUILD) --quiet --message-format=json 2> /dev/null | jq --slurp

timings:
	$(HACK_CLANG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) +nightly build -Z timings

bench:
	RUSTFLAGS="-Awarnings -C target-cpu=native" $(CARGO) $(BENCH) -p $(ACTIVE)

#-------------------------------[done-below]
#ACTIVE := bitcoin-amt
#ACTIVE := bitcoin-arena
#ACTIVE := bitcoin-autofile
#ACTIVE := bitcoin-bech32m
#ACTIVE := bitcoin-bitstream
#ACTIVE := bitcoin-blob
#ACTIVE := bitcoin-bufferedfile
#ACTIVE := bitcoin-locked-page-allocator
#ACTIVE := bitcoin-log
#ACTIVE := bitcoin-mem
#ACTIVE := bitcoin-serialize
#ACTIVE := bitcoin-service
#ACTIVE := bitcoin-sha1
#ACTIVE := bitcoin-string
#ACTIVE := bitcoin-support
#ACTIVE := bitcoin-time
#ACTIVE := bitcoin-u160
#ACTIVE := bitcoin-u256
#ACTIVE := bitcoin-vectorstream
#ACTIVE := bitcoin-aes
#ACTIVE := bitcoin-epoch
#ACTIVE := bitcoin-asmap
#ACTIVE := bitcoin-indirectmap
#ACTIVE := bitcoin-cuckoo-cache
#ACTIVE := bitcoin-golombrice
#ACTIVE := bitcoin-poly1305
#ACTIVE := bitcoin-locked-pool
#ACTIVE := bitcoin-chacha
#ACTIVE := bitcoin-sync
#ACTIVE := bitcoin-compat
#ACTIVE := bitcoin-ripemd
#ACTIVE := bitcoin-siphash
#ACTIVE := bitcoin-service-flags
#ACTIVE := bitcoin-syscall
#ACTIVE := bitcoin-crc32c
#ACTIVE := bitcoin-version
#ACTIVE := bitcoin-univalue       #note this one has a failing test
#ACTIVE := bitcoin-tokenpipe
#ACTIVE := bitcoin-random
#ACTIVE := bitcoin-tx-confirm-stats
#ACTIVE := bitcoin-sha256
#ACTIVE := bitcoin-sha256-sse41
#ACTIVE := bitcoin-sha256-hkdf
#ACTIVE := bitcoin-argsman
#ACTIVE := bitcoin-settings
#ACTIVE := bitcoin-base58
#ACTIVE := bitcoin-get-json-token # note that this one has a failing test
#ACTIVE := bitcoin-hash
#----------------------------------------------[done-but-uninstalled]
#ACTIVE := bitcoin-sha3
#ACTIVE := bitcoin-sha512
#ACTIVE := bitcoin-hmac-sha512
#ACTIVE := bitcoin-hmac-sha256
#ACTIVE := bitcoin-sock
#ACTIVE := bitcoin-fees
#ACTIVE := bitcoin-muhash
#ACTIVE := bitcoin-remote
#ACTIVE := bitcoin-network

#ACTIVE := bitcoinleveldb-arena          #loc: 371
#ACTIVE := bitcoinleveldb-cfg            #loc: 138
#ACTIVE := bitcoinleveldb-compat         #loc: 852
#ACTIVE := bitcoinleveldb-slice          #loc: 318
#ACTIVE := bitcoinleveldb-crc32          #loc: 640
#ACTIVE := bitcoinleveldb-hash           #loc: 111
#ACTIVE := bitcoinleveldb-histogram      #loc: 745
#ACTIVE := bitcoinleveldb-limiter        #loc: 151
#ACTIVE := bitcoinleveldb-rand           #loc: 179
#ACTIVE := bitcoinleveldb-sync           #loc: 60
#ACTIVE := bitcoinleveldb-status         #loc: 390
#ACTIVE := bitcoinleveldb-coding         #loc: 782
#ACTIVE := bitcoinleveldb-comparator     #loc: 185
#ACTIVE := bitcoinleveldb-filter         #loc: 521
#ACTIVE := bitcoinleveldb-cache          #loc: 999
#ACTIVE := bitcoinleveldb-util           #loc: 135
#ACTIVE := bitcoinleveldb-bloom          #loc: 989
#ACTIVE := bitcoinleveldb-key            #loc: 1953
#ACTIVE := bitcoinleveldb-snapshot       #loc: 842
#ACTIVE := bitcoinleveldb-lru            #loc: 4192
#ACTIVE := bitcoinleveldb-skiplist       #loc: 1045
#ACTIVE := bitcoinleveldb-logtools       #loc: 1198
#ACTIVE := bitcoinleveldb-logwriter      #loc: 1656
#ACTIVE := bitcoinleveldb-logreader      #loc: 2026
#ACTIVE := bitcoinleveldb-log            #loc: 8
#ACTIVE := bitcoinleveldb-versionedit    #loc: 1126
#ACTIVE := bitcoinleveldb-env            #loc: 853
#ACTIVE := bitcoinleveldb-posixtools     #loc: 495
#ACTIVE := bitcoinleveldb-posixwfile     #loc: 607
#ACTIVE := bitcoinleveldb-posixseqfile   #loc: 224
#ACTIVE := bitcoinleveldb-posixrafile    #loc: 382
#ACTIVE := bitcoinleveldb-posixlogger    #loc: 2080
#ACTIVE := bitcoinleveldb-posixmmaprfile #loc: 223
#ACTIVE := bitcoinleveldb-posixenv       #loc: 3437
#ACTIVE := bitcoinleveldb-posix          #loc: 632
#ACTIVE := bitcoinleveldb-options        #loc: 339
#ACTIVE := bitcoinleveldb-memenv         #loc: 3569
#ACTIVE := bitcoinleveldb-stringsource
#ACTIVE := bitcoinleveldb-stringsink
#ACTIVE := bitcoinleveldb-reversekeycomparator
#ACTIVE := bitcoinleveldb-blockutil
#ACTIVE := bitcoinleveldb-blockbuilder
#ACTIVE := bitcoinleveldb-block
#ACTIVE := bitcoinleveldb-blockhandle
#ACTIVE := bitcoinleveldb-tablerep
#ACTIVE := bitcoinleveldb-blockconstructor
#ACTIVE := bitcoinleveldb-blockiter
#ACTIVE := bitcoinleveldb-snapshot
#ACTIVE := bitcoinleveldb-tablebuilder
#ACTIVE := bitcoinleveldb-iteratorinner
#ACTIVE := bitcoinleveldb-iterator
#ACTIVE := bitcoinleveldb-file           #loc: 843
#ACTIVE := bitcoinleveldb-merger
#ACTIVE := bitcoinleveldb-keyconvertingiterator
#ACTIVE := bitcoinleveldb-duplex
#ACTIVE := bitcoinleveldb-versioniterator
#ACTIVE := bitcoinleveldb-table
#ACTIVE := bitcoinleveldb-versionsetinterface
#ACTIVE := bitcoinleveldb-compactionstats
#ACTIVE := bitcoinleveldb-emptyiterator
#ACTIVE := bitcoinleveldb-erroriterator
#ACTIVE := bitcoinleveldb-footer
#ACTIVE := bitcoinleveldb-versionsetutil
#ACTIVE := bitcoinleveldb-tableconstructor
#ACTIVE := bitcoinleveldb-tablecache
#ACTIVE := bitcoinleveldb-compaction
#ACTIVE := bitcoinchain-client
#ACTIVE := bitcoinchain-interface
#ACTIVE := bitcoinchain-notifications
#ACTIVE := bitcoinleveldb-memtable
#ACTIVE := bitcoinleveldb-mockversionset
#ACTIVE := bitcoinleveldb-version
#ACTIVE := bitcoinleveldb-batch
#ACTIVE := bitcoinleveldb-specialenv
#ACTIVE := bitcoinleveldb-versionset
#ACTIVE := bitcoinleveldb-dumpfile
#ACTIVE := bitcoinleveldb-testenv
