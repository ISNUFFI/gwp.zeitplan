use gloo::net::http::Request;
use crate::model::employees_result::EmployeesResult;

pub struct Repository;
impl Repository{
    pub async fn get_employees(limit: i32, offset: i32, is_hidden: bool, sort_field: &str) -> EmployeesResult{
        let api_url: String = format!("http://80.87.110.78:22814/api/employee/list?limit={limit}&offset={offset}&hidden={is_hidden}&sort_field={sort_field}");
        let fetched_employees: EmployeesResult = Request::get(&api_url)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        fetched_employees
    }
}