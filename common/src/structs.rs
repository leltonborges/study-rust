use std::borrow::Borrow;
use std::default::Default;

use rand::{Rng, thread_rng};

use conta::{Account, create_account};

use crate::structs::conta::{ManagerAccout, Titular};

const LIST_ACCOUNTS: Vec<Account> = Vec::new();

mod conta {
    use std::fmt::{Display, Formatter};

    use rand::{Rng, thread_rng};
    use rand::rngs::ThreadRng;

    use crate::structs::LIST_ACCOUNTS;

    pub struct Titular {
        nome: String,
        documento: String,
    }

    pub struct Account {
        owner: Titular,
        balance: f64,
        id: u64,
        number: u32,
        agency: u32,
        digit: u8,
    }

    pub fn create_account(owner: Titular) {
        let account = Account {
            owner,
            ..Account::default()
        };
        LIST_ACCOUNTS.push(account)
    }


    impl Default for Account {
        fn default() -> Self {
            let mut rgn = thread_rng();
            Account {
                owner: { Titular::default() },
                balance: 0.0,
                id: rgn.gen_range(1..999),
                number: rgn.gen_range(10000..99999),
                agency: rgn.gen_range(1..999),
                digit: rgn.gen_range(0..99),
            }
        }
    }

    impl Default for Titular {
        fn default() -> Self {
            Titular { nome: String::from("Default"), documento: String::from("000000") }
        }
    }

    impl Display for Titular {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Titular: {{Nome: {}, Documento: {}}}", self.nome, self.documento)
        }
    }

    impl Display for Account {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "id: {}, owner: {{{}}},balance: {}, agency: {}, account: {}-{}", self.id, self.owner, self.balance, self.agency, self.number, self.digit)
        }
    }

    impl ManagerAccout for Account {
        fn withdraw(&mut self, value: f64) -> Result<String, String> {
            if value.le(&self.balance) {
                self.balance -= value;
                return Ok(format!("Saldo atualizado: {}", self.balance));
            }
            Err(String::from("Saldo insuficiente"))
        }

        fn deposit(&mut self, amount: f64) {
            self.balance += amount
        }
    }

    pub trait ManagerAccout {
        fn withdraw(&mut self, amount: f64) -> Result<String, String>;

        fn deposit(&mut self, amount: f64);
    }
}

pub fn example_accounts() {
    println!("====== Accounts ======\n");
    let mut default = Account::default();
    println!("Default: {}", default);

    match default.withdraw(50f64) {
        Ok(x) => println!("{}", x),
        Err(x) => println!("{}", x)
    }
    println!("withdraw: {}", default);

    default.deposit(1000.0);
    println!("deposit: {}", default);

    match default.withdraw(500f64) {
        Ok(x) => println!("{}", x),
        Err(x) => println!("{}", x)
    }
    println!("Withdraw: {}", default);
}
