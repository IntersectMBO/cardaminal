# Command Hierarchy

## Context

Our CLI will provide several heterogeneous commands. Each command has it's own set of args and conventions. We need a hierarchy of nested subcommands to facilitate the tasks of navigating through all options.

## Decision

The command line will be divided into multiple subcommands:

- `chain`: to manage available chains and their data
  - `create` create a new chain config
  - `list`: list all chains configued
  - `delete`: delete a chain by name
  - `update`: sync a chain to latest point
- `wallet`: to manage available wallets and their data
  - `create`: create a new wallet
  - `list`: list available wallets
  - `update`: update wallet state from chain
  - `attach`: attach existing wallet to chain
  - `detach`: detach existing wallet from chain
  - `show`: show wallet history
  - `list`: list current utxos of a wallet
- `transaction`: to build, edit, sign and submit transactions
  - `build`: build a new transaction
  - `edit`: edit existing transaction
  - `sign`: sign pending transaction
  - `submit`: submit pending transaction
