use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct Fork {
    pub(crate) id: Uuid,
    pub(crate) being_held: bool,
}

const ALREADY_BEING_HOLD: &str = "Fork is already being held";

impl  Fork {
    pub fn new() -> Fork {
        Fork {
            id: Uuid::new_v4(),
            being_held: false,
        }
    }

    pub fn hold(&mut self) -> Result<(), String> {
        if self.being_held {
            return Err(ALREADY_BEING_HOLD.parse().unwrap());
        }
        self.being_held = true;
        Ok(())
    }

    pub fn release(&mut self) {
        self.being_held = false;
    }
}


