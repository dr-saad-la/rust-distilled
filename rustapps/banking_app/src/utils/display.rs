use crate::models::account::Account;

/// Displays the balance of the given account.
///
/// # Arguments
///
/// * `account` - The account to display the balance for.
pub fn display_balance(account: &Account) {
    println!(
        "The balance for {} is: ${:.2}",
        account.owner(),
        account.balance()
    );
}
