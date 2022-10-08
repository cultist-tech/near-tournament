#!/bin/bash
source neardev/dev-account.env
source neardev/dev-contracts.env

TOURNAMENT_ID="1"
ACCOUNT_ID="muzikant.testnet"

near call $CONTRACT_NAME tournament_start --accountId $OWNER_ID "{ \"tournament_id\": \"$TOURNAMENT_ID\" }"
