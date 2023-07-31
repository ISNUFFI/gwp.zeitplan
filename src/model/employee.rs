use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Employee {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    //pub email: String,
    //pub bitrix_user_id: Option<i32>,
    pub call_gear_employee_id: Option<i32>,
    pub is_hidden: bool,
}

