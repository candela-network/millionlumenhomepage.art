# million lumen homepage

Demo: [https://millionlumenhomepage.art](https://millionlumenhomepage.art)

## How to launch the dApp on standalone

### 1. Build the contract

Go to mlh-contract
```
cd ./mlh-contract 
```

Then run the build with `make`.

> You will see that the Makefile build two contracts, the first one is to
> initialize the contract, the second one is the actual contract without 
> the initialize() function. This allows to remove the only called once 
> function from the contract.

This creates 2 wasm files in mlh-frontend/target

### 2. Launch the standalone network

Go to mlh-contract and run the standalone.sh script.

### 3. Deploying the contracts

For now on, we will move to the mlh-frontend folder.

First, we need to configure the soroban cli, if you don't have it, you can install using `cargo install soroban-cli`.
```
soroban config network add standalone \
    --rpc-url http://localhost:8000/soroban/rpc \
    --network-passphrase "Standalone Network ; February 2017"
```

We also need an account to set everything up
```
soroban config identity generate admin
```

We need to fund this account
```
curl "http://localhost:8000/friendbot?addr=$(soroban config identity address admin)" 
```

We need to wrap the asset we will use when minting the NFT. Here
we will simply wrap the native asset, feel free to experiment with other
custome assets.

```
soroban lab token wrap --asset native --network standalone --source admin
```

#### Deploy the init contract
```
soroban contract deploy --wasm ./target/milltion-init.wasm \
    --source admin 
    --network standalone
```
This returns the $CONTRACT_ID.

We can now invoke the initialize function:
```
soroban contract invoke --id $CONTRACT_ID --source admin \
    --network standalone \
    -- initialize \
	--admin admin \
	--asset $(soroban lab token id --asset native --network standalone) \
	--price 2560000000
```

#### Install the actual contract
```
soroban contract install \
    --wasm ./target/milltion-prod.wasm \
    --source admin \
    --network standalone
```
This will return a sha256 hash that is the $WASM_HASH we need 
to upgrade the contract.

#### Upgrade the contract
```
soroban contract invoke --id $CONTRACT_ID \
    --source admin \
    --network standalone \
    -- upgrade \
    --wasm_hash $WASM_HASH
```

Now the contract is fully initialized and can be use.

### 4. Launching the frontend

```
rm -r data                                # remove the data folder
rm -r node_modules                        # delete the node_modules folder
                                          # to remove previous contract id
                                          # references

npm i                                     # install all deps

# generate the contract bindings
soroban contract bindings typescript \
    --wasm ./target/milltion-prod.wasm \
	--network standalone \
	--contract-id $(cat ./contract.id) \
	--contract-name Million \
	--output-dir node_modules/Million

mkdir data   # create the data folder
npm run dev  # launch astro
```