#!/bin/bash
source neardev/dev-account.env
source neardev/dev-contracts.env

TOURNAMENT_ID="1"
PLACE="1"
AMOUNT="1000000000000000000000000"

near call $CONTRACT_NAME tournament_add_prize --accountId $OWNER_ID "{ \"tournament_id\": \"$TOURNAMENT_ID\", \"owner_id\": \"$OWNER_ID\", \"place_number\": $PLACE }" --depositYocto $AMOUNT
