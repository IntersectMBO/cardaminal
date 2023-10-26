# Transaction Building

## Context

We want to support both a more interactive, user-friendly transaction building flow and also a transaction flow which is easily scriptable.

## Decision

The interactive flow will be a user-friendly wrapper around the more scriptable commands which are used to build a transaction. We will use the following script-friendly commands for managing transactions.

Note that `TX_ID` refers to the unique identifier assigned by Cardaminal for the transaction which is used to identify transactions in the staging area, and not the transaction hash which is created when a transaction is built.

## General Commands

### Create transaction

Create a new empty transaction in the transaction staging area for the specified chain.

**Usage:**
```
cardaminal transaction new <CHAIN_NAME>
```

**Returns:** the unique identifier for the created transaction (within the staging area), used to specify which transaction in the staging area you wish to issue build commands against.

### List transactions in staging area

List transactions which are in the staging area, along with some information summary regarding the transaction.

**Usage:**
```
cardaminal transaction list
```

**Flags:**
```
--status <STATUS>       only return transactions with the specified status
```

**Returns:** list of transactions in the staging area, with information such as chain, when the transaction was last updated, status (building, signing, submitted, ...), and potentially other useful information.

### Delete transaction

Remove a transaction from the transaction staging area.

**Usage:**
```
cardaminal transaction delete <TX_ID>
```

### Inspect transaction

Return detailed information on a specific transaction in the staging area.

**Usage:**
```
cardaminal transaction inspect <TX_ID>
```

**Returns:** all information about the transaction

## Transaction Building Commands

### Add input

Add an input to a transaction.

**Usage:**
```
cardaminal transaction input add <TX_ID> <UTXO_HASH> <UTXO_IDX>
```

### Remove input

Remove an input from a transaction.

**Usage:**
```
cardaminal transaction input remove <TX_ID> <UTXO_HASH> <UTXO_IDX>
```

### Add reference input

Add a reference input to a transaction.

**Usage:**
```
cardaminal transaction reference-input add <TX_ID> <UTXO_HASH> <UTXO_IDX>
```

### Remove reference input

Remove a reference input from a transaction.

**Usage:**
```
cardaminal transaction reference-input remove <TX_ID> <UTXO_HASH> <UTXO_IDX>
```

### Add output

Add an output to a transaction.

**Usage:**
```
cardaminal transaction output add <TX_ID> <ADDRESS> <LOVELACE> <ASSET_A_POLICY><ASSET_A_NAME>:<ASSET_A_AMOUNT> <ASSET_B_POLICY><ASSET_B_NAME>:<ASSET_B_AMOUNT> ...
```

**Flags:**
```
--datum <"inline" | "hash"> <BYTES>, OR
--datum-file <"inline" | "hash"> <FILE>

--reference-script <"native" | "pv1" | ...> <BYTES>, OR
--reference-script-file <"native" | "pv1" | ...> <FILE>
```

**Returns:** the output index of the added output.

### Remove output

Remove an output from a transaction.

**Usage:**
```
cardaminal transaction output remove <TX_ID> <OUTPUT_INDEX>
```

### Set transaction fee

Manually set the transaction fee of a transaction. If no fee set Cardaminal will attempt to compute the fee.

**Usage:**
```
cardaminal transaction fee set <TX_ID> <LOVELACE>
```

### Remove transaction fee

Clear/remove the transaction fee of a transaction. If no fee set Cardaminal will attempt to compute the fee.

**Usage:**
```
cardaminal transaction fee clear <TX_ID>
```

### Add asset mint

Add an asset and amount to the transaction mint set, negative value for burn.

**Usage:**
```
cardaminal transaction mint add <TX_ID> <ASSET_POLICY><ASSET_NAME> <MINT_AMOUNT>
```

### Remove asset mint

Remove an asset from the transaction mint set.

**Usage:**
```
cardaminal transaction mint remove <TX_ID> <ASSET_POLICY><ASSET_NAME>
```

### Set valid-hereafter

Set the valid-hereafter slot of the transaction (the slot before which the transaction cannot be included in the chain)

**Usage:**
```
cardaminal transaction valid-hereafter set <TX_ID> <SLOT>
```

### Remove valid-hereafter

Clear/remove the valid-hereafter slot of a transaction.

**Usage:**
```
cardaminal transaction valid-hereafter clear <TX_ID>
```

### Set TTL

Set the TTL of the transaction (slot at which the transaction can no longer be included in the chain)

**Usage:**
```
cardaminal transaction ttl set <TX_ID> <SLOT>
```

### Remove TTL

Clear/remove the TTL of a transaction.

**Usage:**
```
cardaminal transaction ttl clear <TX_ID>
```

### TODO: Certificates

### TODO: Withdrawals

### Set network ID

Set the network ID of a transaction.

**Usage:**
```
cardaminal transaction network set <TX_ID> <NETWORK_ID>
```

### Remove network ID

Clear/remove the network ID of a transaction.

**Usage:**
```
cardaminal transaction network clear <TX_ID>
```

### Add collateral input

Add an input to a transaction.

**Usage:**
```
cardaminal transaction collateral-input add <TX_ID> <UTXO_HASH> <UTXO_IDX>
```

### Remove collateral input

Remove an input from a transaction.

**Usage:**
```
cardaminal transaction collateral-input remove <TX_ID> <UTXO_HASH> <UTXO_IDX>
```

### Set collateral return

Set the collateral return output of a transaction.

**Usage:**
```
cardaminal transaction collateral-return set <TX_ID> <ADDRESS> <LOVELACE>
```

### Remove collateral return

Clear/remove the network ID of a transaction.

**Usage:**
```
cardaminal transaction collateral-return clear <TX_ID>
```

### Add disclosed signer (required signer)

Add a public key hash to the required signers set of a transaction, so that it is disclosed to any Plutus scripts being executed by the transaction.

**Usage:**
```
cardaminal transaction disclosed-signer add <TX_ID> <PUBKEYHASH>
```

### Remove disclosed signer (required signer)

Remove a public key hash from the required signers set of a transaction.

**Usage:**
```
cardaminal transaction disclosed-signer remove <TX_ID> <PUBKEYHASH>
```

### Add script

Add a native or Plutus script to a transaction (witness set).

**Usage:**
```
cardaminal transaction script add <TX_ID> <"native" | "pv1" | ...> (--bytes <BYTES> | --file <FILE>)
```

**Returns:** the script hash of the added script.

### Remove script

Remove a script from a transaction (witness set).

**Usage:**
```
cardaminal transaction script remove <TX_ID> <SCRIPT_HASH>
```

### Add datum

Add a datum to a transaction (witness set).

**Usage:**
```
cardaminal transaction datum add <TX_ID> (--bytes <BYTES> | --file <FILE>)
```

**Returns:** the datum hash of the added datum.

### Remove datum

Remove a datum from a transaction (witness set).

**Usage:**
```
cardaminal transaction datum remove <TX_ID> <DATUM_HASH>
```

### Add redeemer

Add a redeemer to a transaction. If execution unit budget not specified Cardaminal will attempt to compute the required budget.

**Usage (spend redeemer):**
```
cardaminal transaction redeemer add spend <TX_ID> <UTXO_HASH> <UTXO_IDX>
```

**Usage (mint redeemer):**
```
cardaminal transaction redeemer add mint <TX_ID> <POLICY_ID>
```

**TODO: certificate redeemer**

**TODO: withdrawal redeemer**

**Flags:**
```
--budget <MEM_BUDGET> <STEPS_BUDGET>
```

### Remove redeemer

Remove a redeemer from a transaction.

**Usage (spend redeemer):**
```
cardaminal transaction redeemer remove spend <TX_ID> <UTXO_HASH> <UTXO_IDX>
```

**Usage (mint redeemer):**
```
cardaminal transaction redeemer remove mint <TX_ID> <POLICY_ID>
```

**TODO: certificate redeemer**

**TODO: withdrawal redeemer**

### Override expected number of transaction signers

For the purpose of fee computation Cardaminal will attempt to compute the minimum required number of signers the transaction has, but this can be overrided if more signatures will be attached to the transaction to ensure the fee computation is sufficient.

**Usage:**
```
cardaminal transaction override-signers-amount set <TX_ID> <NUMBER_OF_SIGNERS>
```

### Remove overrided expected number of transaction signers

Clear the overrided expected number of transaction signers for a transaction.

**Usage:**
```
cardaminal transaction override-signers-amount remove <TX_ID>
```

### Set change address

Set the change address for the transaction which will be used if Cardaminal is responsible for balancing the transaction and/or computing the transaction fee. An output will be created sending any unclaimed value to the change address.

**Usage:**
```
cardaminal transaction change-address set <TX_ID> <ADDRESS>
```

### Remove change address

Clear/remove the change address for a transaction.

**Usage:**
```
cardaminal transaction change-address clear <TX_ID>
```

## Finalizing and Signing

### Build transaction

Build/finalize a transaction in the staging area so that it is ready for signatures to be attached.

**Usage:**
```
cardaminal transaction build <TX_ID>
```

**Returns:** transaction hash of the built transaction

### Sign transaction

Sign a transaction using a Cardaminal wallet.

**Usage:**
```
cardaminal transaction sign <TX_ID> <WALLET_NAME>
```

### Add transaction signature

Manually add an already created signature to a built transaction.

**Usage:**
```
cardaminal transaction signatures add <TX_ID> <PUBLIC_KEY> <SIGNATURE>
```

### Remove transaction signature

Remove a signature from a built transaction.

**Usage:**
```
cardaminal transaction signatures remove <TX_ID> <PUBLIC_KEY>
```

### Submit transaction

Submit a transaction to the specified chain.

**Usage:**
```
cardaminal transaction submit <TX_ID> <CHAIN_NAME>
```