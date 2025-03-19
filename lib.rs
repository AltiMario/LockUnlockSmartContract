#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod lock_unlock_smart_contract {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct LockUnlockSmartContract {
        locker: Option<AccountId>,
        locked_amount: Balance,
    }

    #[ink(event)]
    pub struct Locked {
        #[ink(topic)]
        locker: AccountId,
        amount: Balance,
    }

    #[ink(event)]
    pub struct Redeemed {
        #[ink(topic)]
        locker: AccountId,
        amount: Balance,
    }

    impl Default for LockUnlockSmartContract {
        fn default() -> Self {
            Self {
                locker: None,
                locked_amount: 0,
            }
        }
    }

    impl LockUnlockSmartContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message, payable)]
        pub fn lock(&mut self) {
            let caller = self.env().caller();
            let transferred = self.env().transferred_value();

            assert!(self.locker.is_none(), "Assets already locked");
            assert!(transferred > 0, "Must send assets to lock");

            self.locker = Some(caller);
            self.locked_amount = transferred;

            self.emit_locked_event(caller, transferred);
        }

        #[ink(message)]
        pub fn redeem(&mut self, message: String) {
            let caller = self.env().caller();

            // Check caller is the locker
            assert_eq!(self.locker, Some(caller), "Caller is not the locker");

            // Verify the message matches
            assert_eq!(message, String::from("Hello, World!"), "Incorrect message provided");

            // Transfer locked assets back
            let amount = self.locked_amount;
            self.env().transfer(caller, amount).unwrap_or_else(|error| {
                panic!("Transfer failed: {:?}", error);
            });

            // Reset contract state
            self.locker = None;
            self.locked_amount = 0;

            self.emit_redeemed_event(caller, amount);
        }

        fn emit_locked_event(&self, locker: AccountId, amount: Balance) {
            self.env().emit_event(Locked {
                locker,
                amount,
            });
        }

        fn emit_redeemed_event(&self, locker: AccountId, amount: Balance) {
            self.env().emit_event(Redeemed {
                locker,
                amount,
            });
        }
    }
}
