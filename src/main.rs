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

    fn transfer(from: &mut BankAccount, to: &mut BankAccount, amount: i32) {
        from.decrement(amount);
        to.increment(amount);
    }

    pub fn transfer_transaction(
        from: Arc<Mutex<BankAccount>>,
        to: Arc<Mutex<BankAccount>>,
        amount: i32,
    ) {
        // simulating work
        thread::sleep(Duration::from_millis(500));

        // lock `from` then `to` then do the transfer
        let mut account_from = from.lock().unwrap();
        let mut account_to = to.lock().unwrap();
        BankAccount::transfer(&mut account_from, &mut account_to, amount);
        // locks release after guards are droped when block scope ends
    }
}

fn main() {
    may_deadlock();
}

pub fn may_deadlock() {
    println!("------------------");
    println!("-- May deadlock --");

    // Here we have two bank accounts: account_a and account_b.
    // We will attempt a deadlock by executing two concurrent transfers.
    // During a transfer to avoid races in the account's balances we lock the accounts
    // We always lock the source `from` account first then the target `to` account last <- this is what can create a deadlock

    let account_a = Arc::new(Mutex::new(BankAccount::new(10)));
    let account_b = Arc::new(Mutex::new(BankAccount::new(5)));

    // Thread 1 will transfer $5 from account_b (account_from_thread_1) to account_a (account_to_thread_1)
    // locking account_b then account_a

    let account_from_thread_1 = Arc::clone(&account_b);
    let account_to_thread_1 = Arc::clone(&account_a);
    let handle_1 = thread::spawn(move || {
        println!("Transfering from B to A");
        BankAccount::transfer_transaction(account_from_thread_1, account_to_thread_1, 5);
    });

    // Thread 2 will transfer $2 from account_a (account_from_thread_2) to account_b (account_to_thread_2)
    // locking account_a then account_b
    // PS: notice that the LOCK ORDER IS DIFFERENT than that of thread 1

    let account_from_thread_2 = Arc::clone(&account_a);
    let account_to_thread_2 = Arc::clone(&account_b);
    let handle_2 = thread::spawn(move || {
        println!("Transfering from A to B");
        BankAccount::transfer_transaction(account_from_thread_2, account_to_thread_2, 2);
    });

    // wait thread 1 and thread 2 to complete
    // which may never happen due to a deadlock

    handle_1.join().unwrap();
    handle_2.join().unwrap();

    println!("-- May not be called --");
    println!("Balance A = {}", account_a.lock().unwrap().balance);
    println!("Balance B = {}", account_b.lock().unwrap().balance);

    // The deadlock can happen because the following can happen:
    // - thread 1 locks it's `from` account -- account_b -- to begin the transaction
    // - thread 2 locks it's `from` account -- account_a -- to begin the transaction
    // - thread 1 attempts to lock it's `to` account -- account a -- but thread 2 already has a lock on it so it get's blocked awaiting it's release
    // - thread 2 attempts to lock it's `to` account -- account b -- but thread 1 already has a lock on it so it get's blocked awaiting it's release
    // - none of the threads can progress any further because they are stuck waiting on each other to release their first lock: deadlock

    // But how could we have avoided the deadlock from happening ?
    // The answer in this case is: locking resources in a deterministic global order.
    // Instead of locking the parameters in the order they are received,
    // we order them using a deterministic global ordering strategy --
    // e.g. the accounts could have unique numeric IDs and we would always order them in ascending ID order.
    //
    // General rules to follow to avoid deadlocks:
    // - lock the resources using a deterministic global ordering strategy
    // - avoid calling alien methods/functions when locked: those alien functions can lock other resources
    // - spend as little time as possible holding locks
}
