use std::{thread, sync::{Mutex, Arc}};
use std::ops::Deref;
use std_semaphore::Semaphore;
use crate::fork::fork::Fork;
use crate::philosopher::philosopher::Philosopher;
// use crate::fork::fork::Fork;
// use crate::philosopher::philosopher::Philosopher;

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
        Philosopher::new("Aristotle".to_string()),
        Philosopher::new("Kant".to_string()),
        Philosopher::new("Spinoza".to_string()),
        Philosopher::new("Marx".to_string()),
        Philosopher::new("Russell".to_string()),
    ];
    let threads = philosophers.into_iter().enumerate().map(|(i, mut philosopher)| {
        println!("Philosopher {:?} is thinking", philosopher.name);
        let left_fork = available_forks[i].clone();
        let right_fork = available_forks[(i + 1) % available_forks.len()].clone();
        
        thread::spawn(move || {
            println!("Philosopher {:?} picking up left fork {}", philosopher.name, i);
            philosopher.grab_left_fork(left_fork).unwrap();
            println!("Philosopher {} picked up left fork {}", philosopher.name, i);
            println!("Philosopher {} picking up right fork {}", philosopher.name, (i + 1) % 5);
            philosopher.grab_right_fork(right_fork.try_lock().unwrap().deref().to_owned()).unwrap();
            println!("Philosopher {} picked up right fork {}", philosopher.name, (i + 1) % 5);
            println!("Philosopher {} is eating", philosopher.name);
            thread::sleep(std::time::Duration::from_secs(3));
            println!("Philosopher {} is done eating", philosopher.name);
            philosopher.drop_left_fork().expect("TODO: panic message");
            philosopher.drop_right_fork().expect("TODO: panic message");
        })
    }).collect::<Vec<_>>();
    threads.into_iter().flat_map(|t| t.join()).for_each(drop);
}

