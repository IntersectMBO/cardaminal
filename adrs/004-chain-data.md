# Chain Data

## Decision

For the local chains which will store all the blocks, we will use RollDB from Dolos/Pallas which will store full blocks, which uses rocksdb.

We will have multiple chains, each which maintains its own database of all blocks it has synced (from the remote node) in rocksdb. To intersect with the remote node’s chain, it can use the most recent entries in the block database.

