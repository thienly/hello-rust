use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Serialize, Deserialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Pokemon {
    pub name: String,
    pub color: PokemonColor,
    pub eye_num: i8,
    pub nose_num: i8,
    pub mouth_num: i8
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum PokemonColor {
    Blue,
    Red,
    Green
}
impl Pokemon {
    pub fn new(name: &str, color: &PokemonColor, eye_num: i8, nose_num: i8, mouth_num: i8) -> Self{
        Pokemon {
            name: name.to_string(),
            color: match color {
                PokemonColor::Blue => { PokemonColor::Blue}
                PokemonColor::Red => { PokemonColor::Red}
                PokemonColor::Green => { PokemonColor::Green}
            },
            eye_num,
            nose_num,
            mouth_num
        }
    }
}
#[derive(Clone)]
pub struct Pokemons {
    pub items: Arc<RwLock<HashMap<String, Pokemon>>>
}
impl Pokemons {
    pub fn new() -> Self{
        Pokemons {
            items: Arc::new(RwLock::new(HashMap::new()))
        }
    }

}
