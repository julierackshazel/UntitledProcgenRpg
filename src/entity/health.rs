pub struct Health {
    pub amount: u16,
    pub strength: u16,
}

impl Health {
    pub fn take_damage(&mut self, damage: u16) {
        self.amount.checked_sub(damage.checked_sub(self.strength));
    }
}

