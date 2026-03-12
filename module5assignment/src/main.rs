mod bank_account;

use bank_account::BankAccount;

fn main(){
    let mut account = BankAccount::new(100.0);
    account.deposit(45.0);
    account.withdraw(50.0);
    println!("Balance: {}", account.balance());
}