use std::cmp::min;

// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health != 0 {
            return None
        }
        Some(Player {
            health: 100,
            mana: if self.level >= 10 {Some(100)} else {None},
            level: self.level
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        let mana = match self.mana {
            Some(val) => val,
            None => {
                self.health -= min(mana_cost, self.health);
                return 0
            }
        };
        if mana < mana_cost {
            return 0
        }
        self.mana = Some(mana - mana_cost);
        mana_cost * 2
    }
}
