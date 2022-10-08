#!/bin/bash
source neardev/dev-account.env
source neardev/dev-contracts.env

TOURNAMENT_ID="1"
WINNER="muzikant.testnet"
WINNER_PLACE="1"

near call $CONTRACT_NAME tournament_execute_reward --accountId $OWNER_ID "{ \"tournament_id\": \"$TOURNAMENT_ID\", \"winner_place\": $WINNER_PLACE, \"account_id\": \"$WINNER\" }"
