use gloo::net::http::Request;
use crate::model::employees_result::EmployeesResult;

pub struct Repository;
impl Repository{
    pub async fn get_employees(limit: i32, offset: i32, is_hidden: bool) -> EmployeesResult{
        let api_url: String = format!("http://80.87.110.78:22814/api/employee/list?limit={}&offset={}&hidden={}", limit, offset, is_hidden);
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