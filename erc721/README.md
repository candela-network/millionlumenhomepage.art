# ERC721 implementation for Soroban

Reference: [ERC721 standard](https://eips.ethereum.org/EIPS/eip-721)

Reference implementation: [OpenZeppelin ERC721](https://docs.openzeppelin.com/contracts/3.x/erc721)

You can find here a soroban implementation of ERC721, including standard
extensions Enumerable, Metadata and Burnable. The safeTransfer has been 
ommited since we don't have, for now, wallet contracts able to answer.

Each extenstion is enabled using the corresponding feature.

The following example with only metadat extension will require the following Cargo.toml snippet:
```toml
[dependencies]
erc721 = { workspace = true, default-features = false, features = ["metadata"] }
```

## Create your own contract implementing ERC721
```rust

#[contract]
pub struct MyNFTCollection;

///
/// Basic implementation with metadata only
///
#[contractimpl]
impl MyNFTCollection {

    pub fn initialize(env: Env, admin: Address) {
        let name = String::from_slice(&env, "Non-Fungible Token");
        let sym = String::from_slice(&env, "NFT");
        erc721::ERC721Contract::initialize(env, admin, name, sym);
    }

    pub fn upgrade(env: Env, wasm_hash: BytesN<32>) {
        erc721::ERC721Contract::upgrade(env, wasm_hash)
    }

    pub fn mint(env: Env, to: Address, uri: String) {
        // Check the destination approved the transaction
        to.require_auth();

        // Token Uri must be set at some point, just an example here.
        DatakeyMetadata::Uri(token_id).set(&env, &uri);

        // Mint
        erc721::ERC721Contract::mint(env.clone(), to.clone(), token_id)
    }

    pub fn balance_of(env: Env, owner: Address) -> u32 {
        erc721::ERC721Contract::balance_of(env, owner)
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, token_id: u32) {
        erc721::ERC721Contract::transfer_from(env, spender, from, to, token_id)
    }

    pub fn approve(
        env: Env,
        caller: Address,
        operator: Option<Address>,
        token_id: u32,
        expiration_ledger: u32,
    ) {
        erc721::ERC721Contract::approve(env, caller, operator, token_id, expiration_ledger)
    }

    pub fn set_approval_for_all(
        env: Env,
        caller: Address,
        owner: Address,
        operator: Address,
        approved: bool,
        expiration_ledger: u32,
    ) {
        erc721::ERC721Contract::set_approval_for_all(
            env,
            caller,
            owner,
            operator,
            approved,
            expiration_ledger,
        )
    }

    pub fn get_approved(env: Env, token_id: u32) -> Option<Address> {
        erc721::ERC721Contract::get_approved(env, token_id)
    }

    pub fn is_approval_for_all(env: Env, owner: Address, operator: Address) -> bool {
        erc721::ERC721Contract::is_approval_for_all(env, owner, operator)
    }

    pub fn name(env: Env) -> String {
        erc721::ERC721Contract::name(env)
    }

    pub fn symbol(env: Env) -> String {
        erc721::ERC721Contract::symbol(env)
    }

    pub fn token_uri(env: Env, token_id: u32) -> String {
        erc721::ERC721Contract::token_uri(token_id)
    }
}

```