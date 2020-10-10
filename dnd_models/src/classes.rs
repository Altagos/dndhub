use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Class {
    Wizard,
}

impl Default for Class {
    fn default() -> Self {
        Self::Wizard
    }
}
