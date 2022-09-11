# Testnet Spawner

This was a quick project to solidify some Rust fundamentals, though it has promise as a productionized service.

The testnet spawner deploys ad-hoc Ethereum testnets at a user's request.
The API is simple:
1. `initialize`: Spins up a new testnet, either with a default empty state, or with a state forked off of mainnet, and exposes an RPC URL for sending transactions to the testnet.
2. `reset`: Reset a given testnet to its initial state.
3. `destroy`: Tears down a given testnet.

Currently, everything runs locally: both the server, and the testnets. However, if this were slightly re-architected to run in a cloud environment, it could be a pretty nifty tool for spawning ephemeral testnets.

## Future Considerations
- Accept a state hex dump to `initialize` and/or `reset` for manual configuration of the testnet EVM state.
- Have `initialize` provision infrastructure to run a testnet.
- Move the mapping of RPC URL -> (testnet process, snapshot id) to a database rather than in-memory
    - Would only be justified by massive scale IMO, since the memory requirements are light.