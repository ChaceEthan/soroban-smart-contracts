#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, vec, Env, Symbol, Vec};

/// Defines the contract's data keys for storage.
#[contracttype]
pub enum DataKey {
    Value(Symbol),
}

#[contract]
pub struct GwizaStorageContract;

#[contractimpl]
impl GwizaStorageContract {
    pub fn set_value(env: Env, key: Symbol, value: Symbol) {
        // Set the value in storage
        env.storage().instance().set(&DataKey::Value(key.clone()), &value);

        // Publish an event to notify listeners that a value was set.
        // Topics are indexed and searchable, while data is the payload.
        // Here, we use "set" and the key as topics.
        env.events()
            .publish((symbol_short!("set"), key), value);
    }

    pub fn get_value(env: Env, key: Symbol) -> Option<Symbol> {
        env.storage().instance().get(&DataKey::Value(key))
    }

    pub fn greet(env: Env, name: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), name]
    }
}

#[cfg(test)]
mod test;
