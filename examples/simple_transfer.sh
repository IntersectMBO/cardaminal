#!/bin/bash
set -e

wallet=$1
to_address=$2
to_transfer=$3

# start a new tx from scratch
txid=$(cardaminal tx create $wallet)

# send the specified amount to the external address
cardaminal tx edit $wallet $txid add-output $to_address $to_transfer

# select an utxo from our wallet to use as input
input=$(cardaminal wallet select $wallet --min-lovelace $to_transfer)
cardaminal tx edit $wallet $txid add-input $input

# define the fee for the tx (TODO: fee estimation via cardaminal command)
cardaminal tx edit $wallet $txid set-fee 3000000

# return the remaining value back to my address
change=$(cardaminal tx balance $wallet $txid -a)
my_address=$(cardaminal wallet address $wallet --testnet)
cardaminal tx edit $wallet $txid add-output $my_address $change

# print the tx as json
cardaminal tx inspect $wallet $txid | jq

# turn the tx into cbor
cardaminal tx build $wallet $txid

echo "tx id: $txid"
