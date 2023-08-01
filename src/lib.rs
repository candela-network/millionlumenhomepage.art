#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, Address, BytesN, Env,
    IntoVal, Map, String, TryFromVal, Val, Vec,
};

mod storage;
use crate::storage::Storage;

pub trait ERC721 {
    fn balance_of(env: Env, owner: Address) -> u32;
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, token_id: u32);
    fn approve(
        env: Env,
        caller: Address,
        operator: Option<Address>,
        token_id: u32,
        expiration_ledger: u32,
    );
    fn set_approval_for_all(
        env: Env,
        caller: Address,
        owner: Address,
        operator: Address,
        approved: bool,
        expiration_ledger: u32,
    );
    fn get_approved(env: Env, token_id: u32) -> Option<Address>;
    fn is_approval_for_all(env: Env, owner: Address, operator: Address) -> bool;
}

pub trait ERC721Metadata {
    fn name(env: Env) -> String;
    fn symbol(env: Env) -> String;
    fn token_uri(env: Env, token_uri: u32) -> String;
}

pub trait ERC721Enumerable {
    fn total_supply(env: Env) -> u32;
    fn token_by_index(env: Env, index: u32) -> u32;
    fn token_of_owner_by_index(env: Env, owner: Address, index: u32) -> u32;
}

#[contracttype]
pub enum DataKey {
    Balance(Address),           // instance
    TokenOwner(u32),            // instance
    Approved(u32),              // temporary
    Operator(Address, Address), // temporary
}
impl storage::Storage for DataKey {
    fn get<V: TryFromVal<Env, Val>>(&self, env: &Env) -> Option<V> {
        match self {
            DataKey::Balance(_) | DataKey::TokenOwner(_) => storage::Instance::get(env, self),
            DataKey::Approved(_) | DataKey::Operator(_, _) => storage::Temporary::get(env, self),
        }
    }

    fn set<V: IntoVal<Env, Val>>(&self, env: &Env, val: &V) {
        match self {
            DataKey::Balance(_) | DataKey::TokenOwner(_) => storage::Instance::set(env, self, val),
            DataKey::Approved(_) | DataKey::Operator(_, _) => {
                storage::Temporary::set(env, self, val)
            }
        }
    }

    fn has(&self, env: &Env) -> bool {
        match self {
            DataKey::Balance(_) | DataKey::TokenOwner(_) => storage::Instance::has(env, self),
            DataKey::Approved(_) | DataKey::Operator(_, _) => storage::Temporary::has(env, self),
        }
    }

    fn bump(&self, env: &Env, expiration_ledger: u32) {
        match self {
            DataKey::Balance(_) | DataKey::TokenOwner(_) => {
                storage::Instance::bump(env, expiration_ledger)
            }
            DataKey::Approved(_) | DataKey::Operator(_, _) => {
                storage::Temporary::bump(env, self, expiration_ledger)
            }
        }
    }

    fn remove(&self, env: &Env) {
        match self {
            DataKey::Balance(_) | DataKey::TokenOwner(_) => storage::Instance::remove(env, self),
            DataKey::Approved(_) | DataKey::Operator(_, _) => storage::Temporary::remove(env, self),
        }
    }
}

#[contracttype]
pub enum DatakeyMetadata {
    Name,     // instance
    Symbol,   // instance
    Uri(u32), // instance
}
impl storage::Storage for DatakeyMetadata {
    fn get<V: TryFromVal<Env, Val>>(&self, env: &Env) -> Option<V> {
        storage::Instance::get(env, self)
    }

    fn set<V: IntoVal<Env, Val>>(&self, env: &Env, val: &V) {
        storage::Instance::set(env, self, val)
    }

    fn has(&self, env: &Env) -> bool {
        storage::Instance::has(env, self)
    }

    fn bump(&self, env: &Env, expiration_ledger: u32) {
        storage::Instance::bump(env, expiration_ledger)
    }

    fn remove(&self, env: &Env) {
        storage::Instance::remove(env, self)
    }
}

#[contracttype]
pub enum DataKeyEnumerable {
    IndexToken,               // instance
    TokenIndex,               // instance
    OwnerIndexToken(Address), // instance
    OwnerTokenIndex(Address), // instance
}
impl storage::Storage for DataKeyEnumerable {
    fn get<V: TryFromVal<Env, Val>>(&self, env: &Env) -> Option<V> {
        storage::Instance::get(env, self)
    }

    fn set<V: IntoVal<Env, Val>>(&self, env: &Env, val: &V) {
        storage::Instance::set(env, self, val)
    }

    fn has(&self, env: &Env) -> bool {
        storage::Instance::has(env, self)
    }

    fn bump(&self, env: &Env, expiration_ledger: u32) {
        storage::Instance::bump(env, expiration_ledger)
    }

    fn remove(&self, env: &Env) {
        storage::Instance::remove(env, self)
    }
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
            panic_with_error!(&env, Error::NOTAUTHORIZED);
        }

        if let Some(addr) = DataKey::TokenOwner(token_id).get::<Address>(&env) {
            if addr == from {
                if cfg!(feature = "enumerable") && from != to {
                    // update enumerable data
                    let mut from_index: Vec<u32> = DataKeyEnumerable::OwnerIndexToken(from.clone())
                        .get(&env)
                        .unwrap_or_else(|| Vec::new(&env));
                    let mut from_token: Map<u32, u32> =
                        DataKeyEnumerable::OwnerTokenIndex(from.clone())
                            .get(&env)
                            .unwrap_or_else(|| Map::new(&env));
                    let mut to_index: Vec<u32> = DataKeyEnumerable::OwnerIndexToken(to.clone())
                        .get(&env)
                        .unwrap_or_else(|| Vec::new(&env));
                    let mut to_token: Map<u32, u32> =
                        DataKeyEnumerable::OwnerTokenIndex(to.clone())
                            .get(&env)
                            .unwrap_or_else(|| Map::new(&env));

                    from_index.remove(from_token.get(token_id).unwrap());
                    from_token.remove(token_id);
                    DataKey::Balance(from.clone()).set(&env, &from_index.len());

                    to_token.set(token_id, to_index.len());
                    to_index.push_back(token_id);
                    DataKey::Balance(to.clone()).set(&env, &to_index.len());
                }
                DataKey::TokenOwner(token_id).set(&env, &to);
            } else {
                panic_with_error!(&env, Error::NOTOWNER);
            }
        } else {
            panic_with_error!(&env, Error::NOTNFT);
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
            panic_with_error!(&env, Error::NOTNFT);
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
            panic_with_error!(&env, Error::NOTAUTHORIZED);
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
            .unwrap_or_else(|| panic_with_error!(&env, Error::OUTOFBOUNDS))
    }
    fn token_of_owner_by_index(env: Env, owner: Address, index: u32) -> u32 {
        DataKeyEnumerable::OwnerIndexToken(owner)
            .get::<Vec<u32>>(&env)
            .unwrap_or_else(|| panic_with_error!(&env, Error::OUTOFBOUNDS))
            .get(index)
            .unwrap_or_else(|| panic_with_error!(&env, Error::OUTOFBOUNDS))
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

                env.storage()
                    .instance()
                    .set(&DataKey::Balance(to), &owner_index.len());
            } else {
                let balance_key = DataKey::Balance(to);
                let balance = env.storage().instance().get(&balance_key).unwrap_or(0);
                env.storage().instance().set(&balance_key, &(balance + 1));
            }
        }
    }

    #[cfg(feature = "burnable")]
    pub fn burn(env: Env, token_id: u32) {
        let owner: Address = env
            .storage()
            .instance()
            .get(&DataKey::TokenOwner(token_id))
            .unwrap_or_else(|| panic_with_error!(&env, Error::NOTNFT));
        owner.require_auth();

        env.storage()
            .instance()
            .remove(&DataKey::Approved(token_id));
        env.storage()
            .instance()
            .remove(&DataKey::TokenOwner(token_id));
        let balance_key = DataKey::Balance(owner);
        let balance = env.storage().instance().get(&balance_key).unwrap_or(0);
        env.storage().instance().set(&balance_key, &(balance - 1));

        if cfg!(feature = "enumerable") {
            // TODO: remove token from owner's index
        }
    }
}

fn get_admin(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&ADMIN())
        .unwrap_or_else(|| panic!("Not initialized"))
}
