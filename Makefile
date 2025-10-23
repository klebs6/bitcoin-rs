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

#DEFAULT := build_active
#DEFAULT := build
#DEFAULT := hack_test
#DEFAULT := test_one_release
DEFAULT := test_active
#DEFAULT := test_one
#DEFAULT := test_ignored
#DEFAULT := test_one_ignored

FEATURES := --features "natpmp"
FEATURES := 

#-------------------------------[active-below]

#ACTIVE := bitcoin-get-json-token # note that this one has a failing test
ACTIVE := bitcoin-settings

ACTIVE := bitcoin-top
ACTIVE := bitcoin-random
#ACTIVE := bitcoin-block
#ACTIVE := bitcoin-network
#ACTIVE := bitcoin-remote
#ACTIVE := bitcoin-db
#ACTIVE := bitcoin-argsman

#ACTIVE := bitcoin-base58
#ACTIVE := bitcoin-muhash
#ACTIVE := bitcoin-hash

#-------------------------------[next]
#ACTIVE := bitcoinleveldb-arena
#ACTIVE := bitcoinleveldb-batch
#ACTIVE := bitcoinleveldb-bench
#ACTIVE := bitcoinleveldb-bloom
#ACTIVE := bitcoinleveldb-cache
#ACTIVE := bitcoinleveldb-cfg
#ACTIVE := bitcoinleveldb-coding
#ACTIVE := bitcoinleveldb-comparator
#ACTIVE := bitcoinleveldb-compat
#ACTIVE := bitcoinleveldb-crc32
#ACTIVE := bitcoinleveldb-db
#ACTIVE := bitcoinleveldb-dumpfile
#ACTIVE := bitcoinleveldb-duplex
#ACTIVE := bitcoinleveldb-env
#ACTIVE := bitcoinleveldb-file
#ACTIVE := bitcoinleveldb-filter
#ACTIVE := bitcoinleveldb-hash
#ACTIVE := bitcoinleveldb-histogram
#ACTIVE := bitcoinleveldb-key
#ACTIVE := bitcoinleveldb-limiter
#ACTIVE := bitcoinleveldb-log
#ACTIVE := bitcoinleveldb-lru
#ACTIVE := bitcoinleveldb-memenv
#ACTIVE := bitcoinleveldb-memtable
#ACTIVE := bitcoinleveldb-merger
#ACTIVE := bitcoinleveldb-meta
#ACTIVE := bitcoinleveldb-options
#ACTIVE := bitcoinleveldb-posix
#ACTIVE := bitcoinleveldb-rand
#ACTIVE := bitcoinleveldb-repair
#ACTIVE := bitcoinleveldb-skiplist
#ACTIVE := bitcoinleveldb-slice
#ACTIVE := bitcoinleveldb-snapshot
#ACTIVE := bitcoinleveldb-status
#ACTIVE := bitcoinleveldb-sync
#ACTIVE := bitcoinleveldb-table
#ACTIVE := bitcoinleveldb-test
#ACTIVE := bitcoinleveldb-util
#ACTIVE := bitcoinleveldb-version
#ACTIVE := bitcoinleveldb-versionedit

#-------------------------------[future]
#ACTIVE := bitcoin-portmap
#ACTIVE := bitcoinsecp256k1-keys
#ACTIVE := bitcoin-scripting
#ACTIVE := bitcoin-key
#ACTIVE := bitcoin-bdb
#ACTIVE := bitcoin-addr
#ACTIVE := bitcoin-addrman
#ACTIVE := bitcoin-argsman
#ACTIVE := bitcoin-banman
#ACTIVE := bitcoin-bench
#ACTIVE := bitcoin-blockencoding
#ACTIVE := bitcoin-blockfilter
#ACTIVE := bitcoin-blockman
#ACTIVE := bitcoin-blockpolicy
#ACTIVE := bitcoin-bloom
#ACTIVE := bitcoin-cfg
#ACTIVE := bitcoin-chain-consensus
#ACTIVE := bitcoin-chainman
#ACTIVE := bitcoin-checkqueue
#ACTIVE := bitcoin-cli
#ACTIVE := bitcoin-client-ui
#ACTIVE := bitcoin-coincontrol
#ACTIVE := bitcoin-coinselect
#ACTIVE := bitcoin-coinsview
#ACTIVE := bitcoin-compressor
#ACTIVE := bitcoin-connman
#ACTIVE := bitcoin-crypter
#ACTIVE := bitcoin-daemon
#ACTIVE := bitcoin-db
#ACTIVE := bitcoin-deployment
#ACTIVE := bitcoin-derive
#ACTIVE := bitcoin-dns
#ACTIVE := bitcoin-dummywallet
#ACTIVE := bitcoin-dumpwallet
#ACTIVE := bitcoin-foundblock
#ACTIVE := bitcoin-fuzz
#ACTIVE := bitcoin-hdchain
#ACTIVE := bitcoin-http
#ACTIVE := bitcoin-imports
#ACTIVE := bitcoin-index
#ACTIVE := bitcoin-indexed-chain
#ACTIVE := bitcoin-init
#ACTIVE := bitcoin-ipc
#ACTIVE := bitcoin-leveldb
#ACTIVE := bitcoin-mainsignals
#ACTIVE := bitcoin-mem
#ACTIVE := bitcoin-merkle
#ACTIVE := bitcoin-message
#ACTIVE := bitcoin-miner
#ACTIVE := bitcoin-net
#ACTIVE := bitcoin-net-zmq
#ACTIVE := bitcoin-netmsg
#ACTIVE := bitcoin-netpermissions
#ACTIVE := bitcoin-node
#ACTIVE := bitcoin-noui
#ACTIVE := bitcoin-packages
#ACTIVE := bitcoin-peerman
#ACTIVE := bitcoin-policy
#ACTIVE := bitcoin-pow
#ACTIVE := bitcoin-proxy
#ACTIVE := bitcoin-psbt
#ACTIVE := bitcoin-qt
#ACTIVE := bitcoin-rbf
#ACTIVE := bitcoin-restapi
#ACTIVE := bitcoin-sam
#ACTIVE := bitcoin-scheduler
#ACTIVE := bitcoin-scriptpubkeyman
#ACTIVE := bitcoin-secp256k1
#ACTIVE := bitcoin-signet
#ACTIVE := bitcoin-signingprovider
#ACTIVE := bitcoin-sqlite
#ACTIVE := bitcoin-subnet
#ACTIVE := bitcoin-system
#ACTIVE := bitcoin-test
#ACTIVE := bitcoin-top
#ACTIVE := bitcoin-tor
#ACTIVE := bitcoin-tx
#ACTIVE := bitcoin-txmempool
#ACTIVE := bitcoin-txmempoolentry
#ACTIVE := bitcoin-validation
#ACTIVE := bitcoin-walletdb
#ACTIVE := bitcoinchain-client
#ACTIVE := bitcoinchain-interface
#ACTIVE := bitcoinchain-notifications
#ACTIVE := bitcoinchain-params
#ACTIVE := bitcoinnode-interface
#ACTIVE := bitcoinnode-stats
#ACTIVE := bitcoinnode-txrelay
#ACTIVE := bitcoinrpc-blockchain
#ACTIVE := bitcoinrpc-dump
#ACTIVE := bitcoinrpc-mining
#ACTIVE := bitcoinrpc-misc
#ACTIVE := bitcoinrpc-net
#ACTIVE := bitcoinrpc-server
#ACTIVE := bitcoinrpc-txn
#ACTIVE := bitcoinrpc-util
#ACTIVE := bitcoinrpc-wallet
#ACTIVE := bitcoinsecp256k1-bench
#ACTIVE := bitcoinsecp256k1-ec
#ACTIVE := bitcoinsecp256k1-field
#ACTIVE := bitcoinsecp256k1-group
#ACTIVE := bitcoinsecp256k1-modinv
#ACTIVE := bitcoinsecp256k1-parse
#ACTIVE := bitcoinsecp256k1-recovery
#ACTIVE := bitcoinsecp256k1-scalar
#ACTIVE := bitcoinsecp256k1-schnorr
#ACTIVE := bitcoinsecp256k1-scratch
#ACTIVE := bitcoinwallet-client
#ACTIVE := bitcoinwallet-context
#ACTIVE := bitcoinwallet-feature
#ACTIVE := bitcoinwallet-fees
#ACTIVE := bitcoinwallet-init
#ACTIVE := bitcoinwallet-interface
#ACTIVE := bitcoinwallet-library
#ACTIVE := bitcoinwallet-receive
#ACTIVE := bitcoinwallet-salvage
#ACTIVE := bitcoinwallet-spend

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
	$(HACK_CLANG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(TEST) -p $(ACTIVE) --verbose $(FEATURES)

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
#----------------------------------------------[done-but-uninstalled]
#ACTIVE := bitcoin-sha256
#ACTIVE := bitcoin-sha3
#ACTIVE := bitcoin-sha512
#ACTIVE := bitcoin-hmac-sha512
#ACTIVE := bitcoin-hmac-sha256
#ACTIVE := bitcoin-sock
#ACTIVE := bitcoin-tx-confirm-stats
#ACTIVE := bitcoin-fees
