use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Material {
    pub Item: String,
    pub Total: i32,
    pub Missing: i32,
}

impl Material {
    pub fn format_item_count(&self) -> String {
        let amount = self.Missing;
        let stacks = &amount / 64;
        let boxes = &stacks / 27;
        let stacks = stacks % 27;
        let items = &amount % 64;
        let formatted = if boxes >= 1 && stacks >= 1 && items >= 1 {
            format!("{boxes:.0}SB + {stacks}stx + {items}")
        } else if boxes >= 1 && stacks >= 1 {
            format!("{boxes:.0}SB + {stacks}stx")
        } else if boxes >= 1 && items >= 1 {
            format!("{boxes:.0}SB + {items}")
        } else if boxes < 1 && stacks >= 1 && items >= 1 {
            format!("{stacks}stx + {items}")
        } else if boxes < 1 && stacks >= 1 && items < 1 {
            format!("{stacks}stx")
        } else {
            format!("{items}")
        };
        formatted
    }
}
