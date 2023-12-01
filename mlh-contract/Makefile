build: build-init build-prod
build-init:

	soroban contract build \
						--no-default-features \
						--features init

	mkdir -p ../mlh-frontend/target
	cp ../target/wasm32-unknown-unknown/release/mlh_contract.wasm ../mlh-frontend/target/milltion-init.wasm 

	
build-prod:

	soroban contract build \
						--no-default-features \
						--features prod
	mkdir -p ../mlh-frontend/target
	cp ../target/wasm32-unknown-unknown/release/mlh_contract.wasm ../mlh-frontend/target/milltion-prod.wasm 


