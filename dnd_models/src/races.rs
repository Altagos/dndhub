use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Race {
    Dragonborn,
    Dwarf,
    Elf,
    Gnome,
    HalfElf,
    Halfling,
    HalfOrc,
    Human,
    Tiefling,
    Aasimar,
    Warforged,
}

impl Default for Race {
    fn default() -> Self {
        Self::Human
    }
}
