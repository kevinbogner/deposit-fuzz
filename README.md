# Deposit-Fuzz: An Ethereum Deposit Fuzzer (Work In Progress)

Deposit-Fuzz simulates Ethereum deposit scenarios by randomizing the parameters involved. It utilizes `ethereal`, `eth2-val-tools`, and Rust.

## Prerequisites

* [Rust](https://www.rust-lang.org/tools/install)
* [ethereal](https://github.com/wealdtech/ethereal)
* [eth2-val-tools](https://github.com/protolambda/eth2-val-tools)

## Setup

Update the `secrets.env` file to match your configurations. An example of this file is included in the repository.

## Build & Run

```bash
cargo build --release
./target/release/deposit-fuzz
```

## License

Deposit-Fuzz is released under the [GNU General Public License version 3](https://www.gnu.org/licenses/gpl-3.0.en.html).
