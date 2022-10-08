#!/bin/bash
source neardev/dev-account.env
source neardev/dev-contracts.env

TOURNAMENT_ID="123"
NAME="123"
PLAYERS="3"
PRICE="1000000000000000000000000"
NFT_ACCESS="mfight-nft_v2.testnet"

near call $CONTRACT_NAME tournament_create --accountId $OWNER_ID "{ \"tournament_id\": \"$TOURNAMENT_ID\", \"name\": \"$NAME\", \"players_number\": $PLAYERS, \"price\": \"$PRICE\", \"nft_access_contract\": \"$NFT_ACCESS\" }" --deposit "0.1"
