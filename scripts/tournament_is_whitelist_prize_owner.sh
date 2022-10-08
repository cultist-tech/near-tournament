#!/bin/bash
source neardev/dev-account.env
source neardev/dev-contracts.env

TOURNAMENT_ID="1"
ACCOUNT_ID="muzikant.testnet"

near view $CONTRACT_NAME tournament_is_whitelist_prize_owner --accountId $CONTRACT_NAME "{ \"tournament_id\": \"$TOURNAMENT_ID\", \"owner_id\": \"$OWNER_ID\", \"account_id\": \"$ACCOUNT_ID\" }"
