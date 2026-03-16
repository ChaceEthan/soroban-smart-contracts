# Gwiza Storage Soroban Contract

This is a simple storage smart contract for Soroban on the Stellar network. It allows storing and retrieving key-value pairs and emits an event when a value is set.

Features:
- `set_value(key: Symbol, value: Symbol)`: Stores a value under a given key and emits a `("set", key)` event.
- `get_value(key: Symbol) -> Option<Symbol>`: Retrieves the value for a given key.
- `greet(name: Symbol) -> Vec<Symbol>`: A simple function that returns a greeting.