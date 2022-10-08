#!/usr/bin/env bash

set -e

sh ./build.sh

#near deploy --accountId mfight-tournament.testnet --wasmFile ../res/tournament.wasm
near dev-deploy ../res/tournament.wasm

exit 0
