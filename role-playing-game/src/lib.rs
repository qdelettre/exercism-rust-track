// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::cmp::max;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => {
                let mana = if self.level < 10 { None } else { Some(100) };

                Some(Player {
                    health: 100,
                    mana,
                    level: self.level,
                })
            }
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            None => {
                self.health = match (mana_cost < self.health) {
                    true => self.health -mana_cost,
                    _ => 0,
                };
                0
            }
            Some(m) if m > mana_cost => {
                self.mana = Some(m - mana_cost);
                mana_cost * 2
            }
            _ => 0,
        }
    }
}
