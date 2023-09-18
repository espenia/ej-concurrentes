use crate::tech::tech::{BaseTech, Tree, Farm, Mine, Techs};

#[derive(Debug)]
pub struct User {
    gold: u64,
    base: Vec<BaseTech>,
    tree: Vec<Tree>,
    farm: Vec<Farm>,
    mine: Vec<Mine>,
}

impl User {
    pub fn new() -> User {
        User {
            gold: 0,
            base: vec![BaseTech::new()],
            tree: vec![],
            farm: vec![],
            mine: vec![],
        }
    }

    pub fn get_gold(&self) -> u64 {
        self.gold
    }

    pub fn get_tech(&self) -> (Vec<BaseTech>, Vec<Tree>, Vec<Farm>, Vec<Mine>) {
        (self.base.clone(), self.tree.clone(), self.farm.clone(), self.mine.clone())
    }

    pub fn add_gold(&mut self, amount: u64) {
        self.gold += amount;
    }

    pub fn add_base_tech(&mut self, tech: BaseTech) {
        self.base.push(tech);
    }

    pub fn add_tree_tech(&mut self, tech: Tree) {
        self.tree.push(tech);
    }

    pub fn add_farm_tech(&mut self, tech: Farm) {
        self.farm.push(tech);
    }

    pub fn add_mine_tech(&mut self, tech: Mine) {
        self.mine.push(tech);
    }

    pub fn remove_gold(&mut self, amount: u64) {
        self.gold -= amount;
    }

    pub fn remove_base_tech(&mut self, tech: BaseTech) {
        let index = self.base.iter().position(|x| x.id == tech.id);
        if !index.is_none() {
            self.base.remove(index.unwrap());
        }
    }

    pub fn remove_tree_tech(&mut self, tech: Tree) {
        let index = self.tree.iter().position(|x| x.id == tech.id);
        if !index.is_none() {
            self.tree.remove(index.unwrap());
        }
    }

    pub fn remove_farm_tech(&mut self, tech: Farm) {
        let index = self.farm.iter().position(|x| x.id == tech.id);
        if !index.is_none() {
            self.farm.remove(index.unwrap());
        }
    }

    pub fn remove_mine_tech(&mut self, tech: Mine) {
        let index = self.mine.iter().position(|x| x.id == tech.id);
        if !index.is_none() {
            self.mine.remove(index.unwrap());
        }
    }
}
