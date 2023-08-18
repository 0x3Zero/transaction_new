#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

# This script builds all subprojects and puts all created Wasm modules in one dir
echo "compiling crypto..."
cd crypto
cargo update --aggressive
marine build --release

echo "compiling ipfsdag..."
cd ../ipfsdag
cargo update --aggressive
marine build --release

echo "compiling transaction..."
cd ../transaction
cargo update --aggressive
marine build --release

cd ..
mkdir -p artifacts
rm -f artifacts/*.wasm
cp target/wasm32-wasi/release/crypto.wasm artifacts/
cp target/wasm32-wasi/release/crypto.wasm ../builtin-package/
cp target/wasm32-wasi/release/ipfsdag.wasm artifacts/
cp target/wasm32-wasi/release/ipfsdag.wasm ../builtin-package/
cp target/wasm32-wasi/release/transaction.wasm artifacts/
cp target/wasm32-wasi/release/transaction.wasm ../builtin-package/
marine aqua artifacts/crypto.wasm -s Crypto -i crypto > ../aqua/crypto.aqua
marine aqua artifacts/ipfsdag.wasm -s IpfsDag -i ipfsdag > ../aqua/ipfsdag.aqua
marine aqua artifacts/transaction.wasm -s Transaction -i transaction > ../aqua/transaction.aqua

RUST_LOG="info" mrepl --quiet Config.toml