use std::sync::{Arc, Mutex};
use crate::fork::fork::{Fork};

#[derive(Debug)]
pub struct Philosopher {
    pub(crate) left_fork: Option<Fork>,
    pub(crate) right_fork: Option<Fork>,
    eating: bool,
    thinking: bool,
    pub(crate) name: String,
}

const ALREADY_HAVE_LEFT_FORK: &str = "already holding a fork in the left hand";
const ALREADY_HAVE_RIGHT_FORK: &str = "already holding a fork in the right hand";
const DONT_HAVE_BOTH_FORKS: &str = "don't have both forks";

impl Philosopher {
    pub fn new(name: String) -> Philosopher {
        Philosopher {
            left_fork: None,
            right_fork: None,
            eating: false,
            thinking: true,
            name
        }
    }

    pub fn grab_left_fork(&mut self, mut fork: Arc<Mutex<Fork>>) -> Result<bool, String> {
        if self.left_fork.is_none() {
            return match fork.hold() {
                Ok(..) => {
                    self.left_fork = Option::from(fork.try_lock().unwrap().clone());
                    Ok(true)
                },
                Err(e) => {
                    Err(e)
                }
            }
        } else {
            Err(ALREADY_HAVE_LEFT_FORK.parse().unwrap())
        }
    }

    pub fn grab_right_fork(&mut self, mut fork: Fork) -> Result<bool, String> {
        if self.right_fork.is_none() {
            return match fork.hold() {
                Ok(..) => {
                    self.right_fork = Option::from(fork);
                    Ok(true)
                },
                Err(e) => {
                    Err(e)
                }
            }
        } else {
            Err(ALREADY_HAVE_RIGHT_FORK.parse().unwrap())
        }
    }

    pub fn eat(&mut self) -> Result<bool, String> {
        if self.left_fork.is_some() && self.right_fork.is_some() {
            println!("Eating");
            Ok(true)
        } else {
            println!("Can't eat");
            Err(DONT_HAVE_BOTH_FORKS.parse().unwrap())
        }
    }

    pub fn drop_left_fork(&mut self) -> Result<Fork, String> {
        if self.left_fork.is_some() {
            self.left_fork.unwrap().release();
            Ok(self.left_fork.unwrap())
        } else {
            Err("don't have a left fork".parse().unwrap())
        }
    }

    pub fn drop_right_fork(&mut self) -> Result<Fork, String> {
        if self.right_fork.is_some() {
            self.right_fork.unwrap().release();
            Ok(self.right_fork.unwrap())
        } else {
            Err("don't have a right fork".parse().unwrap())
        }
    }


}
