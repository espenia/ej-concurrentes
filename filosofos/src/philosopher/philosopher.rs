use crate::fork::fork::{Fork};

#[derive(Debug)]
pub struct Philosopher {
    left_fork: Option<Fork>,
    right_fork: Option<Fork>
}

const ALREADY_HAVE_LEFT_FORK: String = "already holding a fork in the left hand".parse().unwrap();
const ALREADY_HAVE_RIGHT_FORK: String = "already holding a fork in the right hand".parse().unwrap();
const DONT_HAVE_BOTH_FORKS: String = "don't have both forks".parse().unwrap();

impl Philosopher {
    pub fn new() -> Philosopher {
        Philosopher {
            left_fork: None,
            right_fork: None,
        }
    }

    pub fn grab_left_fork(&mut self, mut fork: Fork) -> Result<bool, String> {
        if self.left_fork.is_none() {
            return match fork.hold() {
                Ok(..) => {
                    self.left_fork = Option::from(fork);
                    Ok(true)
                },
                Err(e) => {
                    Err(e)
                }
            }
        } else {
            Err(ALREADY_HAVE_LEFT_FORK)
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
            Err(ALREADY_HAVE_RIGHT_FORK)
        }
    }

    pub fn eat(&mut self) -> Result<bool, String> {
        if self.left_fork.is_some() && self.right_fork.is_some() {
            println!("Eating");
            Ok(true)
        } else {
            println!("Can't eat");
            Err(DONT_HAVE_BOTH_FORKS)
        }
    }

    pub fn drop_left_fork(&mut self) -> Result<Fork, String> {
        if self.left_fork.is_some() {
            self.left_fork.release();
            self.left_fork = None;
            Ok(self.left_fork.unwrap())
        } else {
            Err("don't have a left fork".parse().unwrap())
        }
    }

    pub fn drop_right_fork(&mut self) -> Result<Fork, String> {
        if self.right_fork.is_some() {
            self.right_fork = None;
            self.right_fork.release();
            Ok(self.right_fork.unwrap())
        } else {
            Err("don't have a right fork".parse().unwrap())
        }
    }


}
