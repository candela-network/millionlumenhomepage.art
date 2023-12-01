echo "INSTALLING PROD"
WASM_HASH=$(soroban -v contract install --wasm ./target/milltion-prod.wasm --fee 500000 --source admin --network standalone)
echo $WASM_HASH

echo "DEPLOYING INIT"
CONTRACT_ID=$(soroban -v contract deploy --wasm ./target/milltion-init.wasm --fee 500000 --source admin --network standalone)
echo $CONTRACT_ID

echo "GET NATIVE"
NATIVE=$(soroban lab token id --asset native --network standalone)

echo "INITIALIZE"
soroban -v contract invoke --id $CONTRACT_ID --source admin --network standalone --fee 1500000 -- initialize \
	--admin admin \
	--asset $(soroban lab token id --asset native --network standalone) \
	--price 2560000000

echo "UPGRAGE"
soroban -v contract invoke --id $CONTRACT_ID --source admin --network standalone --fee 1500000 -- upgrade --wasm_hash $WASM_HASH

echo "BUMP"
soroban -v contract bump --id $CONTRACT_ID --ledgers-to-expire 6000000 --durability persistent --source admin --network standalone
echo "BUMP"
soroban -v contract bump --wasm-hash $WASM_HASH --ledgers-to-expire 6000000 --durability persistent --source admin --network standalone

echo "HELP"
soroban -v contract invoke --id $CONTRACT_ID --source admin --network standalone -- -h

echo $CONTRACT_ID >contract.id

rm -r data
rm -r node_modules
npm i
soroban contract bindings typescript --wasm ./target/milltion-prod.wasm \
	--network standalone \
	--contract-id $(cat ./contract.id) \
	--output-dir node_modules/Million
mkdir data
npm run dev
