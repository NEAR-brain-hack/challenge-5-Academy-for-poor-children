# deploy dao

near deploy \
    --wasmFile out/main.wasm \
    --initFunction "migrate" \
    --initArgs "{}" \
    --accountId near-hacks-academy.manhng.testnet