use near_sdk::{env, near_bindgen, AccountId, PromiseOrValue};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::from_str;
use mfight_sdk::nft::base::NonFungibleTokenReceiver;
use crate::*;
use mfight_sdk::tournament::TournamentOnNftTransferArgs;
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
enum Args {
  Tournament(TournamentOnNftTransferArgs),
}

#[near_bindgen]
impl NonFungibleTokenReceiver for Contract {
    /// where we add the sale because we know nft owner can only call nft_approve

    fn nft_on_transfer(
      &mut self,
      sender_id: AccountId,
      previous_owner_id: AccountId,
      token_id: mfight_sdk::metadata::TokenId,
      msg: String,
    ) -> PromiseOrValue<bool> {
      let nft_contract_id = env::predecessor_account_id();
      let signer_id = env::signer_account_id();

      assert_ne!(
          nft_contract_id,
          signer_id,
          "nft_on_approve should only be called via cross-contract call"
      );
      assert_eq!(
          &sender_id,
          &signer_id,
          "owner_id should be signer_id"
      );

      match near_sdk::serde_json::from_str(&msg).expect("Invalid Args") {
        Args::Tournament(marketArgs) => {
          self.tournament.internal_on_nft_transfer(&marketArgs, &nft_contract_id, &token_id, &sender_id)
        }
      }
    }
}
