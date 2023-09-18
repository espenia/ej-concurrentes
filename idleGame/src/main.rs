use std::{thread, sync::{Mutex, Arc}};

use user::user::User;

mod tech;
mod user;
mod game;


fn main() {
    let user = Arc::new(Mutex::new(User::new()));
    let mut user_ref = Arc::clone(&user);
    let mut store_ref = Arc::clone(&user);
    thread::spawn(move || {
        loop {
            game::idle(&mut user_ref)
        }
    });
    thread::spawn(move || {
        loop {
            game::store(&mut store_ref)
        }
    });

    thread::sleep(std::time::Duration::from_secs(10));
}

