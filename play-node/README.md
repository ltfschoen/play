# play-node

A SRML-based Substrate node.

# Building

Install Rust:

```bash
curl https://getsubstrate.io -sSf | bash -s -- --fast
./scripts/init.sh
```

Build the WebAssembly binary:

```bash
./scripts/build.sh && \
cargo build --release
```

# Run

```bash
rm -rf /tmp/polkadot-chains/

./target/release/play-node \
  --base-path /tmp/polkadot-chains/alice \
  --dev \
  --telemetry-url ws://telemetry.polkadot.io:1024
```

Detailed logs: `RUST_LOG=debug RUST_BACKTRACE=1`

# Test

```bash
cargo test -p play-node-runtime
```

# Custom Types

Add to https://polkadot.js.org/apps/#/settings/developer

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