#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::Events, vec, Env, IntoVal};

#[test]
fn test_set_value_emits_event() {
    let env = Env::default();
    let contract_id = env.register_contract(None, GwizaStorageContract);
    let client = GwizaStorageContractClient::new(&env, &contract_id);

    let key = symbol_short!("name");
    let value = symbol_short!("Gwiza");

    client.set_value(&key, &value);

    // Verify the event
    // env.events().all() returns a Vec of (ContractId, Topics, Data)
    let events = env.events().all();
    assert_eq!(events.len(), 1);

    let event = events.first().unwrap();

    // Verify the contract ID (event.0) matches
    assert_eq!(event.0, contract_id);

    // Verify the topics (event.1): ["set", key]
    // Note: Topics are passed as a tuple in publish, stored as Vec<Val>
    assert_eq!(event.1, vec![&env, symbol_short!("set").into_val(&env), key.into_val(&env)]);

    // Verify the data (event.2): value
    assert_eq!(event.2, value);
}
