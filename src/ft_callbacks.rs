use near_sdk::serde::{ Deserialize, Serialize };
use near_sdk::{ AccountId, PromiseOrValue, env, near_bindgen };
use crate::*;
use near_sdk::json_types::U128;
use mfight_sdk::tournament::TournamentOnFtTransferArgs;
use mfight_sdk::ft::FungibleTokenReceiver;
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
enum Args {
    Tournament(TournamentOnFtTransferArgs),
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String
    ) -> PromiseOrValue<U128> {
        let ft_contract_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();

        assert_ne!(
            ft_contract_id,
            signer_id,
            "nft_on_approve should only be called via cross-contract call"
        );
        assert_eq!(&sender_id, &signer_id, "owner_id should be signer_id");

        match near_sdk::serde_json::from_str(&msg).expect("Invalid Args") {
            Args::Tournament(tournamentArgs) => {
                self.tournament.internal_on_ft_transfer(&tournamentArgs, &ft_contract_id, &amount, &sender_id)
            }
        }
    }
}
