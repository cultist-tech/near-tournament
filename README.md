# TOURNAMENT

Необходимо для проведения различных турниров на блокчейне. Создается турнир(доступ по NFT, как опция) с определенными условиями (количество участников, цена входа, распределение средств между победителями). Каждый участник вносит определенную ранее сумму, после окончания турнира, все собранные средства распределяются согласно условиям турнира. Так же любой человек может внести FT или NFT в призовой фонд турнира.

```rust
pub trait TournamentFactoryCore {
  // Создание нового турнира
  fn tournament_create(
    &mut self,
    // id турнира
    tournament_id: TournamentId,
    // количество участников
    players_number: u8,
    // цена входа (может ее не быть)
    price: Option<U128>,
    // название турнира
    name: String,
    // превью турнира
    media: Option<String>,
    // описание турнира
    summary: Option<String>,
  );

  // получить информацию о турнире
  fn tournament(&self,
    // id турнира
    tournament_id: TournamentId,
    // создатель турнира
    owner_id: AccountId,
  ) -> JsonTournament;

  // запустить турнир
  fn tournament_start(&mut self, tournament_id: TournamentId, owner_id: AccountId);

  // завершить турнир
  fn tournament_end(&mut self, tournament_id: TournamentId, owner_id: AccountId);

  // присмотр призов за каждое место в турнире (может быть nft, ft или near)
  fn tournament_prizes(&self, tournament_id: TournamentId, owner_id: AccountId) -> HashMap<WinnerPlace, Vec<RewardPrize>>;

  // присоедениться к турниру
  fn tournament_join(&mut self, tournament_id: TournamentId, owner_id: AccountId);

  // добавить приз в турнире на определенное место
  fn tournament_add_prize(&mut self, tournament_id: TournamentId, owner_id: AccountId, place_number: u8);

  // узнать сколько свободных мест в турнире
  fn tournament_free_places(&self, tournament_id: TournamentId, owner_id: AccountId) -> Option<U64>;

  // указать какое место занял один из участников а так же передать ему призы за место
  fn tournament_execute_reward(&mut self, tournament_id: TournamentId, owner_id: AccountId, winner_place: u8, account_id: AccountId);

  // узнать находится ли пользователь в турнире
  fn tournament_member(&self, tournament_id: TournamentId, owner_id: AccountId, account_id: AccountId) -> bool;
}

pub trait TournamentFactoryEnumeration{
  // список турниров
  fn tournaments(&self, owner_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonTournament>;
  // список игроков в турнире
  fn tournament_players(&self, tournament_id: TournamentId, owner_id: AccountId) -> Vec<AccountId>;
}

pub trait TournamentFactoryNftAccess {
  // список nft токенов являющихся доступов к турниру
  fn tournament_nft_access(&self, tournament_id: TournamentId, owner_id: AccountId) -> Vec<TokenId>;

  // добавить nft токен в список доступов
  fn tournament_add_nft_access(&mut self, tournament_id: TournamentId, owner_id: AccountId, token_ids: Vec<TokenId>);
}

trait FungibleTokenReceiver {
    // перевод ft токена в призовой пулл
    fn ft_on_transfer(&mut self, sender_id: AccountId, amount: U128, msg: String) -> PromiseOrValue<U128>;
}

trait NonFungibleTokenApprovalsReceiver {
   // перевод nft токена в призовой пулл, либо использование как nft доступ для входа в турнир
  fn nft_on_transfer(
    &mut self,
    sender_id: AccountId,
    previous_owner_id: AccountId,
    token_id: TokenId,
    msg: String,
  ) -> PromiseOrValue<bool>;
}
```
