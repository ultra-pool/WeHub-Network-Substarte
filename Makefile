PORT_0?=30333
WS_PORT_0?=9945
RPC_PORT_0?=9933

PORT_1?=30334
WS_PORT_1?=9946
RPC_PORT_1?=9934

BASE_PATH_PREFIX?=./tmp-public-chain
KEYS_PATH_PREFIX?=keys
TELEMETRY_URL?='wss://telemetry.polkadot.io/submit/ 0'
BOOT_NODE_PREFIX?=/ip4/$(BOOT_NODE_IP)/tcp/$(PORT_0)/p2p

PUBLIC_CHAIN_SPEC?=./publicChainSpecRaw.json
PUBLIC_RPC_CORS?=all

run:
	cargo run -- --dev --tmp

purge:
	cargo run -- purge-chain --dev -y

restart: purge run

check:
	SKIP_WASM_BUILD=1 cargo check

test:
	SKIP_WASM_BUILD=1 cargo test --all

test-lib:
	SKIP_WASM_BUILD=1 cargo test -p pallet-wehub --lib

build:
	cargo build

start:
	cargo run --release -- --dev --tmp

node-build:
	cargo build --release

keystore-add:
	curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "@keystore.json"

local-node-0-start:
	./target/release/node-template -lruntime=debug \
	--base-path $(BASE_PATH_PREFIX)/node_0 \
	--chain local \
	--port $(PORT_0) \
	--ws-port $(WS_PORT_0) \
	--rpc-port $(RPC_PORT_0) \
	--validator \
	--rpc-methods Unsafe \
	--telemetry-url $(TELEMETRY_URL) \
	--name wehub-node-0 \

local-node-1-start:
	./target/release/node-template -lruntime=debug \
	--base-path $(BASE_PATH_PREFIX)/node_1 \
	--chain local \
	--port $(PORT_1) \
	--ws-port $(WS_PORT_1) \
	--rpc-port $(RPC_PORT_1) \
	--validator \
	--rpc-methods Unsafe \
	--telemetry-url $(TELEMETRY_URL) \
	--name wehub-node-1 \

local-add-all-keys:
	make local-node-0-add-key-aura && \
	make local-node-0-add-key-grandpa && \
	make local-node-0-add-key-whub && \
	make local-node-1-add-key-aura && \
	make local-node-1-add-key-grandpa && \
	make local-node-1-add-key-whub \

local-node-0-add-key-aura:
	curl http://localhost:$(RPC_PORT_0) -H "Content-Type:application/json;charset=utf-8" -d "@$(KEYS_PATH_PREFIX)/local-node-0-aura.json"

local-node-0-add-key-grandpa:
	curl http://localhost:$(RPC_PORT_0) -H "Content-Type:application/json;charset=utf-8" -d "@$(KEYS_PATH_PREFIX)/local-node-0-grandpa.json"

local-node-0-add-key-whub:
	curl http://localhost:$(RPC_PORT_0) -H "Content-Type:application/json;charset=utf-8" -d "@$(KEYS_PATH_PREFIX)/local-node-0-whub.json"

local-node-1-add-key-aura:
	curl http://localhost:$(RPC_PORT_1) -H "Content-Type:application/json;charset=utf-8" -d "@$(KEYS_PATH_PREFIX)/local-node-1-aura.json"

local-node-1-add-key-grandpa:
	curl http://localhost:$(RPC_PORT_1) -H "Content-Type:application/json;charset=utf-8" -d "@$(KEYS_PATH_PREFIX)/local-node-1-grandpa.json"

local-node-1-add-key-whub:
	curl http://localhost:$(RPC_PORT_1) -H "Content-Type:application/json;charset=utf-8" -d "@$(KEYS_PATH_PREFIX)/local-node-1-whub.json"


public-chain-spec:
	./target/release/node-template build-spec --chain=public --raw --disable-default-bootnode > publicChainSpecRaw.json

public-boot-node-start:
	./target/release/node-template -lruntime=debug \
	--base-path $(BASE_PATH_PREFIX)/public \
	--chain $(PUBLIC_CHAIN_SPEC) \
	--port $(PORT_0) \
	--ws-port $(WS_PORT_0) \
	--rpc-port $(RPC_PORT_0) \
	--validator \
	--rpc-cors $(PUBLIC_RPC_CORS) \
	--rpc-methods Unsafe \
	--no-mdns \
	--name whub-public-boot-node \

public-node-start:
	./target/release/node-template -lruntime=debug \
	--base-path $(BASE_PATH_PREFIX)/$(NAME) \
	--chain $(PUBLIC_CHAIN_SPEC) \
	--port $(PORT_0) \
	--ws-port $(WS_PORT_0) \
	--rpc-port $(RPC_PORT_0) \
	--validator \
	--rpc-cors $(PUBLIC_RPC_CORS) \
	--rpc-methods Unsafe \
	--bootnodes $(BOOT_NODE_PREFIX)/$(BOOT_NODE_IDENTITY) \
	--no-mdns \
	--name $(NAME) \

public-node-add-keys:
	curl http://localhost:$(RPC_PORT_0) -H "Content-Type:application/json;charset=utf-8" -d "@$(KEYS_PATH_PREFIX)/private-key-aura.json" && \
	curl http://localhost:$(RPC_PORT_0) -H "Content-Type:application/json;charset=utf-8" -d "@$(KEYS_PATH_PREFIX)/private-key-grandpa.json"