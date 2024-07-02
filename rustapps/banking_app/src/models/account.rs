/// Represents a bank account.
pub struct Account {
    owner: String,
    balance: f64,
}

impl Account {
    /// Creates a new account.
    ///
    /// # Arguments
    ///
    /// * `owner` - The name of the account owner.
    /// * `initial_balance` - The initial balance of the account.
    pub fn new(owner: &str, initial_balance: f64) -> Self {
        Account {
            owner: owner.to_string(),
            balance: initial_balance,
        }
    }

    /// Deposits an amount into the account.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to deposit.
    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    /// Withdraws an amount from the account.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to withdraw.
    ///
    /// # Returns
    ///
    /// Returns `true` if the withdrawal was successful, otherwise `false`.
    pub fn withdraw(&mut self, amount: f64) -> bool {
        if self.balance >= amount {
            self.balance -= amount;
            true
        } else {
            false
        }
    }

    /// Gets the current balance of the account.
    ///
    /// # Returns
    ///
    /// The current balance of the account.
    pub fn balance(&self) -> f64 {
        self.balance
    }

    /// Gets the owner of the account.
    ///
    /// # Returns
    ///
    /// The name of the account owner.
    pub fn owner(&self) -> &str {
        &self.owner
    }
}
