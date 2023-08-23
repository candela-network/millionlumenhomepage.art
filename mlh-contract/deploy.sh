WASM_HASH=$(soroban contract install --wasm ./target/milltion-prod.wasm --source admin --network standalone)
echo $WASM_HASH
CONTRACT_ID=$(soroban contract deploy --wasm ./target/milltion-init.wasm --source --network)
echo $CONTRACT_ID
NATIVE=$(soroban lab token id --asset native --network standalone)
soroban contract invoke --id $CONTRACT_ID --source admin --network standalone -- initialize \
	--admin $(soroban config identity address admin) \
	--asset $(soroban lab token id --asset native --network standalone) \
	--price 2560000000
soroban contract invoke --id $CONTRACT_ID --source admin --network standalone -- upgrade --wasm_hash $WASM_HASH

soroban contract invoke --id $CONTRACT_ID --source admin --network standalone -- -h
