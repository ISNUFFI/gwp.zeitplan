use serde::{Deserialize, Serialize};

pub struct BitrixUser {
    pub id: i32,
    pub bitrix24_id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
}