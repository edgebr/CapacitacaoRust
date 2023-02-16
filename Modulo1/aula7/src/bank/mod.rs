#[derive(Debug)]
pub struct BankAccount {
    pub owner: String,
    pub balance: f64,
}

#[allow(dead_code)] // remover esse comentário
impl BankAccount {
    pub fn owner(&self) -> &str {
        self.owner.as_str()
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }

    pub fn builder() -> BankAccountBuilder {
        BankAccountBuilder::new()
    }
}

#[derive(Default)]
pub struct BankAccountBuilder(String, f64);

impl BankAccountBuilder {
    fn new() -> BankAccountBuilder {
        BankAccountBuilder(String::default(), f64::default())
    }
    pub fn owner(mut self, name: &str) -> Self {
        self.0 = String::from(name);
        self
    }

    pub fn balance(mut self, balance: f64) -> Self {
        self.1 = balance;
        self
    }

    pub fn build(self) -> BankAccount {
        BankAccount {
            owner: self.0,
            balance: self.1,
        }
        //self.drop();
    }
}
