use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Clone)]
pub struct Temp {
    pub y: i32,
    pub m: i32,
    pub d: i32,
    pub h: i32,
    pub averageTemp: f32,
}

impl Default for Temp {
    fn default() -> Self {
        Temp {
            y: 0,
            m: 0,
            d: 0,
            h: 0,
            averageTemp: 0.0,
        }
    }
}
