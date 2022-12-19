RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
mkdir -p ./dist
cp target/wasm32-unknown-unknown/release/nft_benefits_vault.wasm ./dist/