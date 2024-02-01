use soroban_sdk::{contracterror, contracttype, Address, Env, IntoVal, TryFromVal, Val};

use storage::*;

#[contracttype]
pub enum Admin {
    User,
}
impl storage::Storage for Admin {
    fn get<V: soroban_sdk::TryFromVal<Env, soroban_sdk::Val>>(&self, env: &Env) -> Option<V> {
        storage::Persistent::get(env, self)
    }

    fn set<V: soroban_sdk::IntoVal<Env, soroban_sdk::Val>>(&self, env: &Env, val: &V) {
        storage::Persistent::set(env, self, val)
    }

    fn has(&self, env: &Env) -> bool {
        storage::Persistent::has(env, self)
    }

    fn extend(&self, env: &Env, min_ledger_to_live: u32) -> &Self {
        storage::Persistent::extend(env, self, min_ledger_to_live);
        self
    }

    fn remove(&self, env: &Env) {
        storage::Persistent::remove(env, self)
    }
}
#[contracttype]
pub enum DataKey {
    Balance(Address),           // instance
    TokenOwner(u32),            // instance
    Approved(u32),              // temporary
    Operator(Address, Address), // temporary
}
impl Storage for DataKey {
    fn get<V: TryFromVal<Env, Val>>(&self, env: &Env) -> Option<V> {
        match self {
            DataKey::Balance(_) | DataKey::TokenOwner(_) => storage::Persistent::get(env, self),
            DataKey::Approved(_) | DataKey::Operator(_, _) => storage::Temporary::get(env, self),
        }
    }

    fn set<V: IntoVal<Env, Val>>(&self, env: &Env, val: &V) {
        match self {
            DataKey::Balance(_) | DataKey::TokenOwner(_) => {
                storage::Persistent::set(env, self, val)
            }
            DataKey::Approved(_) | DataKey::Operator(_, _) => {
                storage::Temporary::set(env, self, val)
            }
        }
    }

    fn has(&self, env: &Env) -> bool {
        match self {
            DataKey::Balance(_) | DataKey::TokenOwner(_) => storage::Persistent::has(env, self),
            DataKey::Approved(_) | DataKey::Operator(_, _) => storage::Temporary::has(env, self),
        }
    }

    fn extend(&self, env: &Env, min_ledger_to_live: u32) -> &Self {
        match self {
            DataKey::Balance(_) | DataKey::TokenOwner(_) => {
                storage::Persistent::extend(env, self, min_ledger_to_live)
            }
            DataKey::Approved(_) | DataKey::Operator(_, _) => {
                storage::Temporary::extend(env, self, min_ledger_to_live)
            }
        };
        self
    }

    fn remove(&self, env: &Env) {
        match self {
            DataKey::Balance(_) | DataKey::TokenOwner(_) => storage::Persistent::remove(env, self),
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

    fn extend(&self, env: &Env, min_ledger_to_live: u32) -> &Self {
        storage::Instance::extend(env, min_ledger_to_live);
        self
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

    fn extend(&self, env: &Env, min_ledger_to_live: u32) -> &Self {
        storage::Instance::extend(env, min_ledger_to_live);
        self
    }

    fn remove(&self, env: &Env) {
        storage::Instance::remove(env, self)
    }
}

#[contracterror]
#[derive(Copy, Clone, Debug)]
pub enum Error {
    NotOwner = 0,
    NotNFT = 1,
    NotAuthorized = 2,
    OutOfBounds = 4,
}

pub enum Event {
    Mint,
    Transfer,
    Approve,
    Burn,
}
impl Event {
    fn name(&self) -> &'static str {
        match self {
            Event::Mint => stringify!(Mint),
            Event::Transfer => stringify!(Transfer),
            Event::Approve => stringify!(Approve),
            Event::Burn => stringify!(Burn),
        }
    }
    pub fn publish<D>(&self, env: &Env, value: D)
    where
        D: IntoVal<Env, Val>,
    {
        env.events().publish((self.name(),), value);
    }
}
