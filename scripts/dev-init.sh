#!/bin/bash
source neardev/dev-account.env

ACCOUNT_ID="muzikant.testnet"

near call $CONTRACT_NAME new_with_default_meta --accountId $ACCOUNT_ID "{ \"owner_id\": \"$CONTRACT_NAME\" }"
