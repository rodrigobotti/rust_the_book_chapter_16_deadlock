use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct BankAccount {
    balance: i32,
}

impl BankAccount {
    pub fn increment(&mut self, amount: i32) {
        self.balance += amount;
    }
    pub fn decrement(&mut self, amount: i32) {
        self.balance -= amount;
    }

    pub fn new(balance: i32) -> BankAccount {
        BankAccount { balance }
    }

    pub fn transfer(from: &mut BankAccount, to: &mut BankAccount, amount: i32) {
        from.decrement(amount);
        to.increment(amount);
    }
}

fn main() {
    may_deadlock();
}

pub fn may_deadlock() {
    println!("-- May deadlock --");

    let account_a = Arc::new(Mutex::new(BankAccount::new(10)));
    let account_b = Arc::new(Mutex::new(BankAccount::new(5)));

    let to_1 = Arc::clone(&account_a);
    let from_1 = Arc::clone(&account_b);
    let handle_1 = thread::spawn(move || {
        println!("Transfering from B to A");

        thread::sleep(Duration::from_millis(500));

        let amount = 5;
        let mut from = from_1.lock().unwrap();
        let mut to = to_1.lock().unwrap();
        BankAccount::transfer(&mut from, &mut to, amount);
    });

    let to_2 = Arc::clone(&account_b);
    let from_2 = Arc::clone(&account_a);
    let handle_2 = thread::spawn(move || {
        println!("Transfering from A to B");

        thread::sleep(Duration::from_millis(500));

        let amount = 2;
        let mut from = from_2.lock().unwrap();
        let mut to = to_2.lock().unwrap();
        BankAccount::transfer(&mut from, &mut to, amount);
    });

    handle_1.join().unwrap();
    handle_2.join().unwrap();

    println!("-- May not be called --");

    println!("Balance A = {}", account_a.lock().unwrap().balance);
    println!("Balance B = {}", account_b.lock().unwrap().balance);
}
