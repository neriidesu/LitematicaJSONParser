use serde::{Deserialize, Serialize};

use crate::material_list::material::Material;

pub mod material;

#[derive(Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct MaterialList {
    pub Name: String,
    pub Multiplier: i32,
    pub Materials: Vec<Material>,
}

impl MaterialList {
    pub fn from_str(str: &str) -> MaterialList {
        let l: MaterialList = serde_json::from_str(&str).expect("err");
        l
    }
    /*
    pub fn generate_text(&self) -> String {
        let mut out: String = "".to_owned();
        for material in &self.Materials {
            let format = format!("{0}, {1}\n", material.Item, material.format_item_count());
            out.push_str(&format);
        }
        out
    }
    // */
}
