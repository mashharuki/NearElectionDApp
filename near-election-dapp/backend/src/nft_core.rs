use crate::*;

// trait of NonFungibleTokenCore
pub trait NonFungibleTokenCore {
    // get nft info
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken>;
    // transfer
    fn nft_transfer(&mut self, receiver_id: AccountId, token_id: TokenId,);
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {
    // get token info
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken> {

        if let Some(token) = self.tokens_by_id.get(&token_id) {
            // get metadata
            let metadata = self.token_metadata_by_id.get(&token_id).unwrap();
            // return
            Some(JsonToken {
                owner_id: token.owner_id,
                metadata,
            })
        } else {
            None
        }
    }

    // transfer function
    #[payable]
    fn nft_transfer(&mut self, receiver_id: AccountId, token_id: TokenId,) {
        assert!(
            !(&self.is_election_closed),
            "You can no longer vote because it's been closed!"
        );

        assert_one_yocto();
        // get sender ID
        let sender_id = env::predecessor_account_id();
        // transfer
        self.internal_transfer(&sender_id, &receiver_id, &token_id);
    }
}