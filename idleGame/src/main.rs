use std::{thread, sync::{Mutex, Arc}};
use std::time::Duration;
use std_semaphore::Semaphore;
use user::user::User;

mod tech;
mod user;
mod game;


fn main() {
    // let user_status = Arc::new(Semaphore::new(0));
    // let user_idling = user_status.clone();
    // let user_buying = user_status.clone();
    let user = Arc::new(Mutex::new(User::new()));
    let mut user_idle_ref = Arc::clone(&user);
    let mut user_store_ref = Arc::clone(&user);
    let t1 = thread::spawn(move || {
        loop {
            game::idle(&mut user_idle_ref);
            thread::sleep(Duration::from_millis(100));
        }
    });
    let t2 = thread::spawn(move || {
        loop {
            game::store(&mut user_store_ref);
            thread::sleep(Duration::from_secs(3));
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();
}

