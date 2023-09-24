use std::{thread, sync::{Mutex, Arc}};
use crate::fork::fork::Fork;
use crate::philosopher::philosopher::Philosopher;

mod fork;
mod philosopher;


fn main() {
    let mut available_forks = vec![
        Arc::new(Mutex::new(Fork::new())),
        Arc::new(Mutex::new(Fork::new())),
        Arc::new(Mutex::new(Fork::new())),
        Arc::new(Mutex::new(Fork::new())),
        Arc::new(Mutex::new(Fork::new()))
    ];
    let mut philosophers = vec![
        Arc::new(Mutex::new(Philosopher::new())),
        Arc::new(Mutex::new(Philosopher::new())),
        Arc::new(Mutex::new(Philosopher::new())),
        Arc::new(Mutex::new(Philosopher::new())),
        Arc::new(Mutex::new(Philosopher::new()))
    ];
    // esto lo tiro co-pilot ni idea jaja
    let threads = philosophers.iter_mut().enumerate().map(|(i, philosopher)| {
        let left_fork = available_forks[i].clone();
        let right_fork = available_forks[(i + 1) % available_forks.len()].clone();
        let philosopher = philosopher.clone();
        thread::spawn(move || {
            let mut philosopher = philosopher.lock().unwrap();
            philosopher.left_fork = Some(left_fork);
            philosopher.right_fork = Some(right_fork);
            philosopher.eat();
        })
    }).collect::<Vec<_>>();
}

