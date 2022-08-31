# Testnet Spawner

This is a service that deploys ad-hoc testnets at a user's requests.
The API is simple:
1. `initialize`: Spins up a new testnet, either with a default empty state, with a state forked off of mainnet, or with a state given by a provided snapshot. Exposes an RPC URL for sending transactions to the testnet.
2. `reset`: Reset a given testnet's state to a provided state snapshot.
3. `destroy`: Tears down a given testnet.