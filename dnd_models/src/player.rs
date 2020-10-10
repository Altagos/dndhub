use crate::{
    ability_scores::AbilityScores,
    classes::Class,
    items::{Item, ItemId},
    races::Race,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Player {
    pub name: Option<String>,
    pub race: Option<Race>,
    pub class: Option<Class>,
    pub level: Option<u32>,
    pub hit_points: Option<u32>,
    pub proficiency_bonus: Option<i32>,
    pub ability_scores: Option<AbilityScores>,
    pub background: Option<String>,
    pub alignment: Option<String>,
    pub personality_traits: Option<String>,
    pub ideals: Option<String>,
    pub bonds: Option<String>,
    pub flaws: Option<String>,
    pub equipment: Option<HashMap<ItemId, Item>>,
}

impl Player {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);

        self
    }

    pub fn race(mut self, race: Race) -> Self {
        self.race = Some(race);

        self
    }

    pub fn class(mut self, class: Class) -> Self {
        self.class = Some(class);

        self
    }

    pub fn level(mut self, level: u32) -> Self {
        self.level = Some(level);

        self
    }

    pub fn hit_points(mut self, hit_points: u32) -> Self {
        self.hit_points = Some(hit_points);

        self
    }

    pub fn proficiency_bonus(mut self, proficiency_bonus: i32) -> Self {
        self.proficiency_bonus = Some(proficiency_bonus);

        self
    }

    pub fn ability_scores(mut self, ability_scores: AbilityScores) -> Self {
        self.ability_scores = Some(ability_scores);

        self
    }

    pub fn background(mut self, background: String) -> Self {
        self.background = Some(background);

        self
    }

    pub fn alignment(mut self, alignment: String) -> Self {
        self.alignment = Some(alignment);

        self
    }

    pub fn personality_traits(mut self, personality_traits: String) -> Self {
        self.personality_traits = Some(personality_traits);

        self
    }

    pub fn ideals(mut self, ideals: String) -> Self {
        self.ideals = Some(ideals);

        self
    }

    pub fn bonds(mut self, bonds: String) -> Self {
        self.bonds = Some(bonds);

        self
    }

    pub fn flaws(mut self, flaws: String) -> Self {
        self.flaws = Some(flaws);

        self
    }

    pub fn equipment(mut self, equipment: HashMap<ItemId, Item>) -> Self {
        self.equipment = Some(equipment);

        self
    }

    pub fn level_up(&mut self) {
        if let Some(mut level) = self.level {
            level += 1;
            self.level = Some(level);
        }
    }

    pub fn add_equipment(&mut self, id: ItemId, equipment: Item) {
        if self.equipment.is_some() {
            let mut items = self.equipment.clone().unwrap();
            items.insert(id, equipment);
            self.equipment = Some(items);
        }
    }

    pub fn remove_equipment(&mut self, id: ItemId) {
        if self.equipment.is_some() {
            let mut items = self.equipment.clone().unwrap();
            items.remove(&id);
            self.equipment = Some(items);
        }
    }
}
