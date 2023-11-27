#!/bin/bash

chain=$1

while true
do
   cardaminal chain sync $chain
   echo "sleeping for a while"
   sleep 120
done