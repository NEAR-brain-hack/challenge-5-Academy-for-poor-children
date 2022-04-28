use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider, TokenMetadata, NFT_METADATA_SPEC
};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::{
    env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise, PromiseOrValue, Balance
};
use near_sdk::json_types::{U128};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    tokens: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    total_supply: u64
}

const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz4KPCEtLSBHZW5lcmF0b3I6IEFkb2JlIElsbHVzdHJhdG9yIDI0LjMuMCwgU1ZHIEV4cG9ydCBQbHVnLUluIC4gU1ZHIFZlcnNpb246IDYuMDAgQnVpbGQgMCkgIC0tPgo8c3ZnIHZlcnNpb249IjEuMSIgaWQ9IkxheWVyXzEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6eGxpbms9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiIHg9IjBweCIgeT0iMHB4IgoJIHZpZXdCb3g9IjAgMCA1MTIgNTEyIiBzdHlsZT0iZW5hYmxlLWJhY2tncm91bmQ6bmV3IDAgMCA1MTIgNTEyOyIgeG1sOnNwYWNlPSJwcmVzZXJ2ZSI+CjxkZXNjPkNyZWF0ZWQgd2l0aCBza2V0Y2h0b29sLjwvZGVzYz4KPGcgaWQ9IkNvdXJzZS1BdXRob3ItX3gyRl8tTGludXgtQWNhZGVteS1JbnRlZ3JhdGlvbiI+Cgk8ZyBpZD0iQ291cnNlLUF1dGhvci1feDJGXy1MaW51eC1BY2FkZW15LUludGVncmF0aW9uLV94MkZfLU1hcC1PdXQtQ291cnNlIiB0cmFuc2Zvcm09InRyYW5zbGF0ZSgtNTg4LjAwMDAwMCwgLTIwMzcuMDAwMDAwKSI+CgkJPGcgaWQ9IkNvdXJzZS1TZWN0aW9uLTEtQ29weSIgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoMzIyLjAwMDAwMCwgODQ0LjAwMDAwMCkiPgoJCQk8ZyBpZD0iU2VjdGlvbi1Db3B5IiB0cmFuc2Zvcm09InRyYW5zbGF0ZSg1My4wMDAwMDAsIDYwOC4wMDAwMDApIj4KCQkJCTxnIGlkPSJBZGQtTGVhcm5pbmctQWN0aXZpdHkiIHRyYW5zZm9ybT0idHJhbnNsYXRlKDQ5LjAwMDAwMCwgNDc3LjAwMDAwMCkiPgoJCQkJCTxnIGlkPSJBZGRlZC1MZWFybmluZy1BY3Rpdml0eSI+CgkJCQkJCTxnIGlkPSJNb2R1bGUtQnRuIiB0cmFuc2Zvcm09InRyYW5zbGF0ZSgyNC4wMDAwMDAsIDk4LjAwMDAwMCkiPgoJCQkJCQkJPGcgaWQ9Ikljb25feDJGX1doaXRlX3gyRl9MaW51eC1BY2FkZW15IiB0cmFuc2Zvcm09InRyYW5zbGF0ZSgxMzkuMDAwMDAwLCA4LjAwMDAwMCkiPgoJCQkJCQkJCTxwYXRoIGlkPSJNYXNrIiBkPSJNNDYzLjMsMTQ4LjV2NjcuOGgtMjAuN3YtNTguOGwtMzMuNCwxNC42YzIxLjEsMzAuMSwzMy40LDY2LjcsMzMuNCwxMDYuMkw0NDIuNSw0MTNsLTU5LjMsMQoJCQkJCQkJCQljLTMzLjUsMzEuMy03OC44LDUwLjQtMTI4LjMsNTAuNHMtOTMuNy0xOS4xLTEyNy4xLTUwLjRsLTU3LjMtMVYyNzguNHYwYzAtNDAuMSwxMi41LTc3LjIsMzMuOC0xMDcuNmwtNzUuMi0zMS40TDI2Miw1MS4zCgkJCQkJCQkJCXYtMC40bDAsMC4ybDAtMC4ydjAuNGwyMjIsODguMUw0NjMuMywxNDguNXogTTEyMS44LDE3OC4xYy0xOS4zLDI1LjQtMzAuNiw1Ni40LTMwLjYsODkuOWMwLDg1LjYsNzQuMSwxNTUuMSwxNjUuNCwxNTUuMQoJCQkJCQkJCQlTNDIyLDM1My43LDQyMiwyNjhjMC0zMi45LTEwLjktNjMuNC0yOS42LTg4LjVMMjYyLDIzNi42djAuNGwwLTAuMmwwLDAuMnYtMC40TDEyMS44LDE3OC4xeiBNMjY3LjUsMjc4LjRsMzAuNSwyMC40CgkJCQkJCQkJCWwtMzAuNSw2Mi4zbC0zMS42LTYyLjNMMjY3LjUsMjc4LjR6IE0xODQuMiwyNTcuN2M1LjcsMCwxMC4zLDQuNiwxMC4zLDEwLjNzLTQuNiwxMC4zLTEwLjMsMTAuM2MtNS43LDAtMTAuMy00LjYtMTAuMy0xMC4zCgkJCQkJCQkJCVMxNzguNSwyNTcuNywxODQuMiwyNTcuN3ogTTMyOC45LDI1Ny43YzUuNywwLDEwLjMsNC42LDEwLjMsMTAuM3MtNC42LDEwLjMtMTAuMywxMC4zcy0xMC4zLTQuNi0xMC4zLTEwLjMKCQkJCQkJCQkJUzMyMy4yLDI1Ny43LDMyOC45LDI1Ny43eiIvPgoJCQkJCQkJPC9nPgoJCQkJCQk8L2c+CgkJCQkJPC9nPgoJCQkJPC9nPgoJCQk8L2c+CgkJPC9nPgoJPC9nPgo8L2c+Cjwvc3ZnPgo=";
const ONE_NEAR_ES_YOCTO: Balance = 1_000_000_000_000_000_000_000_000;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Example NEAR non-fungible token".to_string(),
                symbol: "EXAMPLE".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        metadata.assert_valid();
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
            total_supply: 0
        }
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let this: Self = env::state_read().expect("Cannot deserialize");

        assert_eq!(
            env::predecessor_account_id(),
            this.tokens.owner_id,
            "Only owner"
        );

        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                this.tokens.owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval),
            ),
            metadata: this.metadata,
            total_supply: this.total_supply
        }
    }

    // One person could create only one nft which token's id is user's account_id
    #[payable]
    pub fn nft_mint(
        &mut self,
        receiver_id: AccountId,
    ) -> Token {
        let deposit_value = env::attached_deposit();
        assert!(deposit_value >= 2 * ONE_NEAR_ES_YOCTO);
        let metadata = TokenMetadata {
            media: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
            title: Some("NEAR HACKS ACADEMY DONATION".to_string()),
            description: Some("Thanks for your donation".to_string()),
            media_hash: None,
            copies: None,
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        };
        self.total_supply += 1;
        self.tokens.internal_mint(self.total_supply.to_string(), receiver_id, Some(metadata))
    }

    pub fn withdraw(&mut self, receiver: AccountId, amount: U128) -> Promise {
        assert_eq!(
            receiver,
            self.tokens.owner_id
        );

        Promise::new(receiver).transfer(amount.into())
    }
}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}
