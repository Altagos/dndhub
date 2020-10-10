use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct Item;

#[derive(Debug, Clone, Deserialize, Serialize, Hash, PartialEq, Eq)]
pub struct ItemId(u32);
