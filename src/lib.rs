#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, Address, BytesN, Env,
    String,
};

pub trait ERC721 {
    fn balance_of(env: Env, owner: Address) -> u128;
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, token_id: u128);
    fn approve(env: Env, operator: Option<Address>, token_id: u128, expiration_ledger: u32);
    fn set_approval_for_all(
        env: Env,
        owner: Address,
        operator: Address,
        approved: bool,
        expiration_ledger: u32,
    );
    fn get_approved(env: Env, token_id: u128) -> Option<Address>;
    fn is_approval_for_all(env: Env, owner: Address, operator: Address) -> bool;
}

pub trait ERC721Metadata {
    fn name(env: Env) -> String;
    fn symbol(env: Env) -> String;
    fn token_uri(env: Env, token_uri: u128) -> String;
}

pub trait ERC721Enumerable {
    fn total_supply(env: Env) -> u128;
    fn token_by_index(env: Env, index: u128) -> u128;
    fn token_of_owner_by_index(env: Env, owner: Address, index: u128) -> u128;
}

#[contracttype]
pub enum DataKey {
    Balance(Address),           // instance
    TokenOwner(u128),           // instance
    Approved(u128),             // temporary
    Operator(Address, Address), // temporary
}

#[contracttype]
pub enum DatakeyMetadata {
    Name,      // instance
    Symbol,    // instance
    Uri(u128), // instance
}

#[contracttype]
pub enum DataKeyEnumerable {
    Supply,                    // instance
    Index(u128),               // instance
    OwnerIndex(Address, u128), // instance
}

#[contracterror]
#[derive(Copy, Clone)]
pub enum Error {
    NOTOWNER = 0,
    NOTNFT = 1,
    NOTAUTHORIZED = 2,
    OUTOFBOUNDS = 4,
}

#[contract]
pub struct ERC721Contract;

#[contractimpl]
impl ERC721 for ERC721Contract {
    fn balance_of(env: Env, owner: Address) -> u128 {
        env.storage()
            .instance()
            .get(&DataKey::Balance(owner))
            .unwrap_or(0)
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, token_id: u128) {
        spender.require_auth();
        let is_sender_approved = if spender != from {
            let has_approved = if let Some(approved) = env
                .storage()
                .temporary()
                .get::<DataKey, Address>(&DataKey::Approved(token_id))
            {
                if approved == spender {
                    env.storage()
                        .temporary()
                        .remove(&DataKey::Approved(token_id));
                    true
                } else {
                    false
                }
            } else {
                false
            };
            if !has_approved {
                env.storage()
                    .temporary()
                    .has(&DataKey::Operator(from.clone(), spender))
            } else {
                true
            }
        } else {
            true
        };
        if !is_sender_approved {
            panic_with_error!(&env, Error::NOTAUTHORIZED);
        }

        if let Some(addr) = env
            .storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::TokenOwner(token_id))
        {
            if addr == from {
                env.storage()
                    .instance()
                    .set(&DataKey::TokenOwner(token_id), &to);
            } else {
                panic_with_error!(&env, Error::NOTOWNER);
            }
        } else {
            panic_with_error!(&env, Error::NOTNFT);
        }
    }
    fn approve(env: Env, spender: Option<Address>, token_id: u128, expiration_ledger: u32) {
        if let Some(owner) = env
            .storage()
            .temporary()
            .get::<DataKey, Address>(&DataKey::TokenOwner(token_id))
        {
            owner.require_auth();
        } else {
            panic_with_error!(&env, Error::NOTNFT);
        }
        if let Some(to_approve) = spender {
            env.storage()
                .temporary()
                .set(&DataKey::Approved(token_id), &to_approve);
            env.storage().temporary().bump(
                &DataKey::Approved(token_id),
                expiration_ledger
                    .checked_sub(env.ledger().sequence())
                    .unwrap(),
            );
        } else {
            env.storage()
                .temporary()
                .remove(&DataKey::Approved(token_id));
        }
    }
    fn set_approval_for_all(
        env: Env,
        owner: Address,
        operator: Address,
        approved: bool,
        expiration_ledger: u32,
    ) {
        if approved {
            let key = DataKey::Operator(owner, operator);
            env.storage().temporary().set(&key, &true);
            env.storage().temporary().bump(
                &key,
                expiration_ledger
                    .checked_sub(env.ledger().sequence())
                    .unwrap(),
            );
        }
    }
    fn get_approved(env: Env, token_id: u128) -> Option<Address> {
        env.storage().temporary().get(&DataKey::Approved(token_id))
    }
    fn is_approval_for_all(env: Env, owner: Address, operator: Address) -> bool {
        env.storage()
            .temporary()
            .get(&DataKey::Operator(owner, operator))
            .unwrap_or(false)
    }
}

#[cfg(feature = "metadata")]
#[contractimpl]
impl ERC721Metadata for ERC721Contract {
    fn name(env: Env) -> String {
        env.storage()
            .instance()
            .get(&DatakeyMetadata::Name)
            .unwrap()
    }
    fn symbol(env: Env) -> String {
        env.storage()
            .instance()
            .get(&DatakeyMetadata::Symbol)
            .unwrap()
    }
    fn token_uri(env: Env, token_id: u128) -> String {
        env.storage()
            .instance()
            .get(&DatakeyMetadata::Uri(token_id))
            .unwrap()
    }
}

#[cfg(feature = "enumerable")]
#[contractimpl]
impl ERC721Enumerable for ERC721Contract {
    fn total_supply(env: Env) -> u128 {
        env.storage()
            .instance()
            .get(&DataKeyEnumerable::Supply)
            .unwrap_or(0)
    }
    fn token_by_index(env: Env, index: u128) -> u128 {
        env.storage()
            .instance()
            .get(&DataKeyEnumerable::Index(index))
            .unwrap_or_else(|| panic_with_error!(&env, Error::OUTOFBOUNDS))
    }
    fn token_of_owner_by_index(env: Env, owner: Address, index: u128) -> u128 {
        env.storage()
            .instance()
            .get(&DataKeyEnumerable::OwnerIndex(owner, index))
            .unwrap_or_else(|| panic_with_error!(&env, Error::OUTOFBOUNDS))
    }
}

#[contracttype]
pub struct ADMIN();
impl ERC721Contract {
    fn initialize(
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
    }

    fn upgrade(env: Env, hash: BytesN<32>) {
        get_admin(&env).require_auth();
        env.deployer().update_current_contract_wasm(hash);
    }

    fn mint(env: Env, to: Address, token_id: u128) {
        get_admin(&env).require_auth();

        if !env.storage().instance().has(&DataKey::TokenOwner(token_id)) {
            env.storage()
                .instance()
                .set(&DataKey::TokenOwner(token_id), &to);

            let balance_key = DataKey::Balance(to.clone());
            let balance = env.storage().instance().get(&balance_key).unwrap_or(0);
            env.storage().instance().set(&balance_key, &(balance + 1));

            if cfg!(feature = "enumerable") {
                let supply: u128 = env
                    .storage()
                    .instance()
                    .get(&DataKeyEnumerable::Supply)
                    .unwrap_or(0);
                env.storage()
                    .instance()
                    .set(&DataKeyEnumerable::Supply, &(supply + 1));

                env.storage()
                    .instance()
                    .set(&DataKeyEnumerable::Index(supply), &token_id);
                env.storage()
                    .instance()
                    .set(&DataKeyEnumerable::OwnerIndex(to, balance), &token_id);
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
