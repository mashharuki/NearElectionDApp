/**
 * ============================================================
 * SmartContract
 * ============================================================
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise};
// import other rs
mod vote;
mod enumeration;
mod internal;
mod metadata;
mod mint;
mod nft_core;

pub use crate::enumeration::*;
use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use vote::*;

// struct data of election Contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
    pub tokens_per_kind: LookupMap<TokenKind, UnorderedSet<TokenId>>,
    pub tokens_by_id: LookupMap<TokenId, TokenOwner>,
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,
    pub metadata: LazyOption<NFTContractMetadata>,
    pub token_id_counter: u128,
    pub likes_per_candidate: LookupMap<TokenId, Likes>,
    pub added_voter_list: LookupMap<ReceiverId, TokenId>,
    pub voted_voter_list: LookupMap<ReceiverId, u128>,
    pub is_election_closed: bool,
}

// enum of StorageKey
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokensPerKind,
    TokensPerOwnerInner { account_id_hash: CryptoHash },
    TokensPerKindInner { token_kind: TokenKind },
    TokensById,
    TokenMetadataById,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    NFTContractMetadata,
    LikesPerCandidate,
    AddedVoterList,
    VotedVoterList,
}

#[near_bindgen]
impl Contract {

    // function for initialization
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        let this = Self {
            owner_id,
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_per_kind: LookupMap::new(StorageKey::TokensPerKind.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(StorageKey::TokenMetadataById.try_to_vec().unwrap(),),
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            token_id_counter: 0,
            likes_per_candidate: LookupMap::new(
                StorageKey::LikesPerCandidate.try_to_vec().unwrap(),
            ),
            added_voter_list: LookupMap::new(StorageKey::AddedVoterList.try_to_vec().unwrap()),
            voted_voter_list: LookupMap::new(StorageKey::VotedVoterList.try_to_vec().unwrap()),
            is_election_closed: false,
        };
        // return
        this
    }

    // initialization function
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "Near Vote Contact".to_string(),
                reference: "This contract is design for fair election!".to_string(),
            },
        )
    }
}

/**
 * ============================================================
 * test code
 * ============================================================
 */
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use std::collections::HashMap;
    use super::*;

    const MINT_STORAGE_COST: u128 = 100000000000000000000000;

    /**
     * create virtual Blockchain for test
     */
    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        // return
        builder
    }

    /**
     * nft mint test
     */
    #[test]
    fn mint_test() {
        // create virtual env
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        // create Contract
        let mut contract = Contract::new_default_meta(accounts(1).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(1))
            .build()
        );

        // check ID
        assert_eq!(contract.owner_id, accounts(1));
        // mint 
        contract.nft_mint(
            TokenMetadata {
                title: None,
                description: None,
                media: "https...".to_string(),
                media_CID: "Qeo....".to_string(),
                candidate_name: None,
                candidate_manifest: None,
                token_kind: "candidate".to_string(),
                token_id: None,
            },
            accounts(1),
        );

        // check 
        assert_eq!(u128::from(contract.nft_total_supply()), 1);
        // get nft info
        let nft_info = contract.nft_tokens(None, None);
        // check nft infos
        assert_eq!(nft_info[0].metadata.media, "https...".to_string());
        assert_eq!(u128::from(contract.nft_supply_for_owner(accounts(1))), 1);
        assert_eq!(
            nft_info[0].owner_id,
            contract.nft_tokens_for_owner(accounts(1), None, None)[0].owner_id
        );
        assert_eq!(
            nft_info[0].owner_id,
            contract.nft_tokens_for_kind("candidate".to_string(), None, None)[0].owner_id
        );
    }

    /**
     * vote test
     */
    #[test]
    fn vote_closed_test() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        // deploy contract
        let mut contract = Contract::new_default_meta(accounts(1).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(1))
            .build()
        );

        // check
        assert_eq!(contract.is_election_closed, false);
        // close election
        contract.close_election();
        // check
        assert_eq!(contract.is_election_closed, true);
        // reopen election
        contract.reopen_election();
        assert_eq!(contract.is_election_closed, false);
    }

    /**
     * transfer test
     */
    #[test]
    fn transfer_test() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        // deploy contract
        let mut contract = Contract::new_default_meta(accounts(1).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_STORAGE_COST)
            .predecessor_account_id(accounts(1))
            .build()
        );

        // mint
        contract.nft_mint(
            TokenMetadata {
                title: None,
                description: None,
                media: "https....".to_string(),
                media_CID: "Qeo....".to_string(),
                candidate_name: None,
                candidate_manifest: None,
                token_kind: "candidate".to_string(),
                token_id: None,
            },
            accounts(1),
        );

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(1)
            .predecessor_account_id(accounts(1))
            .build()
        );

        // transfer nft
        contract.nft_transfer(accounts(2), 0);
        // get nft info
        let nft_info = contract.nft_tokens(None, None);
        // check
        assert_eq!(nft_info[0].owner_id, accounts(2));
    }
}