#!bin/bash
stellar contract build

stellar contract deploy \
--source-account testnet-deployer \
--wasm ./target/wasm32-unknown-unknown/release/untangled-vault.wasm \
--network testnet \ 
--alias vault