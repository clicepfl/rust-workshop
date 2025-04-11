#![allow(dead_code, clippy::new_without_default)]

const MAX_NAME_LENGTH: usize = 20;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NameEmpty,
    NameTooLong,
    AccountNotFound,
    NotEnoughFunds,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Transfer {
    from: String,
    to: String,
    amount: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Account {
    // TODO
}

impl Account {
    /// Create a new account, checking that the name as the correct size. If not, it returns the corresponding error.
    pub fn new(name: String, initial_funds: u64) -> Result<Self, Error> {
        todo!()
    }

    /// Remove the given amount from the account, or returns an error if there is not enough funds.
    pub fn take(&mut self, amount: u64) -> Result<(), Error> {
        todo!()
    }

    /// Add the given amount to the account.
    pub fn put(&mut self, amount: u64) {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Bank {
    // TODO
}

impl Bank {
    pub fn new() -> Self {
        todo!()
    }

    /// Add a new account to the bank.
    pub fn add_account(&mut self, name: String, initial_funds: u64) -> Result<(), Error> {
        todo!()
    }

    /// Find the account with the given name, if any. Returns a mutable reference.
    pub fn find_account(&mut self, name: &str) -> Result<&mut Account, Error> {
        todo!()
    }

    /// Process all the transfer in the list. Each of them needs to be atomic, i.e. either the funds are fully moved or nothing is done.
    /// They need to be processed in the order given. Once a failure occurs, the function should stop and return the error.
    pub fn process_transfers(&mut self, transactions: Vec<Transfer>) -> Result<(), Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn account_name_empty() {
        assert_eq!(Account::new("".to_owned(), 10), Err(Error::NameEmpty))
    }

    #[test]
    fn account_name_too_long() {
        assert_eq!(
            Account::new("123456789123456789123456789".to_owned(), 10),
            Err(Error::NameTooLong)
        )
    }

    #[test]
    fn account_take_enough() {
        assert_eq!(Account::new("acc".to_owned(), 10).unwrap().take(5), Ok(()))
    }

    #[test]
    fn account_take_not_enough() {
        assert_eq!(
            Account::new("acc".to_owned(), 10).unwrap().take(15),
            Err(Error::NotEnoughFunds)
        )
    }

    #[test]
    fn bank_account_not_found() {
        let mut bank = Bank::new();

        assert_eq!(bank.find_account("acc"), Err(Error::AccountNotFound))
    }

    #[test]
    fn bank_add_find_account() {
        let mut bank = Bank::new();
        assert!(bank.add_account("acc".to_owned(), 10).is_ok());

        assert!(bank.find_account("acc").is_ok());
    }

    #[test]
    fn bank_transfer_no_failures() {
        let mut bank = Bank::new();
        assert!(bank.add_account("a".to_owned(), 10).is_ok());
        assert!(bank.add_account("b".to_owned(), 10).is_ok());

        assert!(
            bank.process_transfers(vec![Transfer {
                from: "a".to_owned(),
                to: "b".to_owned(),
                amount: 5,
            }])
            .is_ok()
        );

        assert!(bank.find_account("a").is_ok_and(|a| a.funds == 5));
        assert!(bank.find_account("b").is_ok_and(|b| b.funds == 15));
    }

    #[test]
    fn bank_transfer_failures1() {
        let mut bank = Bank::new();
        assert!(bank.add_account("a".to_owned(), 10).is_ok());
        assert!(bank.add_account("b".to_owned(), 10).is_ok());

        assert_eq!(
            bank.process_transfers(vec![
                Transfer {
                    from: "a".to_owned(),
                    to: "b".to_owned(),
                    amount: 5,
                },
                Transfer {
                    from: "d".to_owned(),
                    to: "e".to_owned(),
                    amount: 5,
                },
                Transfer {
                    from: "a".to_owned(),
                    to: "b".to_owned(),
                    amount: 5,
                },
            ]),
            Err(Error::AccountNotFound)
        );

        assert!(bank.find_account("a").is_ok_and(|a| a.funds == 5));
        assert!(bank.find_account("b").is_ok_and(|b| b.funds == 15));
    }

    #[test]
    fn bank_transfer_failures2() {
        let mut bank = Bank::new();
        assert!(bank.add_account("a".to_owned(), 10).is_ok());
        assert!(bank.add_account("b".to_owned(), 10).is_ok());

        assert_eq!(
            bank.process_transfers(vec![
                Transfer {
                    from: "a".to_owned(),
                    to: "b".to_owned(),
                    amount: 5,
                },
                Transfer {
                    from: "a".to_owned(),
                    to: "b".to_owned(),
                    amount: 5,
                },
                Transfer {
                    from: "a".to_owned(),
                    to: "b".to_owned(),
                    amount: 5,
                },
            ]),
            Err(Error::NotEnoughFunds)
        );

        assert!(bank.find_account("a").is_ok_and(|a| a.funds == 0));
        assert!(bank.find_account("b").is_ok_and(|b| b.funds == 20));
    }

    #[test]
    fn bank_transfer_failures_atomic() {
        let mut bank = Bank::new();
        assert!(bank.add_account("a".to_owned(), 10).is_ok());
        assert!(bank.add_account("b".to_owned(), 10).is_ok());

        assert_eq!(
            bank.process_transfers(vec![
                Transfer {
                    from: "a".to_owned(),
                    to: "b".to_owned(),
                    amount: 5,
                },
                Transfer {
                    from: "a".to_owned(),
                    to: "e".to_owned(),
                    amount: 5,
                },
            ]),
            Err(Error::AccountNotFound)
        );

        assert!(bank.find_account("a").is_ok_and(|a| a.funds == 5));
        assert!(bank.find_account("b").is_ok_and(|b| b.funds == 15));
    }
}

fn main() {
    todo!()
}
