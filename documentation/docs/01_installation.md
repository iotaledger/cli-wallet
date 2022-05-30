# Installation

## From release

## From source

### 1. Install Rust

https://www.rust-lang.org/tools/install

### 2. Compile

```sh
git clone git@github.com:iotaledger/cli-wallet.git -b develop
cd cli-wallet
cargo build --profile production
```

Resulting binary will be located at `./target/production/wallet`.
