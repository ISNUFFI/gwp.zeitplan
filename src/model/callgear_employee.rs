use serde::{Deserialize, Serialize};

pub struct CallGearEmployee {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub call_gear_id: Option<i32>,
}