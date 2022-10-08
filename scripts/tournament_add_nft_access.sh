#!/bin/bash
source neardev/dev-account.env
source neardev/dev-contracts.env

TOURNAMENT_ID="1"
TOKEN_IDS="\"1\""

near call $CONTRACT_NAME tournament_add_nft_access --accountId $OWNER_ID "{ \"tournament_id\": \"$TOURNAMENT_ID\", \"token_ids\": [$TOKEN_IDS] }"
