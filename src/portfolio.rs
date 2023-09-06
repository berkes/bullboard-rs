// Purpose: Portfolio struct and methods

use account::Account;
use bullboard::aggregate::Aggregate;

struct Portfolio {
    accounts: Vec<Account>,
}

impl Portfolio {
    fn new() -> Self {
        Self { accounts: vec![] }
    }

    fn find_account_by_id(&self, id: &str) -> Option<Account> {
        self.accounts
            .iter()
            .find(|account| account.id == id)
            .cloned()
    }
}

impl Aggregate for Portfolio {
    type Event = PortfolioEvent;

    type Command = PortfolioCommand;

    type Error = PortfolioError;

    fn handle(&mut self, command: Self::Command) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            PortfolioCommand::CreateAccount { name, id } => {
                if self.find_account_by_id(&id).is_some() {
                    return Err(PortfolioError::AccountAlreadyExists);
                }

                let account = Account::new(id, name);
                let event = PortfolioEvent::AccountCreated(account);
                Ok(vec![event])
            }
            PortfolioCommand::DeleteAccount { id } => {
                let account = self
                    .find_account_by_id(&id)
                    .ok_or(PortfolioError::AccountDoesNotExist)?;

                let event = PortfolioEvent::AccountDeleted(account.clone());
                Ok(vec![event])
            }
            PortfolioCommand::UpdateAccount { id, name } => {
                let account: &mut Account = self
                    .accounts
                    .iter_mut()
                    .find(|account| account.id == id)
                    .ok_or(PortfolioError::AccountDoesNotExist)?;

                account.name = name;

                let event = PortfolioEvent::AccountUpdated(account.clone());
                Ok(vec![event])
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        match event {
            PortfolioEvent::AccountCreated(account) => self.accounts.push(account),
            PortfolioEvent::AccountDeleted(account) => self.accounts.retain(|a| a.id != account.id),
            PortfolioEvent::AccountUpdated(account) => {
                let account: &mut Account = self
                    .accounts
                    .iter_mut()
                    .find(|a| a.id == account.id)
                    .unwrap();

                account.name = account.name.clone();
            }
        }
    }
}

enum PortfolioCommand {
    CreateAccount { name: String, id: String },
    DeleteAccount { id: String },
    UpdateAccount { id: String, name: String },
}

#[derive(Debug, PartialEq, Clone)]
enum PortfolioEvent {
    AccountCreated(Account),
    AccountDeleted(Account),
    AccountUpdated(Account),
}

#[derive(Debug, PartialEq)]
enum PortfolioError {
    AccountAlreadyExists,
    AccountDoesNotExist,
}

mod account {
    #[derive(Debug, PartialEq, Clone)]
    pub struct Account {
        pub name: String,
        pub id: String,
        assets: Vec<Asset>,
    }

    impl Account {
        pub fn new(id: String, name: String) -> Self {
            Self {
                name,
                id,
                assets: vec![],
            }
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Asset {
        name: String,
        id: String,
        quantity: f64,
        price: f64,
        currency: String,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TestFramework;

    #[test]
    fn test_that_create_account_creates_account() {
        let mut portfolio = Portfolio::new();
        let mut cqrs = TestFramework::new(&mut portfolio);
        let account = Account::new("1337".to_string(), "test".to_string());

        cqrs.when(PortfolioCommand::CreateAccount {
            name: "test".to_string(),
            id: "1337".to_string(),
        })
        .then(vec![PortfolioEvent::AccountCreated(account)]);
    }

    #[test]
    fn test_that_create_account_doesnt_add_duplicates() {
        let mut portfolio = Portfolio::new();
        let mut cqrs = TestFramework::new(&mut portfolio);
        let account = Account::new("1337".to_string(), "test".to_string());
        cqrs.given(vec![PortfolioEvent::AccountCreated(account.clone())]);
        cqrs.when_err(PortfolioCommand::CreateAccount {
            name: "test".to_string(),
            id: "1337".to_string(),
        })
        .then_err(PortfolioError::AccountAlreadyExists);
    }

    #[test]
    fn test_that_update_account_updates_account() {
        let mut portfolio = Portfolio::new();
        let mut cqrs = TestFramework::new(&mut portfolio);

        let account = Account::new("1337".to_string(), "test".to_string());
        let account2 = Account::new("1337".to_string(), "test2".to_string());

        cqrs.given(vec![PortfolioEvent::AccountCreated(account.clone())]);
        cqrs.when(PortfolioCommand::UpdateAccount {
            name: "test2".to_string(),
            id: "1337".to_string(),
        })
        .then(vec![PortfolioEvent::AccountUpdated(account2)]);

        assert_eq!(portfolio.accounts[0].name, "test2");
    }

    #[test]
    fn test_that_update_account_fails_on_nonexisting_account() {
        let mut portfolio = Portfolio::new();
        let mut cqrs = TestFramework::new(&mut portfolio);

        cqrs.when_err(PortfolioCommand::UpdateAccount {
            name: "test2".to_string(),
            id: "1337".to_string(),
        })
        .then_err(PortfolioError::AccountDoesNotExist);
    }

    #[test]
    fn test_that_delete_account_removes_account() {
        let mut portfolio = Portfolio::new();
        let mut cqrs = TestFramework::new(&mut portfolio);

        let account = Account::new("1337".to_string(), "test".to_string());

        cqrs.given(vec![PortfolioEvent::AccountCreated(account.clone())]);
        cqrs.when(PortfolioCommand::DeleteAccount {
            id: "1337".to_string(),
        })
        .then(vec![PortfolioEvent::AccountDeleted(account)]);

        assert_eq!(portfolio.accounts.len(), 0);
    }
}
