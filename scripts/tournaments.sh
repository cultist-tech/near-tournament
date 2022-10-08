#!/bin/bash
source neardev/dev-account.env
source neardev/dev-contracts.env

FROM_INDEX="0"
LIMIT="10"

near view $CONTRACT_NAME tournaments --accountId $CONTRACT_NAME "{ \"owner_id\": \"$OWNER_ID\", \"from_index\": \"$FROM_INDEX\", \"limit\": $LIMIT }"
