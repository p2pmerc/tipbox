//! tipbox: fans send NEAR to a creator's tip balance, and creators withdraw it.
//!
//! Each account has a balance inside the contract. `tip` credits a creator, `withdraw` pays the
//! caller out of their own balance.

use near_sdk::json_types::U128;
use near_sdk::store::LookupMap;
use near_sdk::{env, near, require, AccountId, NearToken, PanicOnDefault, Promise};

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct TipBox {
    balances: LookupMap<AccountId, u128>,
    total_tips: u128,
}

#[near]
impl TipBox {
    #[init]
    pub fn new() -> Self {
        Self { balances: LookupMap::new(b"b"), total_tips: 0 }
    }

    /// Send the attached NEAR to `creator`'s tip balance.
    #[payable]
    pub fn tip(&mut self, creator: AccountId) {
        let amount = env::attached_deposit().as_yoctonear();
        require!(amount > 0, "attach some NEAR to tip");
        require!(creator != env::predecessor_account_id(), "you cannot tip yourself");
        let current = self.balances.get(&creator).copied().unwrap_or(0);
        self.balances.insert(creator, current + amount);
        self.total_tips += amount;
    }

    /// How much `account` can currently withdraw, in yoctoNEAR.
    pub fn balance_of(&self, account: AccountId) -> U128 {
        U128(self.balances.get(&account).copied().unwrap_or(0))
    }

    pub fn total_tips(&self) -> U128 {
        U128(self.total_tips)
    }

    /// Pay `amount` yoctoNEAR out to the caller and reduce their balance by the same amount.
    pub fn withdraw(&mut self, amount: U128) -> Promise {
        let caller = env::predecessor_account_id();
        let owed = self.balances.get(&caller).copied().unwrap_or(0);
        self.balances.insert(caller.clone(), owed.saturating_sub(amount.0));
        Promise::new(caller).transfer(NearToken::from_yoctonear(amount.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    fn context(predecessor: AccountId, deposit: u128) {
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(predecessor)
            .attached_deposit(NearToken::from_yoctonear(deposit))
            .build());
    }

    #[test]
    fn tips_accumulate_for_the_creator() {
        let mut c = TipBox::new();
        context(accounts(1), 1_000);
        c.tip(accounts(0));
        context(accounts(2), 500);
        c.tip(accounts(0));
        assert_eq!(c.balance_of(accounts(0)).0, 1_500);
        assert_eq!(c.total_tips().0, 1_500);
    }

    #[test]
    #[should_panic(expected = "cannot tip yourself")]
    fn cannot_tip_yourself() {
        let mut c = TipBox::new();
        context(accounts(0), 1_000);
        c.tip(accounts(0));
    }

    #[test]
    fn creator_withdraws_their_tips() {
        let mut c = TipBox::new();
        context(accounts(1), 1_000);
        c.tip(accounts(0));
        context(accounts(0), 0);
        let _ = c.withdraw(U128(400));
        assert_eq!(c.balance_of(accounts(0)).0, 600);
    }
}
