use crate::*;

// trait of NonFungibleTokenCore
pub trait NonFungibleTokenCore {
    // get nft info
    fn nft_token(&self, token_id: TokenId) -> Option<JsonToken>;
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
}