use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_sdk::ext_contract;

/// External interface for interaction with NFT NEP-171 compatible contracts.
#[ext_contract(nft)]
trait NonFungibleToken {
    fn nft_token(&self, token_id: TokenId) -> Option<Token>;
}
