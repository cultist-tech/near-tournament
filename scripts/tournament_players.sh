#!/bin/bash
source neardev/dev-account.env
source neardev/dev-contracts.env

TOURNAMENT_ID="1"

near view $CONTRACT_NAME tournament_players --accountId $CONTRACT_NAME "{ \"tournament_id\": \"$TOURNAMENT_ID\", \"owner_id\": \"$OWNER_ID\" }"
