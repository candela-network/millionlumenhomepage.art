WASM_HASH=$(soroban contract install --wasm ./target/milltion-prod.wasm --source admin --network standalone)
echo $WASM_HASH
CONTRACT_ID=$(soroban contract deploy --wasm ./target/milltion-init.wasm --source admin --network standalone)
echo $CONTRACT_ID
NATIVE=$(soroban lab token id --asset native --network standalone)
soroban contract invoke --id $CONTRACT_ID --source admin --network standalone -- initialize \
	--admin admin \
	--asset $(soroban lab token id --asset native --network standalone) \
	--price 2560000000
soroban contract invoke --id $CONTRACT_ID --source admin --network standalone -- upgrade --wasm_hash $WASM_HASH

soroban contract bump --id $CONTRACT_ID --ledgers-to-expire 6000000 --durability persistent --source admin --network standalone
soroban contract bump --wasm-hash $WASM_HASH --ledgers-to-expire 6000000 --durability persistent --source admin --network standalone
soroban contract invoke --id $CONTRACT_ID --source admin --network standalone -- -h

echo $CONTRACT_ID >contract.id

rm -r data
rm -r node_modules
npm i
soroban contract bindings typescript --wasm ./target/milltion-prod.wasm \
	--network standalone \
	--contract-id $(cat ./contract.id) \
	--contract-name Million \
	--output-dir node_modules/Million
mkdir data
npm run dev
