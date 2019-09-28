# play

# My custom blockchain

# Table of contents

  * [Quick start basic usage](#chapter-f85d7f)
  * [Create blockchain from template with latest dependencies](#chapter-5f0881)
  * [Update and maintain blockchain with latest dependencies](#chapter-e16e68)
  * [Create custom blockchain configuration](#chapter-b1b53c)
  * [Run multiple node PoA testnet using custom blockchain configuration](#chapter-f21efd)
  * [Creating new custom runtime module](#chapter-7f4be8)
  * [Import a template SRML module](#chapter-352eaa)
  * [Interact with blockchain using UI](#chapter-6d9058)
  * [Create custom blockchain UI](#chapter-1c48d9)
  * [Interact with blockchain using CLI](#chapter-9c32f0)
  * [Other resources](#chapter-7af38c)

Note: Generate a new chapter with `openssl rand -hex 3`

## Quick start basic usage <a id="chapter-f85d7f"></a>

* Run the play Substrate-based runtime as described here: [play-node README](./play-node/README.md)
* Interact with it using [Polkadot.js Apps](https://polkadot.js.org/apps)
* Interact with it using custom Polkadot.js API scripts as described here: [play-js README](./play-node/README.md)

## Create blockchain from template <a id="chapter-5f0881"></a>

### Create blockchain from template

Create a Substrate SRML-based blockchain.

* Install Rust. Obtain and setup blockchain template

```bash
curl https://getsubstrate.io -sSf | bash
git clone https://github.com/shawntabrizi/substrate-package
cd substrate-package
git submodule update --init
./substrate-package-rename.sh my-node "My Org"
mv my-node ../ &&
mv substrate-module-template/ ../
```

### Build blockchain 

* Install required tools and dependencies
* Build the WebAssembly binary from all code

```bash
cd .. && \
cd my-node && \
./scripts/init.sh && \
./scripts/build.sh && \
cargo build --release
```

### Save changes with Git

```
git add . && \
git commit -m "Initial node template" && \
```

### Run blockchain node (full node)

* Connect to development testnet (`--chain development`)
```bash
./target/release/my-node --dev
```

### References

* https://github.com/substrate-developer-hub/substrate-package
* https://substrate.dev/substrate-collectables-workshop/#/0/running-a-custom-node
* https://github.com/substrate-developer-hub/substrate-module-template
* https://github.com/paritytech/substrate-up
* https://github.com/paritytech/substrate#51-on-mac-and-ubuntu

## Update and maintain blockchain with latest dependencies <a id="chapter-e16e68"></a>

* Run with detailed logs

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/my-node --dev
```

* Add new runtime module

```bash
cd my-node && \
substrate-module-new <module-name> <author>
```

* Update Rust dependencies

```bash
./scripts/init.sh
```

* Re-build runtime (after changes). Purge chain of all blocks.

```bash
./target/release/my-node purge-chain --dev

./scripts/build.sh
cargo build --release
```

## Create custom blockchain configuration <a id="chapter-b1b53c"></a>

### Production

* Create latest chain specification code changes of local chain

```bash
mkdir -p ./src/chain-spec-templates
./target/release/my-node build-spec \
  --chain=local > ./src/chain-spec-templates/chainspec_latest.json
```

* Create template chain specification from default local chain

```bash
mkdir -p ./src/chain-spec-templates
./target/release/my-node build-spec \
  --chain=local > ./src/chain-spec-templates/chainspec_default.json
```

* Edit chain specification according to cryptocurrency design requirements
  * Refer to [Chainspec Template with comments](./src/chain-spec-templates/chainspec_with_comments.md)
  * Note: Use `null` for no value
  * References:
    * paritytech/substrate/node/testing/src/genesis.rs
    * https://github.com/hicommonwealth/edgeware-node/blob/master/mainnet/chainspec.json

* Edit WebAssembly code blob with latest chain changes by copying the "code" section from chainspec_latest.json and pasting it into the "code" field of chainspec_my.json

* Build "raw" chain definition for the new chain

```bash
mkdir -p ./src/chain-definition-custom
./target/release/my-node build-spec \
  --chain ./src/chain-spec-templates/chainspec_my.json \
  --raw > ./src/chain-definition-custom/mychain.json
```

* TODO - Add Session and Staking SRMLs

## Run multiple node PoA testnet using custom blockchain configuration <a id="chapter-f21efd"></a>

### Development

```bash
./target/release/my-node \
  --base-path /tmp/polkadot-chains/alice \
  --dev \
  --name "My Testnet" \
  --telemetry-url ws://telemetry.polkadot.io:1024
```

### Production

* Run custom Substrate-based blockchain on local machine testnet with multiple terminals:
  * Imported custom chain definition for custom testnet
  * Use default accounts Alice and Bob as the two initial authorities of the genesis configuration that have been endowed with testnet units that will run validator nodes
  * Multiple authority nodes using the Aura consensus to produce blocks

Terminal 1: Alice's Substrate-based node on default TCP port 30333 with her chain database stored locally at `/tmp/polkadot-chains/alice` and where the bootnode ID of her node is `Local node identity is: QmZ5kgdoLCx3Qfy8nJAiP1U9i6iY3qeiDNSCdHmHRJtSnF` (peer id), which is generated from the `--node-key` value specified below and shown when the node is running. Note that `--alice` provides Alice's session key that is shown when you run `subkey -e inspect //Alice`, alternatively you could provide the private key to that is necessary to produce blocks with `--key "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice"`. In production the session keys are provided to the node using RPC calls `author_insertKey` and `author_rotateKeys`. 
If you explicitly specify a `--node-key` when you start your validator node, the logs will still display your peer id with `Local node identity is: Qxxxxxx`, and you could then include it in the chainspec.json file under "bootNodes". Also the peer id is listed when you go to view the list of full nodes and authority nodes at Polkadot.js Apps https://polkadot.js.org/apps/#/explorer/node:

```bash
./target/release/my-node --validator \
  --base-path /tmp/polkadot-chains/alice \
  --keystore-path "/tmp/polkadot-chains/alice/keys" \
  --chain ./src/chain-definition-custom/mychain.json \
  --key "bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice" \
  --node-key 88dc3417d5058ec4b4503e0c12ea1a0a89be200fe98922423d4334014fa6b0ee \
  --port 30333 \
  --telemetry-url ws://telemetry.polkadot.io:1024
```

Terminal 2: Bob's Substrate-based node on a different TCP port of 30334, and with his chain database stored locally at `/tmp/polkadot-chains/alice`. We'll specify a value for the `--bootnodes` option that will connect his node to Alice's bootnode ID on TCP port 30333:

```bash
./target/release/my-node --validator \
  --base-path /tmp/polkadot-chains/bob \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/QmZ5kgdoLCx3Qfy8nJAiP1U9i6iY3qeiDNSCdHmHRJtSnF \
  --chain ./src/chain-definition-custom/mychain.json \
  --bob \
  --port 30334 \
  --telemetry-url ws://telemetry.polkadot.io:1024
```

* Distribute the custom chain specification to allow others to synchronise and validate if they are an authority

* Add session keys for other account(s) to be configured as authorities (validators)
  * TODO

* View on [Polkascan](https://polkascan.io) or [Polkadot Telemetry](https://telemetry.polkadot.io/#list/My%20Testnet)
  * TODO - Register on Polkascan

## Creating a new custom runtime modules <a id="chapter-7f4be8"></a>

* References:
  * Types
    * Substrate-specific Types
      * Example: e.g. AccountId, Balance, Hash
      * Access another Module's Types:
        * Update your custom runtime module's `Trait` to inherit the types defined in other modules (i.e. balances module).
        * Access these types where we have specified the generic `<T: Trait>` using `T::Type`

  * Module-specific
    * https://substrate.dev/substrate-collectables-workshop/#/1/creating-a-module
    * https://substrate.dev/substrate-verifiable-credentials
    * https://substrate.dev/docs/en/tutorials/adding-a-module-to-your-runtime#adding-runtime-hooks
    * https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

  * Storage-specific
    * References:
      * https://substrate.dev/docs/en/overview/glossary#storage-items
      * https://substrate.dev/rustdocs/v1.0/srml_support_procedural/macro.decl_storage.html
      * https://polkadot.js.org/api/types/#codec-types
      * https://substrate.dev/rustdocs/v1.0/srml_support/storage/trait.StorageValue.html
      * https://substrate.dev/rustdocs/v1.0/srml_support/storage/trait.StorageMap.html
      * https://doc.rust-lang.org/std/vec/struct.Vec.html
      * https://substrate.dev/docs/en/runtime/types/genesisconfig-struct

  * General
    * References:
      * https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

## Import a template SRML module <a id="chapter-352eaa"></a>

* References:
  * https://substrate.dev/docs/en/tutorials/adding-a-module-to-your-runtime#adding-runtime-hooks

## Interact with blockchain using UI <a id="chapter-6d9058"></a>

* Interact with node:
  * Go to Polkadot.js Apps "Settings" tab at https://polkadot.js.org/apps/#/settings
  * General > remote node/endpoint to connect to > Local Node (127.0.0.1:9944)

* Important:
  * Prior to being able to submit extrinics at https://polkadot.js.org/apps/#/extrinsics (i.e. roaming > createKitty()) it is necessary to add
  the custom types to https://polkadot.js.org/apps/#/settings/developer, as follows, otherwise the "Submit Transaction" button will not work.
```
{
  "Kitty": {
    "id": "Hash",
    "dna": "Hash",
    "price": "Balance",
    "gen": "u64"
  }
}
```

* References:
  * https://polkadot.js.org/api/start/types.extend.html

## Create custom blockchain UI <a id="chapter-1c48d9"></a>

* TODO

* References:
  * Types
    * https://polkadot.js.org/api/types/#polkadot-types-2

  * General
    * API / Testing https://github.com/polkadot-js/api/blob/master/packages/api/test/e2e/api/promise-tx-eras.spec.ts
    * https://github.com/polkadot-js/api/blob/jg-hex-string-seeds/docs/start/keyring.md
    * https://polkadot.js.org/api/start/keyring.html
    * Storage cache https://github.com/polkadot-js/apps/blob/master/packages/app-storage/src/Query.tsx
    * https://github.com/substrate-developer-hub/substrate-front-end-template/blob/master/src/examples/TemplateModule.jsx
    * Scripts https://github.com/soc1c/kusama-scripts/blob/master/validator-nominations/nominations.js
    * TypeScript https://polkadot.js.org/api/start/typescript.html

## Interact with blockchain using CLI <a id="chapter-9c32f0"></a>

* TODO

* References:
  * https://github.com/polkadot-js/tools/tree/master/packages/api-cli


## Other resources <a id="chapter-7af38c"></a>

* References:
  * https://github.com/paritytech/substrate/releases
  * https://substrate.dev/en/tutorials