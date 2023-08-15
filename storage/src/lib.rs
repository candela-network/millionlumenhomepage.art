#![no_std]
use soroban_sdk::{Env, IntoVal, TryFromVal, Val};

pub trait Storage {
    fn get<V: TryFromVal<Env, Val>>(&self, env: &Env) -> Option<V>;
    fn set<V: IntoVal<Env, Val>>(&self, env: &Env, val: &V);
    fn has(&self, env: &Env) -> bool;
    fn bump(&self, env: &Env, min_ledger_to_live: u32) -> &Self;
    fn bump_until(&self, env: &Env, expiration_ledger: u32) -> &Self;
    fn remove(&self, env: &Env);
}

pub struct Instance;
impl Instance {
    pub fn get<K: IntoVal<Env, Val>, V: TryFromVal<Env, Val>>(env: &Env, key: &K) -> Option<V> {
        env.storage().instance().get::<K, V>(key)
    }
    pub fn set<K: IntoVal<Env, Val>, V: IntoVal<Env, Val>>(env: &Env, key: &K, val: &V) {
        env.storage().instance().set(key, val);
    }
    pub fn has<K: IntoVal<Env, Val>>(env: &Env, key: &K) -> bool {
        env.storage().instance().has(key)
    }
    pub fn bump(env: &Env, min_ledger_to_live: u32) {
        env.storage().instance().bump(min_ledger_to_live)
    }
    pub fn bump_until(env: &Env, expiration_ledger: u32) {
        env.storage().instance().bump(
            expiration_ledger
                .checked_sub(env.ledger().sequence())
                .unwrap(),
        )
    }
    pub fn remove<K: IntoVal<Env, Val>>(env: &Env, key: &K) {
        env.storage().instance().remove(key);
    }
}

pub struct Persistent;
impl Persistent {
    pub fn get<K: IntoVal<Env, Val>, V: TryFromVal<Env, Val>>(env: &Env, key: &K) -> Option<V> {
        env.storage().persistent().get::<K, V>(key)
    }
    pub fn set<K: IntoVal<Env, Val>, V: IntoVal<Env, Val>>(env: &Env, key: &K, val: &V) {
        env.storage().persistent().set(key, val);
    }
    pub fn has<K: IntoVal<Env, Val>>(env: &Env, key: &K) -> bool {
        env.storage().persistent().has(key)
    }
    pub fn bump<K: IntoVal<Env, Val>>(env: &Env, key: &K, min_ledger_to_live: u32) {
        env.storage().persistent().bump(key, min_ledger_to_live)
    }
    pub fn bump_until<K: IntoVal<Env, Val>>(env: &Env, key: &K, expiration_ledger: u32) {
        env.storage().persistent().bump(
            key,
            expiration_ledger
                .checked_sub(env.ledger().sequence())
                .unwrap(),
        )
    }
    pub fn remove<K: IntoVal<Env, Val>>(env: &Env, key: &K) {
        env.storage().persistent().remove(key);
    }
}

pub struct Temporary;
impl Temporary {
    pub fn get<K: IntoVal<Env, Val>, V: TryFromVal<Env, Val>>(env: &Env, key: &K) -> Option<V> {
        env.storage().temporary().get::<K, V>(key)
    }
    pub fn set<K: IntoVal<Env, Val>, V: IntoVal<Env, Val>>(env: &Env, key: &K, val: &V) {
        env.storage().temporary().set(key, val);
    }
    pub fn has<K: IntoVal<Env, Val>>(env: &Env, key: &K) -> bool {
        env.storage().temporary().has(key)
    }
    pub fn bump<K: IntoVal<Env, Val>>(env: &Env, key: &K, min_ledger_to_live: u32) {
        env.storage().temporary().bump(key, min_ledger_to_live)
    }
    pub fn bump_until<K: IntoVal<Env, Val>>(env: &Env, key: &K, expiration_ledger: u32) {
        env.storage().temporary().bump(
            key,
            expiration_ledger
                .checked_sub(env.ledger().sequence())
                .unwrap(),
        )
    }
    pub fn remove<K: IntoVal<Env, Val>>(env: &Env, key: &K) {
        env.storage().temporary().remove(key);
    }
}
