use soroban_sdk::{contracttype, Env};
#[contracttype]
pub enum MillionDataKey {
    TokenId,
    AssetAddress,
    Price,
}
impl storage::Storage for MillionDataKey {
    fn get<V: soroban_sdk::TryFromVal<Env, soroban_sdk::Val>>(&self, env: &Env) -> Option<V> {
        storage::Instance::get(env, self)
    }

    fn set<V: soroban_sdk::IntoVal<Env, soroban_sdk::Val>>(&self, env: &Env, val: &V) {
        storage::Instance::set(env, self, val)
    }

    fn has(&self, env: &Env) -> bool {
        storage::Instance::has(env, self)
    }

    fn bump(&self, env: &Env, min_ledger_to_live: u32) -> &Self {
        storage::Instance::bump(env, min_ledger_to_live);
        self
    }
    fn bump_until(&self, env: &Env, expiration_ledger: u32) -> &Self {
        storage::Instance::bump_until(env, expiration_ledger);
        self
    }

    fn remove(&self, env: &Env) {
        storage::Instance::remove(env, self)
    }
}

#[contracttype]
pub enum Coords {
    Token(u32, u32),
    Xy(u32),
}
impl storage::Storage for Coords {
    fn get<V: soroban_sdk::TryFromVal<Env, soroban_sdk::Val>>(&self, env: &Env) -> Option<V> {
        storage::Persistent::get(env, self)
    }

    fn set<V: soroban_sdk::IntoVal<Env, soroban_sdk::Val>>(&self, env: &Env, val: &V) {
        storage::Persistent::set(env, self, val)
    }

    fn has(&self, env: &Env) -> bool {
        storage::Persistent::has(env, self)
    }

    fn bump(&self, env: &Env, min_ledger_to_live: u32) -> &Self {
        storage::Persistent::bump(env, self, min_ledger_to_live);
        self
    }
    fn bump_until(&self, env: &Env, expiration_ledger: u32) -> &Self {
        storage::Persistent::bump_until(env, self, expiration_ledger);
        self
    }

    fn remove(&self, env: &Env) {
        storage::Persistent::remove(env, self)
    }
}
/*
#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MillionError {
    Exhausted = 1,
}
*/
