use super::*;
#[near_bindgen]
impl Contract {
    pub fn assert_owner(&self, account_id: AccountId) {
        require!(
            self.tokens.owner_id == account_id,
            "Only owner can call this method"
        );
    }
    pub fn get_owner(&self) -> AccountId {
        return self.tokens.owner_id.clone();
    }
}
