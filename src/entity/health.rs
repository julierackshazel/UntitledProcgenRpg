use std::cmp::max;

pub struct Health {
    pub amount: u16,
    pub strength: u16,
}

impl Health {
    pub fn take_damage(&mut self, damage: u16) {
        let total_damage = damage - max(damage, self.strength);
        
        match self.amount.checked_sub(total_damage) {
            Some(_) => {},
            None => { }, // KILL
        }
    }
}

