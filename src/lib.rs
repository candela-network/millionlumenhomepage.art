#![no_std]

use crate::erc721traits::burnable::ERC721Burnable;
use crate::erc721traits::enumerable::ERC721Enumerable;
use crate::erc721traits::erc721::ERC721;
use crate::erc721traits::metadata::ERC721Metadata;
use soroban_sdk::{
    contract, contractimpl, contracttype, panic_with_error, Address, BytesN, Env, Map, String, Vec,
};

mod erc721traits;
mod storage;
mod types;
use crate::storage::Storage;
use crate::types::*;

#[contract]
pub struct ERC721Contract;

#[contractimpl]
impl ERC721 for ERC721Contract {
    fn balance_of(env: Env, owner: Address) -> u32 {
        DataKey::Balance(owner).get(&env).unwrap_or(0)
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, token_id: u32) {
        spender.require_auth();
        let is_sender_approved = if spender != from {
            let has_approved =
                if let Some(approved) = DataKey::Approved(token_id).get::<Address>(&env) {
                    // Clear the approval on transfer
                    DataKey::Approved(token_id).remove(&env);
                    approved == spender
                } else {
                    false
                };
            if !has_approved {
                DataKey::Operator(from.clone(), spender).has(&env)
            } else {
                true
            }
        } else {
            true
        };
        if !is_sender_approved {
            panic_with_error!(&env, Error::NotAuthorized);
        }

        if let Some(addr) = DataKey::TokenOwner(token_id).get::<Address>(&env) {
            if addr == from {
                if cfg!(feature = "enumerable") && from != to {
                    // update enumerable datai
                    let from_index_key = DataKeyEnumerable::OwnerIndexToken(from.clone());
                    let from_token_key = DataKeyEnumerable::OwnerTokenIndex(from.clone());
                    let to_index_key = DataKeyEnumerable::OwnerIndexToken(to.clone());
                    let to_token_key = DataKeyEnumerable::OwnerTokenIndex(to.clone());
                    let mut from_index: Vec<u32> =
                        from_index_key.get(&env).unwrap_or_else(|| Vec::new(&env));
                    let mut from_token: Map<u32, u32> =
                        from_token_key.get(&env).unwrap_or_else(|| Map::new(&env));
                    let mut to_index: Vec<u32> =
                        to_index_key.get(&env).unwrap_or_else(|| Vec::new(&env));
                    let mut to_token: Map<u32, u32> =
                        to_token_key.get(&env).unwrap_or_else(|| Map::new(&env));

                    // Remove token from index of from address
                    from_index.remove(from_token.get(token_id).unwrap());
                    from_token.remove(token_id);

                    // Remove token from index of to address
                    to_token.set(token_id, to_index.len());
                    to_index.push_back(token_id);

                    // Update from address vec and map
                    from_index_key.set(&env, &from_index);
                    from_token_key.set(&env, &from_token);
                    DataKey::Balance(from.clone()).set(&env, &from_index.len());

                    // Update to address vec and map
                    to_token_key.set(&env, &to_token);
                    to_index_key.set(&env, &to_index);
                    DataKey::Balance(to.clone()).set(&env, &to_index.len());
                }
                DataKey::TokenOwner(token_id).set(&env, &to);
            } else {
                panic_with_error!(&env, Error::NotOwner);
            }
        } else {
            panic_with_error!(&env, Error::NotNFT);
        }
    }
    fn approve(
        env: Env,
        caller: Address,
        operator: Option<Address>,
        token_id: u32,
        expiration_ledger: u32,
    ) {
        if let Some(owner) = DataKey::TokenOwner(token_id).get::<Address>(&env) {
            if owner == caller {
                owner.require_auth();
            } else if DataKey::Operator(owner, caller.clone())
                .get::<bool>(&env)
                .unwrap_or(false)
            {
                caller.require_auth();
            }
        } else {
            panic_with_error!(&env, Error::NotNFT);
        }
        if let Some(to_approve) = operator {
            DataKey::Approved(token_id).set(&env, &to_approve);
            DataKey::Approved(token_id).bump(&env, expiration_ledger);
        } else {
            DataKey::Approved(token_id).remove(&env);
        }
    }
    fn set_approval_for_all(
        env: Env,
        caller: Address,
        owner: Address,
        operator: Address,
        approved: bool,
        expiration_ledger: u32,
    ) {
        if owner == caller {
            owner.require_auth();
        } else if DataKey::Operator(owner.clone(), caller.clone())
            .get::<bool>(&env)
            .unwrap_or(false)
        {
            caller.require_auth();
        } else {
            panic_with_error!(&env, Error::NotAuthorized);
        }
        let key = DataKey::Operator(owner, operator);
        if approved {
            key.set(&env, &true);
            key.bump(&env, expiration_ledger);
        } else {
            key.remove(&env);
        }
    }
    fn get_approved(env: Env, token_id: u32) -> Option<Address> {
        DataKey::Approved(token_id).get(&env).unwrap_or(None)
    }
    fn is_approval_for_all(env: Env, owner: Address, operator: Address) -> bool {
        DataKey::Operator(owner, operator)
            .get(&env)
            .unwrap_or(false)
    }
}

#[cfg(feature = "metadata")]
#[contractimpl]
impl ERC721Metadata for ERC721Contract {
    fn name(env: Env) -> String {
        DatakeyMetadata::Name.get(&env).unwrap()
    }
    fn symbol(env: Env) -> String {
        DatakeyMetadata::Symbol.get(&env).unwrap()
    }
    fn token_uri(env: Env, token_id: u32) -> String {
        DatakeyMetadata::Uri(token_id).get(&env).unwrap()
    }
}

#[cfg(feature = "enumerable")]
#[contractimpl]
impl ERC721Enumerable for ERC721Contract {
    fn total_supply(env: Env) -> u32 {
        DataKeyEnumerable::IndexToken
            .get::<Vec<u32>>(&env)
            .unwrap()
            .len()
    }
    fn token_by_index(env: Env, index: u32) -> u32 {
        DataKeyEnumerable::IndexToken
            .get::<Vec<u32>>(&env)
            .unwrap()
            .get(index)
            .unwrap_or_else(|| panic_with_error!(&env, Error::OutOfBounds))
    }
    fn token_of_owner_by_index(env: Env, owner: Address, index: u32) -> u32 {
        DataKeyEnumerable::OwnerIndexToken(owner)
            .get::<Vec<u32>>(&env)
            .unwrap_or_else(|| panic_with_error!(&env, Error::OutOfBounds))
            .get(index)
            .unwrap_or_else(|| panic_with_error!(&env, Error::OutOfBounds))
    }
}

#[cfg(feature = "burnable")]
impl ERC721Burnable for ERC721Contract {
    fn burn(env: Env, caller: Address, token_id: u32) {
        let owner: Address = DataKey::TokenOwner(token_id)
            .get(&env)
            .unwrap_or_else(|| panic_with_error!(&env, Error::NotNFT));
        if owner == caller {
            owner.require_auth();
        } else if DataKey::Operator(owner.clone(), caller.clone())
            .get::<bool>(&env)
            .unwrap_or(false)
        {
            caller.require_auth();
        } else {
            panic_with_error!(&env, Error::NotAuthorized);
        }

        DataKey::Approved(token_id).remove(&env);
        DataKey::TokenOwner(token_id).remove(&env);

        if cfg!(feature = "enumerable") {
            let from_index_key = DataKeyEnumerable::OwnerIndexToken(owner.clone());
            let from_token_key = DataKeyEnumerable::OwnerTokenIndex(owner.clone());

            let mut from_index: Vec<u32> =
                from_index_key.get(&env).unwrap_or_else(|| Vec::new(&env));
            let mut from_token: Map<u32, u32> =
                from_token_key.get(&env).unwrap_or_else(|| Map::new(&env));

            from_index.remove(from_token.get(token_id).unwrap());
            from_token.remove(token_id);

            from_index_key.set(&env, &from_index);
            from_token_key.set(&env, &from_token);
            DataKey::Balance(owner).set(&env, &from_index.len());
        } else {
            let balance_key = DataKey::Balance(owner.clone());
            let balance = balance_key.get(&env).unwrap_or(0);
            balance_key.set(&env, &(balance - 1));
        }
    }
}
#[contracttype]
pub struct ADMIN();

#[contractimpl]
impl ERC721Contract {
    pub fn initialize(
        env: Env,
        admin: Address,
        #[cfg(feature = "metadata")] name: String,
        #[cfg(feature = "metadata")] symbol: String,
    ) {
        if env.storage().instance().has(&ADMIN()) {
            panic!("Already initialized")
        }
        env.storage().instance().set(&ADMIN(), &admin);
        if cfg!(feature = "metadata") {
            env.storage().instance().set(&DatakeyMetadata::Name, &name);
            env.storage()
                .instance()
                .set(&DatakeyMetadata::Symbol, &symbol);
        }

        if cfg!(feature = "enumerable") {
            DataKeyEnumerable::IndexToken.set(&env, &Vec::<u32>::new(&env));
            DataKeyEnumerable::TokenIndex.set(&env, &Map::<u32, u32>::new(&env));
        }
    }

    pub fn upgrade(env: Env, hash: BytesN<32>) {
        get_admin(&env).require_auth();
        env.deployer().update_current_contract_wasm(hash);
    }

    pub fn mint(env: Env, to: Address, token_id: u32) {
        get_admin(&env).require_auth();

        if !env.storage().instance().has(&DataKey::TokenOwner(token_id)) {
            env.storage()
                .instance()
                .set(&DataKey::TokenOwner(token_id), &to);

            if cfg!(feature = "enumerable") {
                let mut owned_index: Vec<u32> = DataKeyEnumerable::IndexToken.get(&env).unwrap();
                let mut owned_token_index: Map<u32, u32> =
                    DataKeyEnumerable::TokenIndex.get(&env).unwrap();

                let mut owner_index: Vec<u32> = DataKeyEnumerable::OwnerIndexToken(to.clone())
                    .get(&env)
                    .unwrap_or_else(|| Vec::new(&env));
                let mut owner_token_index: Map<u32, u32> =
                    DataKeyEnumerable::OwnerTokenIndex(to.clone())
                        .get(&env)
                        .unwrap_or_else(|| Map::new(&env));

                owned_token_index.set(token_id, owned_index.len());
                owned_index.push_back(token_id);

                owner_token_index.set(token_id, owner_index.len());
                owner_index.push_back(token_id);

                DataKeyEnumerable::IndexToken.set(&env, &owned_index);
                DataKeyEnumerable::TokenIndex.set(&env, &owned_token_index);
                DataKeyEnumerable::OwnerIndexToken(to.clone()).set(&env, &owner_index);
                DataKeyEnumerable::OwnerTokenIndex(to.clone()).set(&env, &owner_token_index);

                DataKey::Balance(to).set(&env, &owner_index.len());
            } else {
                let key = DataKey::Balance(to);
                let balance = key.get(&env).unwrap_or(0);
                key.set(&env, &(balance + 1));
            }
        }
    }
}

fn get_admin(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&ADMIN())
        .unwrap_or_else(|| panic!("Not initialized"))
}
