use super::*;

#[near_bindgen]
impl Contract {
    pub fn update_uri(&mut self, uri: String) {
        self.assert_owner(env::signer_account_id());
        let prev: Contract = env::state_read().expect("ERR_NOT_INITIALIZED");
        let mut metadata = prev.metadata.get().unwrap();
        metadata.base_uri = Some(uri);

        self.metadata = LazyOption::new(StorageKey::Metadata.try_to_vec().unwrap(), Some(&metadata))
    }

    pub fn update_nft_name(&mut self, name: String) {
        let mut metadata = self.metadata.get().unwrap();
        metadata.name = name;
        self.metadata = LazyOption::new(StorageKey::Metadata.try_to_vec().unwrap(), Some(&metadata))
    }

    pub fn transfer_ownership(&mut self, account_id: AccountId) {
        self.assert_owner(env::signer_account_id());
        self.tokens.owner_id = account_id;
    }
}
