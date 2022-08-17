use crate::*;
use near_sdk::CryptoHash;

// get hash function
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
};

// refund near function
pub(crate) fn refund_deposit(storage_used: u64) {
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    let attached_deposit = env::attached_deposit();

    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNear to cover storage",
        required_cost,
    );

    let refund = attached_deposit - required_cost;

    if refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    };
};

impl Contract {

    // add token to owner
    pub(crate) fn internal_add_token_to_owner(
        &mut self,
        account_id: &AccountId,
        token_id: &TokenId,
    ){
        let mut tokens_set = self.tokens_per_owner.get(account_id).unwrap_or_else(|| {
            UnorderedSet::new(
                StorageKey::TokensPerOwnerInner {
                    account_id_hash: hash_account_id(&account_id),
                }
                .try_to_vec()
                .unwrap(),
            )
            // insert token id
            tokens_set.insert(token_id);
            // insert tokens
            self.tokens_per_owner.insert(account_id, &tokens_set);
        });
    }

    // add token to kind map
    pub(crate) fn internal_add_token_to_kind_map(
        &mut self,
        token_id: &TokenId,
        token_kind: TokenKind,
    ){
        // get token kind
        let token_kind_clone = token_kind.clone();

        let mut tokens_set = self
            .tokens_per_kind
            .get(&token_kind_clone)
            .unwrap_or_else(|| {
                UnorderedSet::new(
                    StorageKey::TokensPerKindInner {
                        token_kind: token_kind,
                    }
                    .try_to_vec()
                    .unwrap(),
                )
            });
        // insert 
        tokens_set.insert(&token_id);
        self.tokens_per_kind.insert(&token_kind_clone, &tokens_set);
    }
};