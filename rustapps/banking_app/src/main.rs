mod models;
mod utils;

use crate::models::account::Account;
use crate::utils::display::display_balance;

fn main() {
    let mut account = Account::new("Alice", 1000.0);
    display_balance(&account);
    account.deposit(500.0);
    display_balance(&account);
    account.withdraw(200.0);
    display_balance(&account);
}
