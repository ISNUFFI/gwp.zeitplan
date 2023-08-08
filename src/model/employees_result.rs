use serde::{Deserialize, Serialize};
use crate::model::employee::Employee;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EmployeesResult{
    pub data: Vec<Employee>,
    pub taken: i32,
    pub skipped: i32,
    pub total: i32,
}