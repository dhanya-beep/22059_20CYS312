struct BankAccount {
    account_number: u32,
    owner_name: String,
    balance: f64,
}

impl BankAccount {
    fn view_balance(&self) {
    println!(
        "Account {} (Owner: {}): Balance = ${}",
        self.account_number, self.owner_name, self.balance
    );
}

    
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited ${}. New balance: ${}", amount, self.balance);
    }
    
    fn withdraw(&mut self, amount: f64) {
        if self.balance >= amount {
            self.balance -= amount;
            println!("Withdrawn ${}. New balance: ${}", amount, self.balance);
        } else {
            println!("Insufficient funds!");
        }
    }
}

fn main() {
    let mut account = BankAccount {
        account_number: 1001,
        owner_name: "Alice".to_string(),
        balance: 500.0,
    };
    account.view_balance();
    account.deposit(200.0);
    account.withdraw(100.0);
}

