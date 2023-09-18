use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::io::{stdin, stdout, Write};

use crate::tech::tech::{BaseTech, Farm, Mine, Techs, Tree};
use crate::user::user::User;

pub fn idle(user_ref: &mut Arc<Mutex<User>>) {
    thread::sleep(Duration::from_secs(1));
    let mut user = user_ref.lock().unwrap();
    let techs = user.get_tech();
    let mut gained = techs.0.len() as u64 * BaseTech::new().produce as u64;
    gained += techs.1.len() as u64 * Tree::new().produce as u64;
    gained += techs.2.len() as u64 * Farm::new().produce as u64;
    gained += techs.3.len() as u64 * Mine::new().produce as u64;
    user.add_gold(gained);
    println!("Gained {} gold", gained);
    println!("Gold: {} | ", user.get_gold());
}

pub fn store(user_ref: &mut Arc<Mutex<User>>) {
    let mut user = user_ref.lock().unwrap();

    let mut s=String::new();
    print!("Do you want to buy: \
        Current Gold: {} \
        \n1. Base Tech \
        \n2. Tree Tech \
        \n3. Farm Tech \
        \n4. Mine Tech \
        \n5. Exit \
        \n> ", user.get_gold());
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    let s = s.trim();
    match s {
        "1" => {
            if user.get_gold() >= BaseTech::new().cost as u64 {
                user.add_base_tech(BaseTech::new());
                user.remove_gold(BaseTech::new().cost as u64);
            } else {
                println!("Not enough gold");
            }
        },
        "2" => {
            if user.get_gold() >= Tree::new().cost as u64 {
                user.add_tree_tech(Tree::new());
                user.remove_gold(Tree::new().cost as u64);
            } else {
                println!("Not enough gold");
            }
        },
        "3" => {
            if user.get_gold() >= Farm::new().cost as u64 {
                user.add_farm_tech(Farm::new());
                user.remove_gold(Farm::new().cost as u64);
            } else {
                println!("Not enough gold");
            }
        },
        "4" => {
            if user.get_gold() >= Mine::new().cost as u64 {
                user.add_mine_tech(Mine::new());
                user.remove_gold(Mine::new().cost as u64);
            } else {
                println!("Not enough gold");
            }
        },
        &_ => {} }
    thread::sleep(Duration::from_secs(3));

}