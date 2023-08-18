soroban config identity generate admin
curl "http://localhost:8000/friendbot?addr=$(soroban config identity address admin)" 2>&1 >/dev/null
soroban lab token wrap --asset native --network standalone --source admin &

CURRENT_DIR=$(pwd)
cd ../mlh-contract/
echo building contract
RUSTFLAGS="-C target-cpu=mvp" cargo +nightly build --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
cd $CURRENT_DIR
#RUSTFLAGS="-C target-cpu=mvp" cargo +nightly build --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --no-default-features --features init
#soroban contract build --no-default-features --features init
#RUSTFLAGS="-C target-feature=-sign-ext" cargo +nightly build --target wasm32-unknown-unknown --release
#soroban contract build

echo deploying contract
CONTRACT_ID=$(soroban contract deploy --wasm ../target/wasm32-unknown-unknown/release/mlh_contract.wasm --source admin --network standalone)

echo initializing contract $CONTRACT_ID
soroban contract invoke --id $CONTRACT_ID --source admin --network standalone -- initialize --admin $(soroban config identity address admin) --asset $(soroban lab token id --asset native --network standalone) --price 2560000000

echo $CONTRACT_ID >./contract.id

#echo building contract for prod
#RUSTFLAGS="-C target-cpu=mvp" cargo +nightly build --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --no-default-features --features prod
#soroban contract build --no-default-features --features prod
#RUSTFLAGS="-C target-cpu=mvp" cargo +nightly build --target wasm32-unknown-unknown --release --no-default-features --features prod

#echo installing contract
#WASM_HASH=$(soroban contract install --wasm ../target/wasm32-unknown-unknown/release/millionlumenhomepage.wasm --source admin --network standalone)

#echo updating contract $WASM_HASH
#soroban contract invoke --id $CONTRACT_ID --source admin --network standalone -- upgrade --wasm_hash $WASM_HASH

#soroban contract invoke --id $CONTRACT_ID --source admin --network standalone -- -h
#CONTRACT_ID=CBEGCPXLRTBPGCK5XRXHL3XQCFZVRSQVFEU26ZKHMLRPL3TSEWF2T43U

mint() {
	USERNFT=nft$1
	soroban config identity generate $USERNFT
	curl -s "http://localhost:8000/friendbot?addr=$(soroban config identity address $USERNFT)" 2>&1 >/dev/null

	soroban contract invoke --id $CONTRACT_ID --source $USERNFT --network standalone -- total_supply
	soroban contract invoke --id $CONTRACT_ID --fee 300000 --source $USERNFT --network standalone -- mint --to $(soroban config identity address $USERNFT) 2>&1 >/dev/null
}

#for i in $(seq 1 4097); do
#	for j in $(seq 1 3); do
#		mint "${i}_${j}"
#	done
#	echo $(soroban lab token wrap --asset native --network standalone --source admin 2>&1) >/dev/null
#done
