#!/bin/bash
source neardev/dev-account.env
source neardev/dev-contracts.env

TOURNAMENT_ID="1"
ACCOUNT_ID="muzikant.testnet"
PRICE="1000000000000000000000000"

near call $CONTRACT_NAME tournament_join --accountId $ACCOUNT_ID "{ \"tournament_id\": \"$TOURNAMENT_ID\", \"owner_id\": \"$OWNER_ID\" }" --depositYocto $PRICE
