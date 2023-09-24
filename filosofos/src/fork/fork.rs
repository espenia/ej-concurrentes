use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Fork {
    pub(crate) id: Uuid,
    pub(crate) being_held: bool,
}

const ALREADY_BEING_HOLD: String = "Fork is already being held".parse().unwrap();

impl  Fork {
    pub fn new() -> Fork {
        Fork {
            id: Uuid::new_v4(),
            being_held: false,
        }
    }

    pub fn hold(&mut self) -> Result<(), String> {
        if self.being_held {
            return Err(ALREADY_BEING_HOLD.clone());
        }
        self.being_held = true;
        Ok(())
    }

    pub fn release(&mut self) {
        self.being_held = false;
    }
}


    