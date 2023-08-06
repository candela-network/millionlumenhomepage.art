echo building contract
#soroban contract build --no-default-features --features init
RUSTFLAGS="-C target-feature=-sign-ext" cargo +nightly build --target wasm32-unknown-unknown --release
#soroban contract build

echo deploying contract
CONTRACT_ID=$(soroban contract deploy --wasm ../target/wasm32-unknown-unknown/release/millionlumenhomepage.wasm --source admin --network standalone)

echo initializing contract $CONTRACT_ID
soroban contract invoke --id $CONTRACT_ID --source admin --network standalone -- initialize --admin $(soroban config identity address admin) --asset $(soroban lab token id --asset native --network standalone) --price 2560000000

#echo building contract for prod
#soroban contract build --no-default-features --features prod
#RUSTFLAGS="-C target-cpu=mvp" cargo +nightly build --target wasm32-unknown-unknown --release --no-default-features --features prod

#echo installing contract
#WASM_HASH=$(soroban contract install --wasm ../target/wasm32-unknown-unknown/release/millionlumenhomepage.wasm --source admin --network standalone)

#echo updating contract $WASM_HASH
#soroban contract invoke --id $CONTRACT_ID --source admin --network standalone -- upgrade --wasm_hash $WASM_HASH

soroban contract invoke --id $CONTRACT_ID --source admin --network standalone -- -h

soroban contract invoke --id $CONTRACT_ID --source admin --network standalone -- total_supply
