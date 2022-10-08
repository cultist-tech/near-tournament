#!/bin/bash
source neardev/dev-account.env
source neardev/dev-contracts.env

TOURNAMENT_ID="1"
PLACE="1"
AMOUNT="5000000000000000000000000"

NAME="test"
PLAYERS="1"
PRICE="1000000000000000000000000"

near call $CONTRACT_NAME tournament_create --accountId $OWNER_ID "{ \"tournament_id\": \"$TOURNAMENT_ID\", \"name\": \"$NAME\", \"players_number\": $PLAYERS, \"price\": \"$PRICE\" }"
near call $CONTRACT_NAME tournament_add_prize --accountId $OWNER_ID "{ \"tournament_id\": \"$TOURNAMENT_ID\", \"place_number\": $PLACE }" --depositYocto $AMOUNT
