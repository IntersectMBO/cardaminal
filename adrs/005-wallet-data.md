# Wallet Data

## Decision

We will have multiple wallets, each maintains its own database of UTxOs and transaction history per chain it is attached to. We will use sqlite for the wallets, but we can explore rocksdb if we need to squeeze more performance out. 1 sqlite db per wallet-chain pairing, in each db is one table for UTxOs, one table for transaction history, one for recent points processed so we can intersect with the local chain when we want to update the wallet, one for the current protocol parameters which are needed for tx building/fee calculating/script execution.

### UTxO table
In wallet-chain database, store UTxOs of the wallet on the chain, which can be queried by payment credential or stake credential

```
	[ID] (PK)
	[utxo_txhash]
	[utxo_txidx]
	[payment credential]
	[stake credential]?
	[slot] (created at slot, we can rollback by removing utxos after rollback slot)
	[era] (so we know how to decode output cbor correctly)
	[outputcbor]
```


### Tx History Table
In wallet-chain database, store transactions involving the wallet on the chain

```
	[ID] (PK)
	[txhash]
	[slot] (to order the history)
	[tx index in block] (to order the history)
	[balance change] (value cbor?)
```

### Recent points table
In wallet-chain database, we need to store the recent blocks processed by the wallet so that we can intersect with the local chain (which may have rollbacked the current tip of the wallet). We will trim this after X blocks or Y slots from tip.

```
	[slot]
	[block hash]
```

### Chain protocol parameters
If we are syncing wallets separately to the local chain, it only really seems to make sense for the wallet to keep the current protocol parameters for each attached chain as of the point it has synced to. Seeing as the structure of this changes through the eras its not completely clear the best way to store them. We could store the most recent transaction which contained a protocol

### Queries

- Store UTxOs: We need to write and delete keys for UTxOs relating to the wallet when processing blocks from the local chain
- Fetch UTxOs for wallet: We have a wallet, which is some initial “key” for payment keys and maybe a stake “key”. It has a single payment key (for now). We fetch UTxOs from the UTxO table.
- Store History: For each transaction in processed blocks, we need to write keys (address, txhash, slot, tx index in block, balance change) for any wallet address linked to the chain which was involved in the transaction
- Fetch history: Query the transaction history for the chain wallet
- Store points: For every new block processed write a key 
- Fetch recent points for intersection: On start up, fetch all the recently processed points for a chain and then use these to intersect with the chain
- Trim intersection points: Trim intersection points after X points or Y slots from cursor
- Rollback to slot X: Remove all entries from all tables with slot greater than X

