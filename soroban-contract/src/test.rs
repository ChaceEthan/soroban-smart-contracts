#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Events, vec, Env, IntoVal};

#[test]
fn test_set_and_get_value() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GwizaStorageContract);
    let client = GwizaStorageContractClient::new(&env, &contract_id);

    let key = symbol_short!("color");
    let value = symbol_short!("blue");

    // 1. Test get_value when nothing is set
    assert_eq!(client.get_value(&key), None);

    // 2. Test set_value
    client.set_value(&key, &value);

    // 3. Test get_value after setting
    assert_eq!(client.get_value(&key), Some(value.clone()));

    // 4. Verify that the correct event was published
    let events = env.events().all();
    assert_eq!(events.len(), 1);

    let event = events.first().unwrap();
    assert_eq!(event.0, contract_id); // Contract ID
    assert_eq!(event.1, vec![&env, symbol_short!("set").into_val(&env), key.into_val(&env)]); // Topics
    assert_eq!(event.2, value.into_val(&env)); // Data
}

#[test]
fn test_greet() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GwizaStorageContract);
    let client = GwizaStorageContractClient::new(&env, &contract_id);

    let name = symbol_short!("friend");
    let response = client.greet(&name);

    assert_eq!(response, vec![&env, symbol_short!("Hello"), name]);
}