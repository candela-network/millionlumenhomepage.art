WASM_HASH=$(soroban contract install --wasm ./target/milltion-prod.wasm --source admin --network futurenet)
echo $WASM_HASH
CONTRACT_ID=$(soroban contract deploy --wasm ./target/milltion-init.wasm --source admin --network futurenet)
echo $CONTRACT_ID
NATIVE=$(soroban lab token id --asset native --network futurenet)
soroban contract invoke --id $CONTRACT_ID --source admin --network futurenet -- initialize \
	--admin admin \
	--asset $(soroban lab token id --asset native --network futurenet) \
	--price 2560000000
soroban contract invoke --id $CONTRACT_ID --source admin --network futurenet -- upgrade --wasm_hash $WASM_HASH

soroban contract bump --id $CONTRACT_ID --ledgers-to-expire 6000000 --durability persistent --source admin --network futurenet
soroban contract bump --wasm-hash $WASM_HASH --ledgers-to-expire 6000000 --durability persistent --source admin --network futurenet

soroban contract invoke --id $CONTRACT_ID --source admin --network futurenet -- -h

echo $CONTRACT_ID >contract.id

soroban contract bindings typescript --wasm ./target/milltion-prod.wasm \
	--network futurenet \
	--contract-id $(cat ./contract.id) \
	--contract-name Million \
	--output-dir ./Million
