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

#-------------------------------./u/write-remaining

#ACTIVE := bitcoin-aes
ACTIVE := bitcoin-network
ACTIVE := bitcoin-log
ACTIVE := bitcoinleveldb-filter

#ACTIVE := bitcoin-client-ui
#ACTIVE := bitcoin-compat
#ACTIVE := bitcoin-crc32c
#ACTIVE := bitcoin-cuckoo-cache
#ACTIVE := bitcoin-daemon
#ACTIVE := bitcoin-db
#ACTIVE := bitcoin-derive
#ACTIVE := bitcoin-epoch
#ACTIVE := bitcoin-fees
#ACTIVE := bitcoin-fuzz
#ACTIVE := bitcoin-golombrice
#ACTIVE := bitcoin-hash
#ACTIVE := bitcoin-http
#ACTIVE := bitcoin-imports
#ACTIVE := bitcoin-index
#ACTIVE := bitcoin-indirectmap
#ACTIVE := bitcoin-init
#ACTIVE := bitcoin-ipc
#ACTIVE := bitcoin-leveldb
#ACTIVE := bitcoin-merkle
#ACTIVE := bitcoin-miner
#ACTIVE := bitcoin-net
#ACTIVE := bitcoin-node
#ACTIVE := bitcoin-pow
#ACTIVE := bitcoin-primitives
#ACTIVE := bitcoin-qt
#ACTIVE := bitcoin-rpc
#ACTIVE := bitcoin-scheduler
#ACTIVE := bitcoin-script
#ACTIVE := bitcoin-secp256k1
#ACTIVE := bitcoin-serialize
#ACTIVE := bitcoin-service-flags
#ACTIVE := bitcoin-settings
#ACTIVE := bitcoin-signer
#ACTIVE := bitcoin-string
#ACTIVE := bitcoin-support
#ACTIVE := bitcoin-sync
#ACTIVE := bitcoin-test
#ACTIVE := bitcoin-tor
#ACTIVE := bitcoin-txmempool
#ACTIVE := bitcoin-univalue
#ACTIVE := bitcoin-version
#ACTIVE := bitcoin-zmq

#-------------------------------DONE

#DEFAULT := build_active
#DEFAULT := build
DEFAULT := test_active

default: $(DEFAULT)

gen_doc:
	RUSTFLAGS=$(RUSTFLAGS) ./u/generate-rustdoc-db

build:
	$(HACK_CLANG) RUST_BACKTRACE=full RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(BUILD)

build_active:
	$(HACK_CLANG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(BUILD) -p $(ACTIVE) --verbose

test_active:
	$(HACK_CLANG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(TEST) -p $(ACTIVE) --verbose

vendor:
	RUSTFLAGS=$(RUSTFLAGS) $(CARGO) vendor

json:
	$(HACK_CLANG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(BUILD) --quiet --message-format=json 2> /dev/null | jq --slurp

timings:
	$(HACK_CLANG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) +nightly build -Z timings
