use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct BaseTech {
    pub(crate) produce: u8,
    pub(crate) cost: u8,
    pub(crate) name: String,
    pub(crate) id: uuid::Uuid,

}

#[derive(Debug, Clone)]
pub struct Farm {
    pub(crate) produce: u16,
    pub(crate) cost: u16,
    pub(crate) name: String,
    pub(crate) id: uuid::Uuid,

}

#[derive(Debug, Clone)]
pub struct Mine {
    pub(crate) produce: u16,
    pub(crate) cost: u16,
    pub(crate) name: String,
    pub(crate) id: uuid::Uuid,

}

#[derive(Debug, Clone)]
pub struct Tree {
    pub(crate) produce: u16,
    pub(crate) cost: u16,
    pub(crate) name: String,
    pub(crate) id: uuid::Uuid,

}


pub(crate) trait  Techs<T> {
    fn new() -> T;
}

trait Upgrade<T, U>  {
    fn upgrade() -> U;
}

impl Techs<BaseTech> for BaseTech {
    fn new() -> BaseTech {
        BaseTech {
            produce: 1,
            cost: 2,
            name: String::from("base"),
            id: Uuid::new_v4(),
        }
    }
}

impl Techs<Tree> for Tree {
    fn new() -> Tree {
        Tree {
            produce: 5,
            cost: 10,
            name: String::from("tree"),
            id: Uuid::new_v4(),
        }
    }
}

impl Techs<Farm> for Farm {
    fn new() -> Farm {
        Farm {
            produce: 50,
            cost: 100,
            name: String::from("farm"),
            id: Uuid::new_v4(),
        }
    }
}

impl Techs<Mine> for Mine {
    fn new() -> Mine {
        Mine {
            produce: 100,
            cost: 500,
            name: String::from("and they call it a mine!!"),
            id: Uuid::new_v4(),
        }
    }
}

impl Upgrade<BaseTech, Tree> for BaseTech {
    fn upgrade() -> Tree {
        Tree::new()
    } 
}

impl Upgrade<Tree, Farm> for Tree {
    fn upgrade() -> Farm {
        Farm::new()
    } 
}

impl Upgrade<Farm, Mine> for Farm {
    fn upgrade() -> Mine {
        Mine::new()
    } 
}



    