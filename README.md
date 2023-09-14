
# Cardaminal

Cardaminal is a CLI-based Cardano wallet tailored for power-users and developers.

## High-Level Goals

- Low-level, granular access to wallet state
- Promotes programmability and automation
- Easy integration with script via shell
- Support for mainnet and common testnets
- Supports custom chains via manual config
- Transaction building via CLI data inputs

## Commands

Preliminary list of available commands (subject to change)

### Syncing chain:
- Update chain: download new blocks : chain update <CHAIN-NAME>
- List chains: chain list
- Create chains: chain create <CHAIN-NAME> ..

### Manipulating wallet:
- Create a wallet, with mnemonics : wallet create <WALLET-NAME>
- Forward / Reverse from the local chain : wallet update
- Attach / Detach a wallet to a chain : wallet attach <WALLET-NAME> <CHAIN-NAME> / wallet detach <WALLET-NAME>
- List : show UTXOs availables : wallet list-utxos <WALLET-NAME>
- History : show events of your wallet : wallet history <WALLET-NAME>
- Create Transaction : create a transaction with given utxos, sender, etc[g][h]

## Project

Project activity is tracked on this [Github project](https://github.com/orgs/txpipe/projects/15)