#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod lock_unlock_smart_contract {
    use ink::env::{ pay::Transfer, DefaultEnvironment };

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

    impl LockUnlockSmartContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                locker: None,
                locked_amount: 0,
            }
        }

        #[ink(message, payable)]
        pub fn lock(&mut self) {
            let caller = self.env().caller();
            let transferred = self.env().transferred_value();

            assert!(self.locker.is_none(), "Assets already locked");
            assert!(transferred > 0, "Must send assets to lock");

            self.locker = Some(caller);
            self.locked_amount = transferred;

            self.env().emit_event(Locked {
                locker: caller,
                amount: transferred,
            });
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
            self.env().transfer(caller, amount).expect("Transfer failed");

            // Reset contract state
            self.locker = None;
            self.locked_amount = 0;

            self.env().emit_event(Redeemed {
                locker: caller,
                amount,
            });
        }
    }
}
