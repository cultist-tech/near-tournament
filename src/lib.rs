extern crate near_sdk;
extern crate mfight_sdk;

use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedSet, UnorderedMap, TreeMap};
use near_sdk::json_types::{U128, U64};
use near_sdk::{
    env, near_bindgen, AccountId,  CryptoHash, PanicOnDefault,  BorshStorageKey
};
use mfight_sdk::whitelist::WhitelistFeature;
use mfight_sdk::tournament::{TournamentFactoryMetadata, TournamentFactory, TournamentId, Tournament, TournamentMetadata, TournamentPrizeId, RewardPrize, WinnerPlace};

mod ft_callbacks;
mod nft_callbacks;

pub type TokenId = String;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    metadata: LazyOption<TournamentFactoryMetadata>,

    tournament: TournamentFactory,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
  PlayersPerTournament,
  TournamentsById,
  TournamentMetadataById,
  TournamentFactoryMetadata,
  TournamentPrizes,
  TournamentPrizesPer,
  TournamentRewardedPrizesByPlace,
  TournamentAccessNft,
  TournamentsByOwner,
  TournamentWhitelistPrizeOwners,
}

#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]
    pub fn new_with_default_meta(owner_id: AccountId) -> Self {
        //calls the other function "new: with some default metadata and the owner_id passed in
        Self::new(
            owner_id,
            TournamentFactoryMetadata {
                name: "Tournaments Contract".to_string(),
                icon: None,
            },
        )
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id.
    */
    #[init]
    pub fn new(owner_id: AccountId,  metadata: TournamentFactoryMetadata) -> Self {
        let metadata = LazyOption::new(
            StorageKey::TournamentFactoryMetadata.try_to_vec().unwrap(),
                Some(&metadata),
        );

        let tournament = TournamentFactory::new(
            StorageKey::PlayersPerTournament,
            StorageKey::TournamentPrizes,
            StorageKey::TournamentPrizesPer,
            StorageKey::TournamentRewardedPrizesByPlace,
            StorageKey::TournamentsById,
            StorageKey::TournamentMetadataById,
            StorageKey::TournamentAccessNft,
            StorageKey::TournamentsByOwner,
          StorageKey::TournamentWhitelistPrizeOwners,
        );

        Self {
            owner_id: owner_id.clone(),
            tournament,
            metadata,
        }
    }

  #[init(ignore_state)]
  #[private]
  pub fn migrate() -> Self {
    #[derive(BorshDeserialize, BorshSerialize)]
    pub struct OldTournamentFactory {
      //keeps track of all the players IDs for a given tournament
      pub players_per_tournament: LookupMap<TournamentId, UnorderedSet<AccountId>>,

      //keeps track of the tournament struct for a given tournament ID
      pub tournaments_by_id: TreeMap<TournamentId, Tournament>,

      //keeps track of the tournament metadata for a given tournament ID
      pub tournament_metadata_by_id: UnorderedMap<TournamentId, TournamentMetadata>,

      //

      pub prize_by_id: LookupMap<TournamentPrizeId, RewardPrize>,

      pub prizes_by_tournament: LookupMap<TournamentId, TreeMap<WinnerPlace, UnorderedSet<TournamentPrizeId>>>,

      pub prizes_by_tournament_rewarded: UnorderedSet<TournamentPrizeId>,

      //

      pub tournament_access_nft: LookupMap<TournamentId, UnorderedSet<TokenId>>,

      pub tournaments_by_owner: TreeMap<AccountId, UnorderedSet<TournamentId>>,

      pub whitelist_prize_owners: LookupMap<TournamentId, WhitelistFeature>,
    }

    #[derive(BorshDeserialize)]
    struct Old {
      owner_id: AccountId,
      metadata: LazyOption<TournamentFactoryMetadata>,

      tournament: OldTournamentFactory,
    }

    let old: Old = env::state_read().expect("Error");

    Self {
      owner_id: old.owner_id,
      metadata: old.metadata,
      tournament: TournamentFactory {
        players_per_tournament: old.tournament.players_per_tournament,
        tournaments_by_id: old.tournament.tournaments_by_id,
        tournament_metadata_by_id: old.tournament.tournament_metadata_by_id,
        prize_by_id: old.tournament.prize_by_id,
        prizes_per_place_rewarded: UnorderedSet::new(StorageKey::TournamentRewardedPrizesByPlace),
        prizes_by_tournament: old.tournament.prizes_by_tournament,
        tournament_access_nft: old.tournament.tournament_access_nft,
        tournaments_by_owner: old.tournament.tournaments_by_owner,
        whitelist_prize_owners: old.tournament.whitelist_prize_owners,
      }
    }
  }
}

pub trait ContractMetadata {
    //view call for returning the contract metadata
    fn tournament_metadata(&self) -> TournamentFactoryMetadata;
}

#[near_bindgen]
impl ContractMetadata for Contract {
    fn tournament_metadata(&self) -> TournamentFactoryMetadata {
        self.metadata.get().unwrap()
    }
}

mfight_sdk::impl_tournament_contract_core!(Contract, tournament);
mfight_sdk::impl_tournament_contract_enumeration!(Contract, tournament);
mfight_sdk::impl_tournament_nft_access!(Contract, tournament);
