use std::{thread, sync::{Mutex, Arc}};
use std_semaphore::Semaphore;
// use crate::fork::fork::Fork;
// use crate::philosopher::philosopher::Philosopher;

mod fork;
mod philosopher;


fn main() {
    let mut available_forks = vec![
        Arc::new(Semaphore::new(1)),
        Arc::new(Semaphore::new(1)),
        Arc::new(Semaphore::new(1)),
        Arc::new(Semaphore::new(1)),
        Arc::new(Semaphore::new(1))
    ];
    let mut philosophers = vec![
        1,2,3,4,5
    ];
    let threads = philosophers.into_iter().enumerate().map(|(i, philosopher)| {
        println!("Philosopher {} is thinking", philosopher);
        let left_fork = available_forks[i].clone();
        let right_fork = available_forks[(i + 1) % available_forks.len()].clone();
        thread::spawn(move || {
            println!("Philosopher {} picking up left fork {}", philosopher, i);
            left_fork.acquire();
            println!("Philosopher {} picked up left fork {}", philosopher, i);
            println!("Philosopher {} picking up right fork {}", philosopher, (i + 1) % 5);
            right_fork.acquire();
            println!("Philosopher {} picked up right fork {}", philosopher, (i + 1) % 5);
            println!("Philosopher {} is eating", philosopher);
            thread::sleep(std::time::Duration::from_secs(3));
            println!("Philosopher {} is done eating", philosopher);
            left_fork.release();
            right_fork.release();
        })
    }).collect::<Vec<_>>();
    threads.into_iter().flat_map(|t| t.join()).for_each(drop);
}

